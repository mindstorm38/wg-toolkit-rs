//! Implementation of a simple demonstration WoT server.

pub mod gen;

use std::collections::{hash_map, HashMap, HashSet};
use std::net::{SocketAddr, SocketAddrV4};
use std::{fs, thread};
use std::sync::{Arc, Mutex};

use rsa::rand_core::{OsRng, RngCore};
use rsa::pkcs8::DecodePrivateKey;
use rsa::RsaPrivateKey;

use blowfish::Blowfish;

use tracing::level_filters::LevelFilter;
use tracing::{info, warn};

use wgtk::net::app::{login, base};

use crate::{CliResult, WotArgs};


/// Entrypoint.
pub fn cmd_wot(args: WotArgs) -> CliResult<()> {

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::builder()
            .with_default_directive(LevelFilter::TRACE.into())
            .from_env_lossy())
        .init();

    let mut login_app = login::App::new(SocketAddr::V4(args.login_app))
        .map_err(|e| format!("Failed to bind login app: {e}"))?;

    let base_app = base::App::new(SocketAddr::V4(args.base_app))
        .map_err(|e| format!("Failed to bind base app: {e}"))?;

    if let Some(priv_key_path) = args.priv_key_path.as_deref() {

        let priv_key_content = fs::read_to_string(priv_key_path)
            .map_err(|e| format!("Failed to read private key at {}: {e}", priv_key_path.display()))?;

        let priv_key = Arc::new(RsaPrivateKey::from_pkcs8_pem(&priv_key_content)
            .map_err(|e| format!("Failed to decode PKCS#8 private key: {e}"))?);

        login_app.set_private_key(priv_key);

    }

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

    Ok(())

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

    fn run(mut self) {

        use login::Event;

        info!(target: "login", "Running on: {}", self.app.addr().unwrap());

        if self.app.has_private_key() {
            info!(target: "login", "Encryption enabled");
        }

        loop {

            match self.app.poll() {
                Event::IoError(error) => {
                    if let Some(addr) = error.addr {
                        warn!(target: "login", %addr, "Error: {}", error.error);
                    } else {
                        warn!(target: "login", "Error: {}", error.error);
                    }
                }
                Event::Ping(ping) => {
                    info!(target: "login", addr = %ping.addr, "Ping-Pong: {:?}", ping.latency);
                }
                Event::Login(login) => {
                    
                    if !*self.login_challenges.entry(login.addr).or_default() {
                        info!(target: "login", addr = %login.addr, "Login pending, sending challenge");
                        self.app.answer_login_challenge(login.addr);
                    } else {

                        info!(target: "login", addr = %login.addr, "Login success");

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
                    info!(target: "login", addr = %challenge.addr, "Challenge successful...");
                    if let Some(completed) = self.login_challenges.get_mut(&challenge.addr) {
                        *completed = true;
                    }
                }
            }

        }

    }

}

impl BaseThread {

    fn run(mut self) {

        info!(target: "base", "Running on: {}", self.app.addr().unwrap());

        loop {

            match self.app.poll() {
                base::Event::IoError(error) => {
                    if let Some(addr) = error.addr {
                        warn!(target: "base", %addr, "Error: {}", error.error);
                    } else {
                        warn!(target: "base", "Error: {}", error.error);
                    }
                }
                base::Event::Login(login) => {
                    
                    let mut clients = self.shared.login_clients.lock().unwrap();
                    let client = match clients.remove(&login.login_key) {
                        Some(client) => client,
                        None => {
                            info!(target: "base", addr = %login.addr, "Login #{}... Invalid key", login.attempt_num);
                            continue;
                        }
                    };

                    if client.addr != login.addr {
                        info!(target: "base", addr = %login.addr, "Login #{}... Invalid address", login.attempt_num);
                        continue;
                    }
                    
                    info!(target: "base", addr = %login.addr, "Login #{}... Success", login.attempt_num);
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
