//! Implementation of a simple demonstration WoT server.

pub mod gen;

use std::collections::{hash_map, HashMap};
use std::net::{SocketAddr, SocketAddrV4};
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use std::fmt::Debug;
use std::io;

use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::rand_core::{OsRng, RngCore};
use rsa::{RsaPrivateKey, RsaPublicKey};

use blowfish::Blowfish;
use tracing::level_filters::LevelFilter;

use tracing::{error, info, instrument, warn};

use wgtk::net::bundle::{Bundle, ElementReader, TopElementReader};
use wgtk::net::element::{DebugElementUndefined, DebugElementVariable16, SimpleElement};
use wgtk::net::app::proxy::{self, PacketDirection};
use wgtk::net::app::common::entity::Entity;
use wgtk::net::app::{login, base, client};

use crate::{CliResult, WotArgs};


/// Entrypoint.
pub fn cmd_wot(args: WotArgs) -> CliResult<()> {

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::builder()
            .with_default_directive(LevelFilter::TRACE.into())
            .from_env_lossy())
        .init();

    if let Some(real_login_app) = args.real_login_app {

        let real_encryption_key;
        if let Some(pub_key_path) = args.real_pub_key_path.as_deref() {
            
            let pub_key_content = fs::read_to_string(pub_key_path)
                .map_err(|e| format!("Failed to read public key at {}: {e}", pub_key_path.display()))?;

            let pub_key = Arc::new(RsaPublicKey::from_public_key_pem(&pub_key_content)
                .map_err(|e| format!("Failed to decode PEM public key: {e}"))?);

            real_encryption_key = Some(pub_key);

        } else {
            real_encryption_key = None;
        }

        let mut login_app = login::proxy::App::new(SocketAddr::V4(args.login_app), SocketAddr::V4(real_login_app), real_encryption_key)
            .map_err(|e| format!("Failed to bind login app: {e}"))?;

        if let Some(priv_key_path) = args.priv_key_path.as_deref() {

            let priv_key_content = fs::read_to_string(priv_key_path)
                .map_err(|e| format!("Failed to read private key at {}: {e}", priv_key_path.display()))?;

            let priv_key = Arc::new(RsaPrivateKey::from_pkcs8_pem(&priv_key_content)
                .map_err(|e| format!("Failed to decode PKCS#8 private key: {e}"))?);

            login_app.set_encryption(priv_key);

        }

        login_app.set_forced_base_app_addr(args.base_app);

        let base_app = proxy::App::new(SocketAddr::V4(args.base_app))
            .map_err(|e| format!("Failed to bind base app: {e}"))?;

        let shared = Arc::new(ProxyShared {
            pending_clients: Mutex::new(HashMap::new()),
        });

        let login_thread = LoginProxyThread {
            app: login_app,
            shared: Arc::clone(&shared),
        };

        let base_thread = BaseProxyThread {
            app: base_app,
            shared,
            entities: HashMap::new(),
            selected_entity_id: None,
            player_entity_id: None,
        };
        
        thread::scope(move |scope| {
            scope.spawn(move || login_thread.run());
            scope.spawn(move || base_thread.run());
        });
        
    } else {

        let mut login_app = login::App::new(SocketAddr::V4(args.login_app))
            .map_err(|e| format!("Failed to bind login app: {e}"))?;

        if let Some(priv_key_path) = args.priv_key_path.as_deref() {

            let priv_key_content = fs::read_to_string(priv_key_path)
                .map_err(|e| format!("Failed to read private key at {}: {e}", priv_key_path.display()))?;

            let priv_key = Arc::new(RsaPrivateKey::from_pkcs8_pem(&priv_key_content)
                .map_err(|e| format!("Failed to decode PKCS#8 private key: {e}"))?);

            login_app.set_encryption(priv_key);

        }

        let base_app = base::App::new(SocketAddr::V4(args.base_app))
            .map_err(|e| format!("Failed to bind base app: {e}"))?;

        let shared = Arc::new(Shared {
            login_clients: Mutex::new(HashMap::new()),
        });

        let login_thread = LoginThread {
            app: login_app,
            shared: Arc::clone(&shared),
            base_app_addr: args.base_app,
            login_challenges: HashMap::new(),
        };

        let base_thread = BaseThread {
            app: base_app,
            shared,
        };

        thread::scope(move |scope| {
            scope.spawn(move || login_thread.run());
            scope.spawn(move || base_thread.run());
        });

    }

    Ok(())

}

// ================= //
// ===== PROXY ===== //
// ================= //

#[derive(Debug)]
struct LoginProxyThread {
    app: login::proxy::App,
    shared: Arc<ProxyShared>,
}

#[derive(Debug)]
struct BaseProxyThread {
    app: proxy::App,
    shared: Arc<ProxyShared>,
    entities: HashMap<u32, &'static ProxyEntityType>,
    selected_entity_id: Option<u32>,
    player_entity_id: Option<u32>,
}

#[derive(Debug)]
struct ProxyShared {
    pending_clients: Mutex<HashMap<SocketAddr, ProxyPendingClient>>,
}

#[derive(Debug)]
struct ProxyPendingClient {
    base_app_addr: SocketAddrV4,
    blowfish: Arc<Blowfish>,
}

impl LoginProxyThread {

    #[instrument(name = "login proxy", skip_all)]
    fn run(mut self) {

        use login::proxy::Event;

        info!("Running on: {}", self.app.addr().unwrap());
        
        if self.app.has_encryption() {
            info!("Encryption enabled");
        }

        loop {
            match self.app.poll() {
                Event::IoError(error) => {
                    if let Some(addr) = error.addr {
                        warn!(%addr, "Error: {}", error.error);
                    } else {
                        warn!("Error: {}", error.error);
                    }
                }
                Event::Ping(ping) => {
                    info!(addr = %ping.addr, "Ping-Pong: {:?}", ping.latency);
                }
                Event::LoginSuccess(success) => {
                    info!(addr = %success.addr, "Login success");
                    self.shared.pending_clients.lock().unwrap().insert(success.addr, ProxyPendingClient { 
                        base_app_addr: success.real_base_app_addr,
                        blowfish: success.blowfish, 
                    });
                }
                Event::LoginError(error) => {
                    info!(addr = %error.addr, "Login error: {:?}", error.error);
                }
            }
        }

    }

}

impl BaseProxyThread {

    #[instrument(name = "base proxy", skip_all)]
    fn run(mut self) {

        use proxy::Event;

        info!("Running on: {}", self.app.addr().unwrap());

        loop {
            match self.app.poll() {
                Event::IoError(error) => {
                    if let Some(addr) = error.addr {
                        warn!(%addr, "Error: {}", error.error);
                    } else {
                        warn!("Error: {}", error.error);
                    }
                }
                Event::Rejection(rejection) => {
                    if let Some(pending_client) = self.shared.pending_clients.lock().unwrap().remove(&rejection.addr) {
                        
                        info!("Rejection of known peer: {} (to {})", rejection.addr, pending_client.base_app_addr);
                        
                        self.app.bind_peer(
                            rejection.addr, 
                            SocketAddr::V4(pending_client.base_app_addr), 
                            Some(pending_client.blowfish),
                            None).unwrap();

                    } else {
                        warn!("Rejection of unknown peer: {}", rejection.addr);
                    }
                }
                Event::Bundle(bundle) => {
                    
                    let res = match bundle.direction {
                        PacketDirection::Out => self.read_out_bundle(bundle.addr, bundle.bundle),
                        PacketDirection::In => self.read_in_bundle(bundle.addr, bundle.bundle),
                    };

                    if let Err(e) = res {
                        error!(addr = %bundle.addr, "Error while reading bundle: ({:?}) {e}", bundle.direction);
                    }

                }
                    
            }
        }

    }

    fn read_out_bundle(&mut self, addr: SocketAddr, bundle: Bundle) -> io::Result<()> {

        let mut reader = bundle.element_reader();
        while let Some(elt) = reader.next_element() {
            match elt {
                ElementReader::Top(elt) => {
                    if !self.read_out_element(addr, elt)? {
                        break;
                    }
                }
                ElementReader::Reply(elt) => {
                    let request_id = elt.request_id();
                    let _elt = elt.read_simple::<()>()?;
                    warn!(%addr, "-> Reply element #{request_id}");
                    break;
                }
            }
        }

        Ok(())

    }

    fn read_out_element(&mut self, addr: SocketAddr, elt: TopElementReader) -> io::Result<bool> {
        
        use base::element::*;

        match elt.id() {
            ClientSessionKey::ID => {
                let elt = elt.read_simple::<ClientSessionKey>()?;
                assert!(elt.request_id.is_none());
                info!(%addr, "-> Session key: 0x{:08X}", elt.element.session_key);
            }
            // TODO: Account::doCmdInt3 (AccountCommands.CMD_SYNC_DATA), exposed id: 0x0E, message id: 0x95
            id => {
                let elt = elt.read_simple::<DebugElementUndefined<0>>()?;
                error!(%addr, "-> Top element #{id} {:?} (request: {:?})", elt.element, elt.request_id);
                return Ok(false);
            }
        }

        Ok(true)

    }

    fn read_in_bundle(&mut self, addr: SocketAddr, bundle: Bundle) -> io::Result<()> {

        let mut reader = bundle.element_reader();
        while let Some(elt) = reader.next_element() {
            match elt {
                ElementReader::Top(elt) => {
                    if !self.read_in_element(addr, elt)? {
                        break;
                    }
                }
                ElementReader::Reply(elt) => {
                    let request_id = elt.request_id();
                    let _elt = elt.read_simple::<()>()?;
                    warn!(%addr, "<- Reply element #{request_id}");
                    break;
                }
            }
        }

        Ok(())

    }

    fn read_in_element(&mut self, addr: SocketAddr, mut elt: TopElementReader) -> io::Result<bool> {

        use client::element::*;

        match elt.id() {
            UpdateFrequencyNotification::ID => {
                let ufn = elt.read_simple::<UpdateFrequencyNotification>()?;
                assert!(ufn.request_id.is_none());
                info!(%addr, "<- Update frequency: {} Hz, game time: {}", ufn.element.frequency, ufn.element.game_time);
            }
            TickSync::ID => {
                let ts = elt.read_simple::<TickSync>()?;
                assert!(ts.request_id.is_none());
                info!(%addr, "<- Tick sync: {}", ts.element.tick);
            }
            ResetEntities::ID => {

                let re = elt.read_simple::<ResetEntities>()?;
                assert!(re.request_id.is_none());

                info!(%addr, "<- Reset entities, keep player on base: {}, entities: {}", 
                    re.element.keep_player_on_base, self.entities.len());

                // Don't delete player entity if requested...
                let mut player_entity = None;
                if re.element.keep_player_on_base {
                    if let Some(player_entity_id) = self.player_entity_id {
                        player_entity = Some(self.entities.remove_entry(&player_entity_id).unwrap());
                    }
                }
                
                self.entities.clear();
                self.player_entity_id = None;
                
                // Restore player entity!
                if let Some((player_entity_id, player_entity)) = player_entity {
                    self.entities.insert(player_entity_id, player_entity);
                    self.player_entity_id = Some(player_entity_id);
                }

            }
            CreateBasePlayerHeader::ID => {

                let cbp = elt.read_simple_stable::<CreateBasePlayerHeader>()?;
                assert!(cbp.request_id.is_none());

                if let Some(entity_type) = cbp.element.entity_type_id.checked_sub(1).and_then(|i| ENTITY_TYPES.get(i as usize)) {
                    self.entities.insert(cbp.element.entity_id, entity_type);
                    self.player_entity_id = Some(cbp.element.entity_id);
                    return (entity_type.create_base_player)(addr, elt);
                }

                self.player_entity_id = None;
                // It's possible to skip it because its len is variable.
                let dbg = elt.read_simple::<DebugElementVariable16<0>>()?;
                warn!(%addr, "<- Create base player with invalid entity type id: 0x{:02X}, {:?}", 
                    cbp.element.entity_type_id, dbg.element);

            }
            CreateCellPlayer::ID => {
                let ccp = elt.read_simple::<CreateCellPlayer>()?;
                assert!(ccp.request_id.is_none());
                warn!(%addr, "<- Create cell player: {:?}", ccp.element);
            }
            SelectPlayerEntity::ID => {
                let spe = elt.read_simple::<SelectPlayerEntity>()?;
                assert!(spe.request_id.is_none());
                if let Some(player_entity_id) = self.player_entity_id {
                    info!(%addr, "<- Select player entity: {player_entity_id}");
                } else {
                    warn!(%addr, "<- Select player entity: no player entity")
                }
                self.selected_entity_id = self.player_entity_id;
            }
            ResourceHeader::ID => {
                let rh = elt.read_simple::<ResourceHeader>()?;
                assert!(rh.request_id.is_none());
                info!(%addr, "<- Resource header: {:?}", rh.element);
            }
            ResourceFragment::ID => {
                let rf = elt.read_simple::<ResourceFragment>()?;
                assert!(rf.request_id.is_none());
                info!(%addr, "<- Resource fragment: {:?}", rf.element);
            }
            id if id::ENTITY_METHOD.contains(id) => {

                // Account::msg#37 = onClanInfoReceived
                // Account::msg#39 = showGUI

                if let Some(entity_id) = self.selected_entity_id {
                    // Unwrap because selected entity should exist!
                    let entity_type = *self.entities.get(&entity_id).unwrap();
                    return (entity_type.entity_method)(addr, entity_id, elt);
                }

                let elt = elt.read_simple::<DebugElementUndefined<0>>()?;
                warn!(%addr, "<- Entity method (unknown selected entity): msg#{} {:?} (request: {:?})", id - id::ENTITY_METHOD.first, elt.element, elt.request_id);
                return Ok(false);

            }
            id if id::ENTITY_PROPERTY.contains(id) => {
                let elt = elt.read_simple::<DebugElementUndefined<0>>()?;
                warn!(%addr, "<- Entity property: msg#{} {:?} (request: {:?})", id - id::ENTITY_PROPERTY.first, elt.element, elt.request_id);
                return Ok(false);
            }
            id => {
                let elt = elt.read_simple::<DebugElementUndefined<0>>()?;
                error!(%addr, "<- Top element #{id} {:?} (request: {:?})", elt.element, elt.request_id);
                return Ok(false);
            }
        }

        Ok(true)

    }

}


/// Represent an entity type and its associated static functions.
#[derive(Debug)]
struct ProxyEntityType {
    create_base_player: fn(SocketAddr, TopElementReader) -> io::Result<bool>,
    entity_method: fn(SocketAddr, u32, TopElementReader) -> io::Result<bool>,
}

impl ProxyEntityType {

    const fn new<E>() -> Self
    where
        E: Entity + Debug,
        E::ClientMethod: Debug,
    {
        Self {
            create_base_player: |addr, elt| {
                use client::element::CreateBasePlayer;
                let cbp = elt.read_simple::<CreateBasePlayer<E>>()?;
                info!(%addr, "<- Create base player: ({}) {:?}", cbp.element.entity_id, cbp.element.entity_data);
                Ok(true)
            },
            entity_method: |addr, entity_id, elt| {
                use client::element::EntityMethod;
                let em = elt.read_simple::<EntityMethod<E::ClientMethod>>()?;
                info!(%addr, "<- Entity method: ({entity_id}) {:?}", em.element.inner);
                Ok(true)
            },
        }
    }

}

const ENTITY_TYPES: &[ProxyEntityType] = &[
    ProxyEntityType::new::<gen::entity::Account>(),
    ProxyEntityType::new::<gen::entity::Avatar>(),
    ProxyEntityType::new::<gen::entity::ArenaInfo>(),
    ProxyEntityType::new::<gen::entity::ClientSelectableObject>(),
    ProxyEntityType::new::<gen::entity::HangarVehicle>(),
    ProxyEntityType::new::<gen::entity::Vehicle>(),
    ProxyEntityType::new::<gen::entity::AreaDestructibles>(),
    ProxyEntityType::new::<gen::entity::OfflineEntity>(),
    ProxyEntityType::new::<gen::entity::Flock>(),
    ProxyEntityType::new::<gen::entity::FlockExotic>(),
    ProxyEntityType::new::<gen::entity::Login>(),
];


// ==================== //
// ===== EMULATOR ===== //
// ==================== //

#[derive(Debug)]
struct LoginThread {
    app: login::App,
    shared: Arc<Shared>,
    base_app_addr: SocketAddrV4,
    login_challenges: HashMap<SocketAddr, bool>,
}

#[derive(Debug)]
struct BaseThread {
    app: base::App,
    shared: Arc<Shared>,
}

#[derive(Debug)]
struct Shared {
    login_clients: Mutex<HashMap<u32, LoginClient>>,
}

#[derive(Debug)]
struct LoginClient {
    addr: SocketAddr,
    blowfish: Arc<Blowfish>,
}

impl LoginThread {

    #[instrument(name = "login", skip_all)]
    fn run(mut self) {

        use login::Event;

        info!("Running on: {}", self.app.addr().unwrap());

        if self.app.has_encryption() {
            info!("Encryption enabled");
        }

        loop {
            match self.app.poll() {
                Event::IoError(error) => {
                    if let Some(addr) = error.addr {
                        warn!(%addr, "Error: {}", error.error);
                    } else {
                        warn!("Error: {}", error.error);
                    }
                }
                Event::Ping(ping) => {
                    info!(addr = %ping.addr, "Ping-Pong: {:?}", ping.latency);
                }
                Event::Login(login) => {

                    if !*self.login_challenges.entry(login.addr).or_default() {
                        info!(addr = %login.addr, "Login pending, sending challenge");
                        self.app.answer_login_challenge(login.addr);
                    } else {

                        info!(addr = %login.addr, "Login success");

                        let mut clients = self.shared.login_clients.lock().unwrap();
                        let (login_key, slot) = loop {
                            let login_key = OsRng.next_u32();
                            match clients.entry(login_key) {
                                hash_map::Entry::Occupied(_) => continue,
                                hash_map::Entry::Vacant(v) => break (login_key, v),
                            }
                        };

                        let blowfish = self.app.answer_login_success(login.addr, self.base_app_addr, login_key, String::new()).unwrap();

                        slot.insert(LoginClient {
                            addr: login.addr,
                            blowfish,
                        });

                        // app.answer_login_error(login.addr, LoginError::Banned, "{\"bans\":\"{\\\"expiryTime\\\":1726435530,\\\"reason\\\":\\\"It's the reason\\\"}\"}".to_string());
                        
                    }
                
                }
                Event::Challenge(challenge) => {
                    info!(addr = %challenge.addr, "Challenge successful...");
                    if let Some(completed) = self.login_challenges.get_mut(&challenge.addr) {
                        *completed = true;
                    }
                }
            }
        }

    }

}

impl BaseThread {

    #[instrument(name = "base", skip_all)]
    fn run(mut self) {

        info!("Running on: {}", self.app.addr().unwrap());

        loop {

            match self.app.poll() {
                base::Event::IoError(error) => {
                    if let Some(addr) = error.addr {
                        warn!(%addr, "Error: {}", error.error);
                    } else {
                        warn!("Error: {}", error.error);
                    }
                }
                base::Event::Login(login) => {
                    
                    let mut clients = self.shared.login_clients.lock().unwrap();
                    let client = match clients.remove(&login.login_key) {
                        Some(client) => client,
                        None => {
                            info!(addr = %login.addr, "Login #{}... Invalid key", login.attempt_num);
                            continue;
                        }
                    };

                    if client.addr != login.addr {
                        info!(addr = %login.addr, "Login #{}... Invalid address", login.attempt_num);
                        continue;
                    }
                    
                    info!(addr = %login.addr, "Login #{}... Success", login.attempt_num);
                    self.app.answer_login_success(login.addr, client.blowfish);

                }
                
            }

            // // Proof of concept:
            // let entity: Handle<entity::Login> = self.app.create_base_player(addr, entity::Login {
            //     accountDBID_s: "09518858105".to_string(),
            //     loginPriority: 0,
            // });

            // self.app.call_method(addr, entity, entity::Login_Client::setPeripheryRoutingGroup());
            // self.app.reset_entities(addr);

            // let entity: Handle<entity::Account> = self.app.create_base_player(addr, entity::Account {
            //     name: "Mindstorm38_".to_string(),

            // });

        }

    }

}
