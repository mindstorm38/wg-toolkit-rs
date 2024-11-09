//! Implementation of a simple demonstration WoT server.

pub mod gen;

use std::collections::{hash_map, HashMap};
use std::net::{SocketAddr, SocketAddrV4};
use std::sync::{Arc, Mutex};
use std::{fs, thread};

use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::rand_core::{OsRng, RngCore};
use rsa::{RsaPrivateKey, RsaPublicKey};

use blowfish::Blowfish;

use tracing::level_filters::LevelFilter;
use tracing::{error, info, instrument, warn};

use wgtk::net::bundle::{Bundle, ElementReader, TopElementReader};
use wgtk::net::element::{SimpleElement, DebugElementUndefined};
use wgtk::net::app::proxy::{self, PacketDirection};
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

#[derive(Debug)]
struct LoginProxyThread {
    app: login::proxy::App,
    shared: Arc<ProxyShared>,
}

#[derive(Debug)]
struct BaseProxyThread {
    app: proxy::App,
    shared: Arc<ProxyShared>,
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
                    match bundle.direction {
                        PacketDirection::Out => self.read_out_bundle(bundle.addr, bundle.bundle),
                        PacketDirection::In => self.read_in_bundle(bundle.addr, bundle.bundle),
                    }
                }
                    
            }
        }

    }

    fn read_out_bundle(&mut self, addr: SocketAddr, bundle: Bundle) {

        let mut reader = bundle.element_reader();
        while let Some(elt) = reader.next_element() {
            match elt {
                ElementReader::Top(elt) => {
                    if !self.read_out_element(addr, elt) {
                        break;
                    }
                }
                ElementReader::Reply(elt) => {
                    let request_id = elt.request_id();
                    let _elt = elt.read_simple::<()>().unwrap();
                    warn!(%addr, "-> Reply element #{request_id}");
                    break;
                }
            }
        }

    }

    fn read_out_element(&mut self, addr: SocketAddr, elt: TopElementReader) -> bool {
        
        use base::element::*;

        match elt.id() {
            ClientSessionKey::ID => {
                let elt = elt.read_simple::<ClientSessionKey>().unwrap();
                assert!(elt.request_id.is_none());
                info!(%addr, "-> Session key: 0x{:08X}", elt.element.session_key);
                true
            }
            id => {
                let elt = elt.read_simple::<()>().unwrap();
                warn!(%addr, "-> Top element #{id}, request: {:?}", elt.request_id);
                false
            }
        }

    }

    fn read_in_bundle(&mut self, addr: SocketAddr, bundle: Bundle) {

        let mut reader = bundle.element_reader();
        while let Some(elt) = reader.next_element() {
            match elt {
                ElementReader::Top(elt) => {
                    if !self.read_in_element(addr, elt) {
                        break;
                    }
                }
                ElementReader::Reply(elt) => {
                    let request_id = elt.request_id();
                    let _elt = elt.read_simple::<()>().unwrap();
                    warn!(%addr, "<- Reply element #{request_id}");
                    break;
                }
            }
        }

    }

    fn read_in_element(&mut self, addr: SocketAddr, elt: TopElementReader) -> bool {

        use client::element::*;

        match elt.id() {
            UpdateFrequencyNotification::ID => {
                let elt = elt.read_simple::<UpdateFrequencyNotification>().unwrap();
                assert!(elt.request_id.is_none());
                info!(%addr, "<- Update frequency: {} Hz, game time: {}", elt.element.frequency, elt.element.game_time);
                true
            }
            TickSync::ID => {
                let elt = elt.read_simple::<TickSync>().unwrap();
                assert!(elt.request_id.is_none());
                info!(%addr, "<- Tick sync: {}", elt.element.tick);
                true
            }
            ResetEntities::ID => {
                let elt = elt.read_simple::<ResetEntities>().unwrap();
                assert!(elt.request_id.is_none());
                info!(%addr, "<- Reset entities, keep player on base: {}", elt.element.keep_player_on_base);
                true
            }
            CreateBasePlayer::ID => {
                let elt = elt.read_simple::<CreateBasePlayer>().unwrap();
                assert!(elt.request_id.is_none());
                warn!(%addr, "<- Create base player: {:?}", elt.element);
                true
            }
            CreateCellPlayer::ID => {
                let elt = elt.read_simple::<CreateCellPlayer>().unwrap();
                assert!(elt.request_id.is_none());
                warn!(%addr, "<- Create cell player: {:?}", elt.element);
                true
            }
            SelectPlayerEntity::ID => {
                let elt = elt.read_simple::<SelectPlayerEntity>().unwrap();
                assert!(elt.request_id.is_none());
                info!(%addr, "<- Select player entity");
                true
            }
            ResourceHeader::ID => {
                let elt = elt.read_simple::<ResourceHeader>().unwrap();
                assert!(elt.request_id.is_none());
                info!(%addr, "<- Resource header: {:?}", elt.element);
                true
            }
            ResourceFragment::ID => {
                let elt = elt.read_simple::<ResourceFragment>().unwrap();
                assert!(elt.request_id.is_none());
                info!(%addr, "<- Resource fragment: {:?}", elt.element);
                true
            }
            id if id::ENTITY_METHOD.contains(id) => {
                let elt = elt.read_simple::<DebugElementUndefined<0>>().unwrap();
                warn!(%addr, "<- Entity method: [{id}] {:?} (request: {:?})", elt.element, elt.request_id);
                false
            }
            id if id::ENTITY_PROPERTY.contains(id) => {
                let elt = elt.read_simple::<DebugElementUndefined<0>>().unwrap();
                warn!(%addr, "<- Entity property: [{id}] {:?} (request: {:?})", elt.element, elt.request_id);
                false
            }
            id => {
                let elt = elt.read_simple::<()>().unwrap();
                error!(%addr, "<- Top element #{id}, request: {:?}", elt.request_id);
                false
            }
        }
    }

}



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
