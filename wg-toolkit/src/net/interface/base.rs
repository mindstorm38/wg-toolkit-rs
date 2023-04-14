//! Base app interface.

use std::net::{SocketAddrV4, SocketAddr};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::time::Instant;
use std::sync::Arc;
use std::io;

use blowfish::Blowfish;
use rand::rngs::OsRng;
use rand::RngCore;

use crate::net::bundle::{BundleElement, BundleElementWriter};
use crate::net::element::Element;

use crate::net::element::base::{
    id as base_id,
    ClientAuth,
    ServerSessionKey, ClientSessionKey,
};

use crate::net::element::client::{
    id as client_id,
    UpdateFrequencyNotification, 
    TickSync,
    CreateBasePlayer, 
    SelectPlayerEntity, ResetEntities,
};

use super::{Interface, Shared, Peer};


/// Interface implementation for base app.
pub struct BaseAppInterface<S: BaseAppShared> {
    pub inner: Interface<BaseApp<S>>,
}

impl<S: BaseAppShared> BaseAppInterface<S> {

    pub fn new(addr: SocketAddrV4, shared: S) -> io::Result<Self> {

        let mut inner = Interface::new(addr, BaseApp {
            shared,
            timer: Timer {
                start_time: Instant::now(),
                update_freq: 10, // Default to 10 Hz
            },
            pending_clients: HashMap::new(),
            clients: HashMap::new(),
            clients_counter: 0,
        })?;

        inner.register_simple(base_id::CLIENT_AUTH, BaseApp::on_client_auth);
        inner.register_simple(base_id::CLIENT_SESSION_KEY, BaseApp::on_client_session_key);

        Ok(Self { inner })

    }

}

/// Shared data for the base app. The underlying shared data provides
/// types and function for manipulating accounts and the underlying
/// game's logic.
pub struct BaseApp<S: BaseAppShared> {
    /// Super shared data.
    #[allow(unused)] // TODO: remove
    shared: S,
    /// The base app timer.
    timer: Timer,
    /// List of clients pending for switching from login app to base app.
    pending_clients: HashMap<u32, PendingClient<S::Account>>,
    /// List of clients logged in the base app mapped to their socket address.
    clients: HashMap<SocketAddr, Client<S::Account>>,
    /// Used to track the number of logged clients, in order to allocated
    /// session ids.
    clients_counter: u32,
}

impl<S: BaseAppShared> BaseApp<S> {

    /// Allocate a new pending client for the given socket address and 
    /// blowfish key.
    pub fn alloc_pending_client(&mut self, 
        account: Box<S::Account>, 
        addr: SocketAddr, 
        bf: &Arc<Blowfish>
    ) -> u32 {
        loop {
            let key = OsRng.next_u32();
            match self.pending_clients.entry(key) {
                Entry::Vacant(v) => {
                    v.insert(PendingClient::new(account, addr, bf.clone()));
                    break key
                }
                _ => continue
            }
        }
    }

    /// Handler for client authentication elements. This is sent by the
    /// client when first connecting to the base app in order to validate
    /// that is has been previously authorized by a login app with the
    /// given `login_key`.
    /// 
    /// This element is exceptionally not blowfish encrypted, but this
    /// handler should enable the encryption if the client is authorized.
    /// 
    /// Once channel encryption is enabled, this handler replies with 
    /// the server session key that will be sent for initialization.
    pub fn on_client_auth(&mut self, element: BundleElement<ClientAuth>, mut peer: Peer<Self>) {

        let login_key = element.element.login_key;
        let request_id = element.request_id.unwrap();
        let peer_addr = peer.addr();

        if let Some(pending_login) = self.pending_clients.remove(&login_key) {
            if pending_login.addr == peer_addr {

                peer.set_channel(pending_login.blowfish);

                self.clients_counter = self.clients_counter.checked_add(1).expect("too much logged clients");
                let logged_key = self.clients_counter;

                self.clients.insert(peer_addr, Client::new(pending_login.account, logged_key));

                // Sent the server session key.
                peer.element_writer().write_simple_reply(ServerSessionKey {
                    session_key: logged_key,
                }, request_id);

            }
        }

    }

    /// Handler for client session key elements.
    pub fn on_client_session_key(&mut self, element: BundleElement<ClientSessionKey>, mut peer: Peer<Self>) {

        // Make sure we have a client associated.
        let Some(logged_client) = self.clients.get_mut(&peer.addr()) else {
            todo!()
        };

        if element.element.session_key != logged_client.session_key {
            todo!("incoherent session key")
        }

        match logged_client.state {
            ClientState::Initial => {

                peer.element_writer().write_simple(client_id::UPDATE_FREQUENCY_NOTIFICATION, UpdateFrequencyNotification {
                    frequency: self.timer.update_freq,
                    game_time: self.timer.current_time(),
                });

                self.timer.timestamp_element(peer.element_writer());
                peer.flush();

                let (
                    entity_data, 
                    entity_type
                ) = self.shared.new_login_entity(&logged_client.account);

                peer.element_writer().write_simple(client_id::CREATE_BASE_PLAYER, CreateBasePlayer {
                    entity_id: 37289213,
                    entity_type,
                    unk: String::new(),
                    entity_data,
                    entity_components_count: 0,
                });
                self.timer.timestamp_element(peer.element_writer());
                peer.flush();

                peer.element_writer().write_simple(client_id::SELECT_PLAYER_ENTITY, SelectPlayerEntity);


                peer.element_writer().write_simple(client_id::ENTITY_METHOD.  EntityMethod::index_to_id(2), UnknownElement(vec![
                    21, 7, 100, 101, 102, 97, 117, 108, 116, 12, 128, 2, 93, 113, 1, 40, 75, 201, 75, 202, 101, 46
                ]));
                self.timer.timestamp_element(peer.element_writer());
                peer.flush();

                peer.element_writer().write_simple(client_id::RESET_ENTITIES, ResetEntities { 
                    keep_player_on_base: false
                });
                self.timer.timestamp_element(peer.element_writer());
                peer.flush();

            }
            _ => {}
        }

    }

}

impl<S: BaseAppShared> Shared for BaseApp<S> { }


/// Login-app-specific shared interface, the implementor must also
/// implement [`InterfaceShared`] as it will used as a callback for
/// global events.
pub trait BaseAppShared: Shared {

    /// Shared type for accounts.
    type Account;

    type LoginEntity: Element<Config = ()>;

    fn new_login_entity(&self, account: &Self::Account) -> (Self::LoginEntity, u16);

}


/// Internal structure used to keep track of a client that is switching
/// from the login app after a successful login to a specific base app.
#[derive(Debug)]
struct PendingClient<A> {
    /// Underlying account.
    account: Box<A>,
    /// The socket address of the client, used to double check with the
    /// token to avoid spoofing.
    addr: SocketAddr,
    /// The blowfish key that should be used for the client's channel 
    /// just after authorization.
    blowfish: Arc<Blowfish>,
    /// The instant this pending client was added, used for timing out.
    instant: Instant,
}

impl<A> PendingClient<A> {

    #[inline]
    pub fn new(account: Box<A>, addr: SocketAddr, blowfish: Arc<Blowfish>) -> Self {
        Self { account, addr, blowfish, instant: Instant::now() }
    }

}

/// Internal structure used to track a client logged in the base app.
#[derive(Debug)]
struct Client<A> {
    /// Underlying account.
    account: Box<A>,
    /// The session key of the client.
    session_key: u32,
    /// Tracked client state.
    state: ClientState,
}

impl<A> Client<A> {

    #[inline]
    pub fn new(account: Box<A>, session_key: u32) -> Self {
        Self { 
            account,
            session_key, 
            state: ClientState::Initial,
        }
    }

}

/// State for a client, used for login procedure in order to send the
/// right data.
#[derive(Debug)]
enum ClientState {
    /// Initial state of the client, the login entity must be sent.
    Initial,
    /// The login entity has been sent, the account entity must be sent.
    LoginSent,
    /// The account entity has been sent.
    AccountSent,
}

/// Internal base app timer.
#[derive(Debug)]
struct Timer {
    /// Start time of the base app, used to know the game time.
    start_time: Instant,
    /// The frequency of update in Hz (cannot exceed 255 Hz).
    update_freq: u8,
}

impl Timer {
    
    /// Get the current total time of execution of this base app, 
    /// in seconds.
    #[inline]
    pub fn current_time(&self) -> u32 {
        self.start_time.elapsed().as_secs() as u32
    }

    /// Just wrap around the current time for tick.
    #[inline]
    pub fn current_time_tick(&self) -> u8 {
        self.current_time() as u8
    }

    /// Internal method used to write a tick sync element to the given
    /// bundle element writer.
    fn timestamp_element(&self, mut writer: BundleElementWriter) {
        writer.write_simple(client_id::TICK_SYNC, TickSync {
            tick: self.current_time_tick()
        })
    }

}
