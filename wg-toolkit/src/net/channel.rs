//! Channel tracking.

use std::collections::{hash_map, HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::net::SocketAddr;
use std::num::NonZero;

use tracing::{debug, instrument, trace};

use super::packet::{Packet, PacketConfig, PacketConfigError};
use super::bundle::Bundle;


/// The (currently hardcoded) timeout on bundle fragments.
const FRAGMENT_TIMEOUT: Duration = Duration::from_secs(10);


/// This tracker helps tracking off-channel and in-channel communications and provides an
/// interface to both prepare bundles and accept incoming packets to form (potentially
/// fragmented) bundles.
#[derive(Debug)]
pub struct ChannelTracker {
    /// State shared with channel handles.
    shared: ChannelTrackerShared,
    /// For each address we are exchanging with, we store a special off-channel channel.
    off_channels: HashMap<SocketAddr, OffChannel>,
    /// Known channels for each address, with optional channel indexing.
    channels: HashMap<(SocketAddr, Option<NonZero<u32>>), OnChannel>,
    /// List of rejected packets.
    rejected_packets: Vec<(SocketAddr, Box<Packet>, PacketRejectionError)>,
}

/// A structure referenced by any channel handle, containing shared states.
#[derive(Debug)]
struct ChannelTrackerShared {
    /// Sequence number allocator used for off-channel communications.
    off_sequence_num_alloc: SequenceNumAllocator,
    /// Represent the last prefix being write to a packet.
    last_accepted_prefix: u32,
    /// The current prefix offset being used for updating all packets' prefixes.
    prefix_offset: u32,
}

impl ChannelTracker {

    pub fn new() -> Self {
        Self {
            shared: ChannelTrackerShared {
                off_sequence_num_alloc: SequenceNumAllocator::new(1),
                last_accepted_prefix: 0,
                prefix_offset: 0,
            },
            off_channels: HashMap::new(),
            channels: HashMap::new(),
            rejected_packets: Vec::new(),
        }
    }

    /// Return the off-channel handle, the off-channel isn't a real channel and bundles
    /// prepared with it will be sent out of channel, but this is used as a shared 
    /// interface with in-channel.
    pub fn off_channel(&mut self, addr: SocketAddr) -> Channel<'_> {

        let channel = self.off_channels.entry(addr)
            .or_insert_with(|| OffChannel {
                off: OffChannelData::new(),
            });

        Channel {
            inner: GenericChannel {
                shared: &mut self.shared,
                off: &mut channel.off,
                on: None,
            }
        }

    }

    /// Return a handle to a channel associated with the given address, optionally 
    /// indexed if desired, the channel is created if not already existing with initial
    /// version of 1.
    pub fn channel(&mut self, addr: SocketAddr, index: Option<NonZero<u32>>) -> Channel<'_> {

        let channel = self.channels.entry((addr, index))
            .or_insert_with(|| OnChannel {
                off: OffChannelData::new(),
                on: match index {
                    None => OnChannelData::new_without_index(),
                    Some(index) => OnChannelData::new_with_index(index),
                },
            });
        
        Channel {
            inner: GenericChannel {
                shared: &mut self.shared,
                off: &mut channel.off,
                on: None,
            }
        }

    }

    /// Reset the prefix offset to zero.
    #[inline]
    pub fn reset_prefix_offset(&mut self) {
        self.shared.prefix_offset = 0;
    }

    /// Return the last accepted prefix from any packet.
    #[inline]
    pub fn last_accepted_prefix(&self) -> u32 {
        self.shared.last_accepted_prefix
    }

    /// Set the current prefix offset used for computing the prefix of prepared packets
    /// from the value of the last accepted packet.
    #[inline]
    pub fn transfer_prefix_offset_from_last_received(&mut self) {
        self.shared.prefix_offset = self.shared.last_accepted_prefix;
        // self.shared.prefix_offset = 0x7A11751F;
    }

    /// Accept a new incoming packet and optionally return a bundle if it just completed
    /// a new bundle.
    /// 
    /// If the packet is rejected for any reason listed in [`PacketRejectionError`], none
    /// is also returned but the packet is internally queued and can later be retrieved 
    /// with the error using [`Self::take_rejected_packets()`].
    #[instrument(level = "trace", skip(self, packet))]
    pub fn accept(&mut self, mut packet: Box<Packet>, addr: SocketAddr) -> Option<(Bundle, Channel<'_>)> {

        // Retrieve the real clear-text length after a potential decryption.
        let len = packet.raw().data_len();

        let mut packet_config = PacketConfig::new();
        if let Err(error) = packet.read_config(len, &mut packet_config) {
            self.rejected_packets.push((addr, packet, PacketRejectionError::Config(error)));
            return None;
        }

        self.shared.last_accepted_prefix = packet.raw().read_prefix();

        // Start by finding the appropriate channel for this packet regarding the local
        // socket address and channel-related flags on this packet.
        let mut channel;
        if packet_config.on_channel() {

            let on_channel;
            if let Some((index, version)) = packet_config.indexed_channel() {
                
                trace!("Is on-channel: {index} v{version}");
                on_channel = self.channels.entry((addr, Some(index)))
                    .or_insert_with(|| OnChannel {
                        off: OffChannelData::new(),
                        on: OnChannelData::new_with_index_version(index, version),
                    });

                // Unwrap because the channel should have index.
                let current_version = on_channel.on.index.unwrap().version;
                if version < current_version {
                    trace!("Outdated, expected v{current_version}");
                    // TODO: outdated packet
                    return None;
                }

            } else {
                trace!("Is on-channel: not indexed");
                on_channel = self.channels.entry((addr, None))
                    .or_insert_with(|| OnChannel {
                        off: OffChannelData::new(),
                        on: OnChannelData::new_without_index(),
                    });
            }

            // First packet after channel creation should contains this flag.
            // TODO: Remove this? Because there is no create flag on external channels.
            if !on_channel.on.received_create_packet {
                if packet_config.create_channel() {
                    trace!("Has-confirmed creation of the channel");
                    on_channel.on.received_create_packet = true;
                } else {
                    // // TODO: expected create channel packet
                    // debug!("Should be channel create");
                    // return None;
                }
            }

            channel = GenericChannel {
                shared: &mut self.shared,
                off: &mut on_channel.off,
                on: Some(&mut on_channel.on),
            };

        } else {

            trace!("Is off-channel");
            let off_channel = self.off_channels.entry(addr)
                .or_insert_with(|| OffChannel { off: OffChannelData::new() });

            channel = GenericChannel {
                shared: &mut self.shared,
                off: &mut off_channel.off,
                on: None,
            };

        }

        // Cumulative ack is not supposed to be used off-channel.
        if let Some(cumulative_ack) = packet_config.cumulative_ack() {
            if channel.on.is_some() {
                channel.acknowledge_reliable_packet_cumulative(cumulative_ack);
            } else {
                // Cumulative ack is not supported off-channel.
                debug!("Cumulative ack is not supported off-channel");
                return None;
            }
        }

        // If we have some acks.
        for &ack in packet_config.single_acks() {
            channel.acknowledge_reliable_packet(ack);
        }

        // If the received packet is reliable, we'll need to send a ack for it in future.
        if packet_config.reliable() {
            channel.add_received_reliable_packet(packet_config.sequence_num());
        }

        // We can observe that packets with the flag 0x1000 are only used
        // for auto acking with sometimes duplicated data that is sent
        // just after. If it become a problem this check can be removed. 
        if packet_config.unk_1000().is_some() {
            return None;
        }
        
        let instant = Instant::now();

        match packet_config.sequence_range() {
            // Only if there is a range and this range is not a single num.
            Some((first_num, last_num)) => {

                let num = packet_config.sequence_num();

                match channel.off.fragments.entry(first_num) {
                    hash_map::Entry::Occupied(mut o) => {

                        // If this fragments is too old, timeout every packet in it
                        // and start again with the packet.
                        // FIXME: Maybe dumb?
                        if o.get().is_old(instant, FRAGMENT_TIMEOUT) {
                            self.rejected_packets.extend(o.get_mut().drain()
                                .map(|packet| (addr, packet, PacketRejectionError::TimedOut)));
                        }

                        o.get_mut().set(num, packet);

                        // When all fragments are collected, remove entry and return.
                        if o.get().is_full() {
                            return Some((o.remove().into_bundle(), Channel { inner: channel }));
                        }

                    },
                    hash_map::Entry::Vacant(v) => {
                        let mut fragments = Fragments::new(last_num - first_num + 1);
                        fragments.set(num, packet);
                        v.insert(fragments);
                    }
                }

            }
            // Not sequence range in the packet, create a bundle only with it.
            _ => {
                return Some((Bundle::with_single(packet), Channel { inner: channel }));
            }
        }

        None

    }

    /// Accept an outgoing packet, this should never be used in practice because the
    /// [`Channel::prepare`] method used to prepare complete bundles is already handling
    /// the reliable tracking. However, this function is used for proxies where we never
    /// manually prepare bundles but instead just forward packets, in such case we should
    /// be able to simulate preparation of outgoing packets.
    #[instrument(level = "trace", skip(self, packet))]
    pub fn accept_out(&mut self, mut packet: Box<Packet>, addr: SocketAddr) -> bool {

        // Retrieve the real clear-text length after a potential decryption.
        let len = packet.raw().data_len();

        let mut packet_config = PacketConfig::new();
        if let Err(_error) = packet.read_config(len, &mut packet_config) {
            return false;
        }

        let channel;
        if packet_config.on_channel() {

            let on_channel;
            if let Some((index, version)) = packet_config.indexed_channel() {
                trace!("Is on-channel: {index} v{version}");
                on_channel = self.channels.entry((addr, Some(index)))
                    .or_insert_with(|| OnChannel {
                        off: OffChannelData::new(),
                        on: OnChannelData::new_with_index_version(index, version),
                    });
            } else {
                trace!("Is on-channel: not indexed");
                on_channel = self.channels.entry((addr, None))
                    .or_insert_with(|| OnChannel {
                        off: OffChannelData::new(),
                        on: OnChannelData::new_without_index(),
                    });
            }

            channel = GenericChannel {
                shared: &mut self.shared,
                off: &mut on_channel.off,
                on: Some(&mut on_channel.on),
            };

        } else {

            trace!("Is off-channel");
            let off_channel = self.off_channels.entry(addr)
                .or_insert_with(|| OffChannel { off: OffChannelData::new() });

            channel = GenericChannel {
                shared: &mut self.shared,
                off: &mut off_channel.off,
                on: None,
            };

        }

        if let Some(_cumulative_ack) = packet_config.cumulative_ack() {
            if channel.on.is_some() {
                // TODO:
            } else {
                // Cumulative ack is not supported off-channel.
                debug!("Cumulative ack is not supported off-channel");
                return false;
            }
        }

        true

    }

}

/// A handle to a channel or to the special off-channel fake channel.
/// 
/// Represent a channel between the app and a client with specific socket address. A 
/// channel is a way to create multiple communication channels between the same pair
/// of addresses and also provides support for reliable communication with sequence
/// number acks and resend logic (not yet implemented).
#[derive(Debug)]
pub struct Channel<'a> {
    inner: GenericChannel<'a>,
}

impl Channel<'_> {

    pub fn is_on(&self) -> bool {
        self.inner.on.is_some()
    }

    pub fn is_off(&self) -> bool {
        !self.is_on()
    }

    pub fn index(&self) -> Option<ChannelIndex> {
        self.inner.on.as_deref().and_then(|on| on.index)
    }

    /// Prepare a bundle to be sent, adding acks and other configuration required by this
    /// tracker into all packets. After this function, all packets are ready to be sent.
    #[instrument(level = "trace", skip(self, bundle))]
    pub fn prepare(&mut self, bundle: &mut Bundle, reliable: bool) {

        let bundle_len = bundle.len() as u32;

        trace!("Count: {bundle_len}");
        
        // Create a common packet config for all the bundle.
        let mut packet_config = PacketConfig::new();

        packet_config.set_reliable(reliable);

        if reliable || bundle_len > 1 {
            packet_config.set_sequence_num(self.inner.alloc_sequence_num(bundle_len));
        }
        
        if bundle_len > 1 {
            let first_num = packet_config.sequence_num();
            packet_config.set_sequence_range(first_num, first_num + bundle_len - 1);
        }
        
        if let Some(on_channel) = self.inner.on.as_deref_mut() {

            packet_config.set_on_channel(true);

            if let Some(index) = on_channel.index {
                packet_config.set_indexed_channel(index.index, index.version);
                trace!("Is on-channel: {} v{}", index.index, index.version);
            } else {
                trace!("Is on-channel: not indexed");
            }

        } else {
            trace!("Is off-channel");
        }

        // This may remove some acks from `off.received_reliable_packets` so it's 
        // important to do it before the swap just after. Only works on-channel.
        if let Some(cumulative_ack) = self.inner.pop_received_reliable_packet_cumulative() {
            packet_config.set_cumulative_ack(cumulative_ack);
        }

        trace!("Pending single acks: {:?}", self.inner.off.received_reliable_packets);
        trace!("Using prefix offset: 0x{:08X}", self.inner.shared.prefix_offset);
        
        // This swap is simple: it places the dequeue of all received reliable packets 
        // and their sequence numbers into the packet config's acks queue. We must 
        // remember after this to transfer back the remaining sequence numbers that
        // have not been sent from the packet config.
        std::mem::swap(&mut self.inner.off.received_reliable_packets, packet_config.single_acks_mut());
        debug_assert!(self.inner.off.received_reliable_packets.is_empty(), "packet config acks were not empty");

        // Now we set the sequence number for 
        for (packet_index, packet) in bundle.packets_mut().iter_mut().enumerate() {

            // Write configuration to the packet and then increment the sequence num.
            // The sequence num should only be set if channel with reliable or if we
            // have multiple packets: this number is unused in other cases.
            packet.write_config(&mut packet_config);
            trace!("Packet #{packet_index} length: {}", packet.raw().data_len());

            // Compute the prefix.
            packet.raw_mut().update_prefix(self.inner.shared.prefix_offset);

            if reliable {
                self.inner.add_reliable_packet(packet_config.sequence_num());
            }

            packet_config.set_sequence_num(packet_config.sequence_num() + 1);

            // Only send the cumulative ack on first packet, so remove it on next.
            packet_config.clear_cumulative_ack();

        }

        // Now we need to restore acks that have not been sent: swap back (read above).
        std::mem::swap(&mut self.inner.off.received_reliable_packets, packet_config.single_acks_mut());
        debug_assert!(packet_config.single_acks().is_empty(), "packet config acks should be empty");

        if !self.inner.off.received_reliable_packets.is_empty() {
            trace!("Remaining single acks: {:?}", self.inner.off.received_reliable_packets)
        }

    }

}

/// Documented from BigWorld source code (programming\bigworld\lib\network\udp_channel.hpp).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelIndex {
    /// An indexed channel is basically a way of multiplexing multiple
	/// channels between a pair of addresses.  Regular channels distinguish
	/// traffic solely on the basis of address, so in situations where you need
	/// multiple channels between a pair of addresses (i.e. channels between
	/// base and cell entities) you use indexed channels to keep the streams
	/// separate.
    pub index: NonZero<u32>,
	/// Indexed channels have a 'version' number which basically tracks how
	/// many times they have been offloaded.  This allows us to correctly
	/// determine which incoming packets are out-of-date and also helps
	/// identify the most up-to-date information about lost entities in a
	/// restore situation.
    pub version: NonZero<u32>,
}

///  Kind of error that caused a packet to be rejected from this socket and not received.
#[derive(Debug, Clone, thiserror::Error)]
pub enum PacketRejectionError {
    /// The packet is part of a sequence but no other packets of the sequence have been
    /// found and therefore no bundle can be reconstructed.
    #[error("timed out")]
    TimedOut,
    /// The packet could not be synchronized from its data.
    #[error("config error: {0}")]
    Config(#[from] PacketConfigError),
}

/// Common data for off-channel and on-channel communication.
#[derive(Debug)]
struct OffChannelData {
    /// All sequences marked as reliable, so that we can ensure that they are received.
    /// It should be naturally sorted (debug_asserted).
    reliable_packets: Vec<ReliablePacket>,
    /// A dequeue containing all received reliable packets for which we should send ack.
    /// It doesn't need to be sorted.
    received_reliable_packets: VecDeque<u32>,
    /// Bundle fragments tracking, mapped to the first sequence.
    fragments: HashMap<u32, Fragments>,
}

/// Data specific to on-channel communication.
#[derive(Debug)]
struct OnChannelData {
    /// Optional index for this channel.
    index: Option<ChannelIndex>,
    /// The next sequence number to return.
    sequence_num_alloc: SequenceNumAllocator,
    /// Set to true after the first packet with the create_channel flag has been received.
    received_create_packet: bool,
    /// Most of the time, reliable sequence numbers are received in order, and so we can
    /// just increment this counter in order to know that all packets up to (but 
    /// excluding) this number has been received.
    /// 
    /// NOTE: This is the same as `inSeqAt_`, `bufferedReceives_` in BigWorld source.
    received_reliable_packets_cumulative: u32,
    /// Last value of `received_reliable_packets_cumulative` popped, used to avoid 
    /// multiple identical cumulative ack.
    /// 
    /// NOTE: The BigWorld implementation seems to add cumulative ack even if there was
    /// no progress, see `UDPChannel::writeFlags` in BigWorld source.
    received_reliable_packets_cumulative_pop: u32,
    /// If a packet is received out-of-order and cannot increment the cumulative sequence
    /// number (in `received_reliable_packets_cumulative`), then we should buffer it in
    /// this **ordered** dequeue, if the gap is filled then we'll be able to increment 
    /// the cumulative sequence number properly.
    received_reliable_packets_buffered: VecDeque<u32>,
}

/// A reliable packet that we sent at given time and waiting for an acknowledgment.
#[derive(Debug)]
struct ReliablePacket {
    /// The sequence number.
    sequence_num: u32,
    /// The time this sequence has been sent.
    time: Instant,
}

#[derive(Debug)]
#[repr(C)]
struct OffChannel {
    off: OffChannelData,
}

#[derive(Debug)]
#[repr(C)]
struct OnChannel {
    off: OffChannelData,
    on: OnChannelData,
}

impl OffChannelData {
    fn new() -> Self {
        Self {
            reliable_packets: Vec::new(),
            received_reliable_packets: VecDeque::new(),
            fragments: HashMap::new(),
        }
    }
}

impl OnChannelData {

    fn new(index: Option<ChannelIndex>) -> Self {
        Self {
            index,
            sequence_num_alloc: SequenceNumAllocator::new(0),
            received_create_packet: false,
            received_reliable_packets_cumulative: 0,
            received_reliable_packets_cumulative_pop: u32::MAX, // to force pop first call
            received_reliable_packets_buffered: VecDeque::new(),
        }
    }

    fn new_without_index() -> Self {
        Self::new(None)
    }

    fn new_with_index_version(index: NonZero<u32>, version: NonZero<u32>) -> Self {
        Self::new(Some(ChannelIndex { index, version }))
    }

    fn new_with_index(index: NonZero<u32>) -> Self {
        Self::new_with_index_version(index, NonZero::new(1).unwrap())
    }

}

/// Internal structure used to reference a channel like a handle to it, providing an
/// internal common interface between both.
#[derive(Debug)]
struct GenericChannel<'a> {
    shared: &'a mut ChannelTrackerShared,
    off: &'a mut OffChannelData,
    on: Option<&'a mut OnChannelData>,
}

impl GenericChannel<'_> {

    fn alloc_sequence_num(&mut self, count: u32) -> u32 {
        if let Some(on) = self.on.as_deref_mut() {
            on.sequence_num_alloc.alloc(count)
        } else {
            self.shared.off_sequence_num_alloc.alloc(count)
        }
    }

    /// TODO: We'll also need to automatically resend the packet's content after some 
    /// time.
    fn add_reliable_packet(&mut self, sequence_num: u32) {
    
        // We are keeping reliable packets ordered by their sequence number and also by
        // their time (Instant::now() can only grow).
        debug_assert!(
            self.off.reliable_packets.is_empty() || 
            self.off.reliable_packets.last().unwrap().sequence_num < sequence_num,
            "reliable packet sequence number should be greater than previous ones");
        
        trace!("Add reliable packet: {sequence_num}");
        self.off.reliable_packets.push(ReliablePacket {
            sequence_num,
            time: Instant::now(),
        });

    }

    /// When a single ack is received on a packet, this can be called to 
    fn acknowledge_reliable_packet(&mut self, sequence_num: u32) -> bool {

        let index = match self.off.reliable_packets.binary_search_by_key(&sequence_num, |p: _| p.sequence_num) {
            Ok(index) => index,
            Err(_) => return false,
        };

        let reliable_packet = self.off.reliable_packets.remove(index);
        trace!("Single ack for reliable packet: {sequence_num} after {:?}", reliable_packet.time.elapsed());
        true

    }

    /// When a cumulative ack is received, this can be used to acknowledge all sequences
    /// up to, but excluding, the given sequence number. Not supported off-channel but
    /// still present here.
    fn acknowledge_reliable_packet_cumulative(&mut self, sequence_num: u32) {
       
        let index = match self.off.reliable_packets.binary_search_by_key(&sequence_num, |p: _| p.sequence_num) {
            Ok(index) => index,
            Err(index) => index,
        };

        trace!("Cumulative ack for reliable packets: ..{sequence_num}");
        for reliable_packet in self.off.reliable_packets.drain(..index) {
            trace!("Cumulative ack for a previous packet after {:?}", reliable_packet.time.elapsed());
        }

    }

    /// When accepting a packet marked as reliable, use this to register it for sending
    /// an ack later in time. This works both for off-channel and on-channel 
    /// communication, however when on-channel there is an additional logic to 
    /// compute the possible cumulative ack.
    /// 
    /// This function basically insert the sequence number into the internal dequeue
    /// `off.received_reliable_packets` which can be retrieved later.
    fn add_received_reliable_packet(&mut self, sequence_num: u32) {

        // We hope that we don't received it twice...
        self.off.received_reliable_packets.push_back(sequence_num);
        trace!("Received reliable packet: {sequence_num}");

        if let Some(on) = self.on.as_deref_mut() {
            
            if sequence_num == on.received_reliable_packets_cumulative {
                // This is the best scenario, packet is received in-order.
                on.received_reliable_packets_cumulative += 1;
                // Continue increment the cumulative number if buffered numbers follows.
                while on.received_reliable_packets_buffered.front().copied()
                == Some(on.received_reliable_packets_cumulative) {
                    on.received_reliable_packets_buffered.pop_front();
                    on.received_reliable_packets_cumulative += 1;
                }
            } else if sequence_num < on.received_reliable_packets_cumulative {
                // Do nothing, the sequence number may have been already received...
            } else {
                // We need to buffer the sequence number because it is not immediately 
                // following the previous one, it will be recovered in the future when
                // the gap will be filled.
                match on.received_reliable_packets_buffered.binary_search(&sequence_num) {
                    Ok(index) => on.received_reliable_packets_buffered.insert(index, sequence_num),
                    Err(_) => return  // Ignore already existing packets.
                }
            }

            trace!("Received reliable packet cumulative: {}, buffered: {:?}", 
                on.received_reliable_packets_cumulative, 
                on.received_reliable_packets_buffered);

        }

    }

    /// If we are on-channel then it returns the cumulative sequence number to ack, if so
    /// it will also remove the corresponding acks from the internal dequeue
    /// `off.received_reliable_packets` so that we don't ack twice.
    /// 
    /// This returns none if not on-channel.
    fn pop_received_reliable_packet_cumulative(&mut self) -> Option<u32> {

        let on = self.on.as_deref_mut()?;
        let ret = on.received_reliable_packets_cumulative;

        if on.received_reliable_packets_cumulative_pop == ret {
            trace!("Pop received reliable packet cumulative: none");
            return None;
        }

        // Remove all single acks that are before the cumulative ack: 
        // retain all sequence numbers after or equal
        self.off.received_reliable_packets.retain(|&num| num >= ret);

        on.received_reliable_packets_cumulative_pop = ret;
        trace!("Pop received reliable packet cumulative: {ret}");
        Some(ret)

    }

    // fn acknowledge_received_reliable_packet_cumulative(&mut self, sequence_num: u32) {

    // }

}

/// An allocator for contiguous sequence numbers.
#[derive(Debug)]
struct SequenceNumAllocator {
    next_num: u32,
}

impl SequenceNumAllocator {

    fn new(next: u32) -> Self {
        Self {
            next_num: next,
        }
    }

    fn alloc(&mut self, count: u32) -> u32 {
        assert!(self.next_num + count < 0x10000000, "sequence number overflow");
        let first_num = self.next_num;
        self.next_num += count;
        trace!("Allocated sequence numbers: {}..{}", first_num, self.next_num);
        first_num
    }

}

/// Internal structure to keep fragments from a given sequence.
#[derive(Debug)]
struct Fragments {
    fragments: Vec<Option<Box<Packet>>>,  // Using boxes to avoid moving huge structures.
    seq_count: u32,
    last_update: Instant,
}

impl Fragments {

    /// Create from sequence length.
    fn new(seq_len: u32) -> Self {
        Self {
            fragments: (0..seq_len).map(|_| None).collect(),
            seq_count: 0,
            last_update: Instant::now()
        }
    }

    /// This this fragments packets and reset internal count to zero.
    fn drain(&mut self) -> impl Iterator<Item = Box<Packet>> {
        self.seq_count = 0;
        std::mem::take(&mut self.fragments)
            .into_iter()
            .filter_map(|slot| slot)
    }

    /// Set a fragment.
    fn set(&mut self, num: u32, packet: Box<Packet>) {
        let frag = &mut self.fragments[num as usize];
        if frag.is_none() {
            self.seq_count += 1;
        }
        self.last_update = Instant::now();
        *frag = Some(packet);
    }

    #[inline]
    fn is_old(&self, instant: Instant, timeout: Duration) -> bool {
        instant - self.last_update > timeout
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.seq_count as usize == self.fragments.len()
    }

    /// Convert this structure to a bundle, **safe to call only if `is_full() == true`**.
    #[inline]
    fn into_bundle(self) -> Bundle {
        assert!(self.is_full());
        let packets = self.fragments.into_iter()
            .map(|o| o.unwrap())
            .collect();
        Bundle::with_multiple(packets)
    }

}
