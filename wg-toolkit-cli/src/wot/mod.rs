//! Implementation of a simple demonstration WoT server.

use std::collections::{hash_map, HashMap};
use std::net::{SocketAddr, SocketAddrV4};
use std::{fs, thread};
use std::sync::{Arc, Mutex};

use rsa::rand_core::{OsRng, RngCore};
use rsa::pkcs8::DecodePrivateKey;
use rsa::RsaPrivateKey;

use blowfish::Blowfish;

use wgtk::net::app::{login, base};

use crate::{CliResult, WotArgs};


/// Entrypoint.
pub fn cmd_wot(args: WotArgs) -> CliResult<()> {

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

        println!("[L] Running on: {}", self.app.addr());

        if self.app.has_private_key() {
            println!("[L] Encryption enabled");
        }

        loop {

            match self.app.poll() {
                Event::IoError(error) => {
                    if let Some(addr) = error.addr {
                        println!("[L] [{addr}] Error: {}", error.error)
                    } else {
                        println!("[L] Error: {}", error.error)
                    }
                }
                Event::Ping(ping) => {
                    println!("[L] [{}] Ping-Pong: {:?}", ping.addr, ping.latency);
                }
                Event::Login(login) => {
                    
                    println!("[L] [{}] Login...", login.addr);

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
                Event::Challenge(challenge) => {
                    println!("[L] [{}] Challenge...", challenge.addr);
                }
            }

        }

    }

}

impl BaseThread {

    fn run(mut self) {

        println!("[B] Running on: {}", self.app.addr());

        loop {

            match self.app.poll() {
                base::Event::IoError(error) => {
                    if let Some(addr) = error.addr {
                        println!("[B] [{addr}] Error: {}", error.error)
                    } else {
                        println!("[B] Error: {}", error.error)
                    }
                }
                base::Event::Login(login) => {
                    
                    let mut clients = self.shared.login_clients.lock().unwrap();
                    let client = match clients.remove(&login.login_key) {
                        Some(client) => client,
                        None => {
                            println!("[B] [{}] Login #{}... Invalid key", login.addr, login.attempt_num);
                            continue;
                        }
                    };

                    if client.addr != login.addr {
                        println!("[B] [{}] Login #{}... Invalid address", login.addr, login.attempt_num);
                        continue;
                    }
                    
                    println!("[B] [{}] Login #{}... Success", login.addr, login.attempt_num);
                    self.app.answer_login_success(login.addr, client.blowfish);

                }
            }
            
        }

    }

}
