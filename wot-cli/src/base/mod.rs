use std::net::{SocketAddr, SocketAddrV4};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::time::Instant;
use std::borrow::Cow;
use std::sync::Arc;
use std::io;

use blowfish::Blowfish;
use rand::rngs::OsRng;
use rand::RngCore;

use wgtk::net::bundle::{ElementReader, Bundle};
use wgtk::net::socket::{BundleSocket, Event, EventKind};

use wgtk::net::element::base::{
    ClientAuth, ServerSessionKey, ClientSessionKey,
    CellEntityMethod,
    BaseEntityMethod,
};

use wgtk::net::element::client::{
    UpdateFrequencyNotification,
    CreateBasePlayer,
    SelectPlayerEntity,
    EntityMethod,
    ResetEntities,
    TickSync,
};

use crate::common::server_settings::ServerSettings;
use crate::common::entity;


/// The state of the base app. It is the app where player are playing.
pub struct BaseApp {
    /// Underlying application.
    pub app: BundleSocket,
    /// List of clients pending for switching from login app to base app.
    pending_clients: HashMap<u32, PendingBaseClient>,
    /// List of clients logged in the base app mapped to their socket address.
    logged_clients: HashMap<SocketAddr, BaseClient>,
    /// A counter for allocating the unique key for logged Client.
    logged_counter: u32,
    /// Start time of the base app, used to know the game time.
    start_time: Instant,
    /// The server settings, sent to new clients.
    server_settings: Box<ServerSettings>,
    /// The required version string, sent to the client and then checked by it.
    required_version: String,
}

impl BaseApp {

    /// Default update frequency to 10 Hz.
    const UPDATE_FREQ: u8 = 10;

    pub fn new(addr: SocketAddrV4, server_settings: Box<ServerSettings>) -> io::Result<Self> {
        Ok(Self {
            app: BundleSocket::new(addr)?,
            pending_clients: HashMap::new(),
            logged_clients: HashMap::new(),
            logged_counter: 0,
            start_time: Instant::now(),
            server_settings,
            required_version: "eu_1.19.1_4".into(),
        })
    }

    #[inline]
    pub fn set_required_version(&mut self, version: impl Into<String>) {
        self.required_version = version.into();
    }

    pub fn handle(&mut self, event: &Event) {

        match &event.kind {
            EventKind::Bundle(bundle) => {
                let mut reader = bundle.element_reader();
                while let Some(element) = reader.next_element() {
                    if !self.handle_element(event.addr, element) {
                        break
                    }
                }
            }
            EventKind::PacketError(_, error) => {
                println!("[BASE/{}] Packet error: {error:?}", event.addr);
            }
        }

    }

    fn handle_element(&mut self, addr: SocketAddr, element: ElementReader) -> bool {

        let mut prefix = format!("[BASE/{addr}]");

        let mut logged_client = self.logged_clients.get_mut(&addr);
        if let Some(_) = logged_client.as_deref_mut() {
            prefix.push_str(" (client)");
        }

        match element {
            ElementReader::Top(ClientAuth::ID, reader) => {

                let client_auth = reader.read_simple::<ClientAuth>().unwrap();

                println!("{prefix} --> Auth, login key: {}, attempt: {}, unk: {}", 
                    client_auth.element.login_key, 
                    client_auth.element.attempts_count,
                    client_auth.element.unk
                );

                if let Some(pending_login) = self.pending_clients.remove(&client_auth.element.login_key) {
                    if pending_login.addr == addr {

                        println!("{prefix}     Enabling channel encryption");
                        self.app.set_channel(addr, pending_login.blowfish);

                        self.logged_counter = self.logged_counter.checked_add(1).expect("too much logged clients");
                        let logged_key = self.logged_counter;

                        self.logged_clients.insert(addr, BaseClient::new(logged_key));

                        println!("{prefix} <-- Session key: {logged_key}");
                        self.app.send(Bundle::new().write_simple_reply(ServerSessionKey {
                            session_key: logged_key,
                        }, client_auth.request_id.unwrap()), addr).unwrap();

                    } else {
                        println!("{prefix}     Incoherent address, expected {}", pending_login.addr);
                    }
                } else {
                    println!("{prefix}     Invalid key, discarding");
                }

                true

            }
            ElementReader::Top(ClientSessionKey::ID, reader) => {
                
                let client_session_auth = reader.read_simple::<ClientSessionKey>().unwrap();
                let session_key = client_session_auth.element.session_key;

                println!("{prefix} --> Session key: {session_key}");

                if let Some(client) = logged_client.as_deref_mut() {
                    if session_key == client.session_key {

                        let mut bundle = Bundle::new();

                        if !client.login_sent {

                            client.login_sent = true;
                            client.account_to_send = true;

                            bundle.write_simple_element(UpdateFrequencyNotification::ID, UpdateFrequencyNotification {
                                frequency: Self::UPDATE_FREQ,
                                game_time: self.current_time(),
                            });
                            println!("{prefix} <-- Update frequency: {}", Self::UPDATE_FREQ);
                            self.timestamp_bundle(&mut bundle);
                            self.app.send(&mut bundle, addr).unwrap();
                            bundle.clear();
                            
                            self.timestamp_bundle(&mut bundle);
                            bundle.write_simple_element(CreateBasePlayer::ID, CreateBasePlayer {
                                entity_id: 37289213,
                                entity_type: 11,
                                unk: String::new(),
                                entity_data: entity::Login { 
                                    account_db_id: "09518858105".into(),
                                },
                                entity_components_count: 0,
                            });
                            println!("{prefix} <-- Create base player: Login");
                            self.app.send(&mut bundle, addr).unwrap();
                            bundle.clear();

                            self.timestamp_bundle(&mut bundle);
                            bundle.write_simple_element(SelectPlayerEntity::ID, SelectPlayerEntity);
                            bundle.write_simple_element(EntityMethod::index_to_id(2), UnknownElement(vec![
                                21, 7, 100, 101, 102, 97, 117, 108, 116, 12, 128, 2, 93, 113, 1, 40, 75, 201, 75, 202, 101, 46
                            ]));
                            println!("{prefix} <-- Select player entity");
                            println!("{prefix} <-- Login.setPeripheryRoutingGroup");
                            self.app.send(&mut bundle, addr).unwrap();
                            bundle.clear();

                            self.timestamp_bundle(&mut bundle);
                            bundle.write_simple_element(ResetEntities::ID, ResetEntities { 
                                keep_player_on_base: false
                            });
                            println!("{prefix} <-- Reset entities (false)");
                            self.app.send(&mut bundle, addr).unwrap();
                            bundle.clear();

                        } else if client.account_to_send {

                            client.account_to_send = false;

                            self.timestamp_bundle(&mut bundle);
                            bundle.write_simple_element(CreateBasePlayer::ID, CreateBasePlayer {
                                entity_id: 37289214,
                                entity_type: 1,
                                unk: String::new(),
                                entity_data: entity::Account {
                                    required_version: self.required_version.clone(),
                                    name: "Mindstorm38_".into(),
                                    initial_server_settings: Cow::Borrowed(&self.server_settings),
                                },
                                entity_components_count: 6,
                            });
                            println!("{prefix} <-- Create base player: Account");
                            self.app.send(&mut bundle, addr).unwrap();
                            bundle.clear();

                            self.timestamp_bundle(&mut bundle);
                            bundle.write_simple_element(SelectPlayerEntity::ID, SelectPlayerEntity);
                            bundle.write_simple_element(EntityMethod::index_to_id(43), UnknownElement(vec![
                                32, 1, 255, 28, 1, 0, 128, 2, 125, 113, 1, 40, 85, 9, 115, 101, 114, 118, 101, 114, 85, 84, 67, 113, 2, 71, 65, 216, 255, 76, 97, 86, 57, 210, 85, 15, 99, 117, 114, 114, 101, 110, 116, 86, 101, 104, 73, 110, 118, 73, 68, 113, 3, 75, 0, 85, 10, 100, 97, 116, 97, 98, 97, 115, 101, 73, 68, 113, 4, 74, 121, 37, 237, 30, 85, 14, 97, 111, 103, 97, 115, 83, 116, 97, 114, 116, 101, 100, 65, 116, 113, 5, 71, 65, 216, 255, 76, 97, 85, 226, 109, 85, 16, 115, 101, 115, 115, 105, 111, 110, 83, 116, 97, 114, 116, 101, 100, 65, 116, 113, 6, 74, 133, 49, 253, 99, 85, 22, 98, 111, 111, 116, 99, 97, 109, 112, 67, 111, 109, 112, 108, 101, 116, 101, 100, 67, 111, 117, 110, 116, 113, 7, 75, 1, 85, 14, 105, 115, 65, 111, 103, 97, 115, 69, 110, 97, 98, 108, 101, 100, 113, 8, 136, 85, 16, 98, 111, 111, 116, 99, 97, 109, 112, 82, 117, 110, 67, 111, 117, 110, 116, 113, 9, 75, 0, 85, 14, 99, 111, 108, 108, 101, 99, 116, 85, 105, 83, 116, 97, 116, 115, 113, 10, 136, 85, 28, 105, 115, 76, 111, 110, 103, 68, 105, 115, 99, 111, 110, 110, 101, 99, 116, 101, 100, 70, 114, 111, 109, 67, 101, 110, 116, 101, 114, 113, 11, 137, 85, 11, 108, 111, 103, 85, 88, 69, 118, 101, 110, 116, 115, 113, 12, 137, 85, 20, 98, 111, 111, 116, 99, 97, 109, 112, 78, 101, 101, 100, 65, 119, 97, 114, 100, 105, 110, 103, 113, 13, 137, 117, 46
                            ]));
                            println!("{prefix} <-- Select player entity");
                            println!("{prefix} <-- Account.showGUI");
                            self.app.send(&mut bundle, addr).unwrap();
                            bundle.clear();

                        }

                    } else {
                        println!("{prefix}     Warning, expected: {}", client.session_key);
                    }
                } else {
                    println!("{prefix}     Warning, no client");
                }

                true

            }
            ElementReader::Top(CellEntityMethod::FIRST_ID..=CellEntityMethod::LAST_ID, reader) => {

                let method = reader.read_simple::<CellEntityMethod>().unwrap();
                let method_idx = CellEntityMethod::id_to_index(method.id);

                println!("{prefix} --> Cell entity method #{method_idx}: {method:?}");

                true

            }
            ElementReader::Top(id @ BaseEntityMethod::FIRST_ID..=BaseEntityMethod::LAST_ID, reader) => {

                let method = reader.read_simple::<BaseEntityMethod>().unwrap();
                let method_idx = BaseEntityMethod::id_to_index(id);

                println!("{prefix} --> Base entity method #{method_idx}: {method:?}");

                true

            }
            ElementReader::Top(id, _) => {
                println!("{prefix} --> Unknown #{id}");
                false
            }
            ElementReader::Reply(id, _) => {
                println!("{prefix} --> Unknown reply to #{id}");
                false
            }
        }

    }

    /// Allocate a new pending client for the given socket address and blowfish key.
    pub fn alloc_pending_client(&mut self, addr: SocketAddr, bf: &Arc<Blowfish>) -> u32 {
        loop {
            let key = OsRng.next_u32();
            match self.pending_clients.entry(key) {
                Entry::Vacant(v) => {
                    v.insert(PendingBaseClient::new(addr, bf.clone()));
                    break key
                }
                _ => continue
            }
        }
    }

    /// Get the current run time of the server in seconds.
    fn current_time(&self) -> u32 {
        self.start_time.elapsed().as_secs() as _
    }

    /// Just wrap around the current time for tick.
    fn current_time_tick(&self) -> u8 {
        self.current_time() as u8
    }

    /// Append a tick sync message to this bundle according to the current time.
    fn timestamp_bundle(&self, bundle: &mut Bundle) {
        bundle.write_simple_element(TickSync::ID, TickSync { 
            tick: self.current_time_tick() 
        });
    }

}


/// Internal structure used to keep track of a client that is switching
/// from the login app after a successful login. 
#[derive(Debug)]
pub struct PendingBaseClient {
    addr: SocketAddr,
    blowfish: Arc<Blowfish>,
}

impl PendingBaseClient {

    #[inline]
    pub fn new(addr: SocketAddr, blowfish: Arc<Blowfish>) -> Self {
        Self { addr, blowfish, }
    }

}


/// Internal structure used to track a client logged in the base app.
#[derive(Debug)]
pub struct BaseClient {
    /// The session key of the client.
    session_key: u32,
    /// True when the login procedure has been sent.
    login_sent: bool,
    /// Set to true when the account entity must be sent.
    account_to_send: bool,
}

impl BaseClient {

    #[inline]
    pub fn new(session_key: u32) -> Self {
        Self { 
            session_key, 
            login_sent: false,
            account_to_send: false,
        }
    }

}
