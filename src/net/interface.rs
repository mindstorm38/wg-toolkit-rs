//! W.I.P.

use std::net::{ToSocketAddrs, UdpSocket, SocketAddr};
use std::collections::HashMap;
use std::marker::PhantomData;

use super::bundle::{BundleAssembler, BundleRawElementsIter};
// use super::element::ElementCodec;
use super::packet::Packet;


/*/// A all-in-one interface for the WG network protocol, using bundles and
/// allowing callbacks of specific elements.
pub struct Interface {
    sock: UdpSocket,
    /// The structure used to re-assemble bundles from received raw packets.
    bundle_assembler: BundleAssembler<SocketAddr>,
    /// If all bundles and packets should have the 4-bytes prefix.
    has_prefix: bool,
    elements: HashMap<u8, Box<dyn InterfaceElement>>,
    /// The next sequence ID to use for bundles.
    next_seq_id: u32
}

impl Interface {

    pub fn new<A: ToSocketAddrs>(addr: A, has_prefix: bool) -> std::io::Result<Self> {
        Ok(Self {
            sock: UdpSocket::bind(addr)?,
            bundle_assembler: BundleAssembler::new(has_prefix),
            has_prefix,
            elements: HashMap::new(),
            next_seq_id: 0 // Need a random starting sequence ID
        })
    }

    pub fn receive(&mut self) {

        let mut packet = Packet::new_boxed(self.has_prefix);
        let (len, addr) = self.sock.recv_from(&mut packet.data).unwrap();

        if let Err(e) = packet.sync_state(len, true) {
            eprintln!("Failed to sync state from data: {:?}", e)
        } else if let Some(bundle) = self.bundle_assembler.try_assemble(addr, packet) {
            let mut iter = bundle.iter_raw_elements();
            while let Some(id) = iter.next_id() {
                if let Some(elt) = self.elements.get_mut(&id) {
                    if !elt.next_element_and_dispatch(&mut iter) {
                        eprintln!("Failed to dispatch message.");
                    }
                }
            }
        }

    }

    /*pub fn send<A: ToSocketAddrs>(&mut self, mut bundle: Bundle, to: A) {
        bundle.finalize(&mut self.next_seq_id);
        for packet in bundle.get_packets() {
            self.sock.send_to(packet.get_valid_data(), to);
        }
    }*/

    /*pub fn register_element<E, C>(&mut self, id: u8, callback: Option<C>)
    where
        E: ElementCodec + 'static,
        C: FnMut(&E) + 'static
    {
        self.elements.insert(id, Box::new(ConcreteInterfaceElement {
            callback,
            phantom: PhantomData
        }));
    }*/

}*/


/*/// Internal abstract element definition.
trait InterfaceElement {
    fn next_element_and_dispatch(&mut self, iter: &mut BundleRawElementsIter) -> bool;
}

struct ConcreteInterfaceElement<E, C> {
    callback: Option<C>,
    phantom: PhantomData<*const E>
}

impl<E, C> InterfaceElement for ConcreteInterfaceElement<E, C>
where
    E: ElementCodec,
    C: FnMut(&E)
{

    fn next_element_and_dispatch(&mut self, iter: &mut BundleRawElementsIter) -> bool {
        todo!()
        /*match iter.next::<E>() {
            Some(elt) => {
                if let Some(callback) = &mut self.callback {
                    callback(&elt.elt);
                }
                true
            }
            _ => false
        }*/
    }

}*/
