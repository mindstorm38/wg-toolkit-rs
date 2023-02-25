use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use std::sync::Arc;
use std::env;
use std::fs;

use rand::rngs::OsRng;
use rand::RngCore;

use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey};

use crypto_common::KeyInit;
use blowfish::Blowfish;

use wgtk::net::bundle::{BundleElement, Bundle};
use wgtk::net::app::{App, EventKind, Event};

use wgtk::net::element::ping::PingCodec;
use wgtk::net::element::login::{
    LoginRequestCodec, 
    LoginResponseCodec, LoginResponse, LoginChallenge, 
    ChallengeResponseCodec, CuckooCycleResponseCodec, LoginSuccess
};

use wgtk::util::TruncateFmt;


fn main() {

    let priv_key_path = env::var("WGT_PRIVKEY_PATH").unwrap();
    let priv_key_content = fs::read_to_string(priv_key_path).unwrap();
    let priv_key = RsaPrivateKey::from_pkcs8_pem(priv_key_content.as_str()).unwrap();

    let mut login_app = LoginApp {
        app: App::new("127.0.0.1:20016".parse().unwrap()).unwrap(),
        priv_key: Arc::new(priv_key),
        clients: HashMap::new(),
    };

    let mut base_app = BaseApp {
        app: App::new("127.0.0.1:20017".parse().unwrap()).unwrap(),
        clients: HashMap::new(),
        clients_keys: HashMap::new(),
    };

    let mut events = Vec::new();

    loop {
        
        login_app.app.poll(&mut events, Some(Duration::from_millis(10))).unwrap();
        for event in &events {
            login_app.handle(&event, &mut base_app);
        }

        base_app.app.poll(&mut events, Some(Duration::from_millis(10))).unwrap();
        for event in &events {
            base_app.handle(event);
        }

    }

}


/// The state of the login app. This handles login requests, and if validated
/// create a client in the base app and send a success response with the base
/// app address.
pub struct LoginApp {
    /// The application.
    app: App,
    /// The RSA private key for login app.
    priv_key: Arc<RsaPrivateKey>,
    /// A client for the login app.
    clients: HashMap<SocketAddr, LoginClient>,
}

impl LoginApp {

    pub fn handle(&mut self, event: &Event, base_app: &mut BaseApp) {
        match &event.kind {
            EventKind::Bundle(bundle) => {
                let mut reader = bundle.get_element_reader();
                while let Some(element) = reader.next_element() {
                    if !self.handle_element(event.addr, element, &mut *base_app) {
                        break
                    }
                }
            }
            EventKind::PacketError(_, error) => {
                println!("[LOGIN/{}] Packet error: {error:?}", event.addr);
            }
        }
    }

    fn handle_element(&mut self, addr: SocketAddr, element: BundleElement, base_app: &mut BaseApp) -> bool {

        let client = match self.clients.entry(addr) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(LoginClient::new(addr)),
        };

        let prefix = format!("[LOGIN/{}]", client.addr);

        match element {
            BundleElement::Simple(PingCodec::ID, reader) => {
    
                let elt = reader.read(&PingCodec).unwrap();
                println!("{prefix} --> Ping #{}", elt.element);
                println!("{prefix} <-- Pong #{}", elt.element);
    
                let mut bundle = Bundle::new_empty(true);
                bundle.add_reply(&PingCodec, elt.element, elt.request_id.unwrap());
                self.app.send(&mut bundle, client.addr).unwrap();
                
                true
    
            }
            BundleElement::Simple(LoginRequestCodec::ID, reader) => {
    
                let codec = LoginRequestCodec::Server(self.priv_key.clone());
                let elt = reader.read(&codec).unwrap();

                println!("{prefix} --> Login {} / {}", TruncateFmt(&elt.element.username, 54), elt.element.password);
    
                // Ensure that blowfish key is set.
                let bf = client.blowfish.insert(Arc::new(Blowfish::new_from_slice(&elt.element.blowfish_key).unwrap()));
                
                let mut bundle = Bundle::new_empty(true);
                
                if !client.challenge_complete {
                    
                    let cuckoo_prefix_value = OsRng.next_u64();
                    let cuckoo_prefix = format!("{cuckoo_prefix_value:>02X}");
                    let cuckoo_easiness = 0.9;
    
                    let challenge = LoginChallenge::CuckooCycle { 
                        prefix: cuckoo_prefix, 
                        max_nonce: ((1 << 20) as f32 * cuckoo_easiness) as _
                    };
    
                    println!("{prefix} <-- Cuckoo Cycle Challenge");

                    bundle.add_reply(
                        &LoginResponseCodec::Encrypted(bf.clone()), 
                        LoginResponse::Challenge(challenge), 
                        elt.request_id.unwrap()
                    );
    
                } else {
    
                    // NOTE: We are currently not checking anything prior to connection.
                    // No password, no challenge is required.
    
                    let success = LoginSuccess {
                        addr: base_app.app.addr(),
                        session_key: base_app.alloc_client(client.addr, &*bf),
                        server_message: String::new(),
                    };
    
                    println!("{prefix} <-- Success, addr: {}, key: {}", success.addr, success.session_key);

                    bundle.add_reply(
                        &LoginResponseCodec::Encrypted(bf.clone()), 
                        LoginResponse::Success(success), 
                        elt.request_id.unwrap()
                    );
                    
                }
    
                self.app.send(&mut bundle, client.addr).unwrap();
    
                true
    
            }
            BundleElement::Simple(ChallengeResponseCodec::ID, reader) => {
                let codec = ChallengeResponseCodec::new(CuckooCycleResponseCodec);
                let _ = reader.read(&codec).unwrap();
                println!("{prefix} --> Challenge Response");
                client.challenge_complete = true;
                true
            }
            BundleElement::Simple(id, _) => {
                println!("{prefix} --> Unknown #{id}");
                false
            }
            BundleElement::Reply(id, _) => {
                println!("{prefix} --> Unknown reply to #{id}");
                false
            }
        }

    }

}


/// The state of the base app. It is the app where player are playing.
pub struct BaseApp {
    /// Underlying application.
    app: App,
    /// List of clients logged in the base app.
    clients: HashMap<SocketAddr, BaseClient>,
    clients_keys: HashMap<u32, SocketAddr>,
}

impl BaseApp {

    pub fn handle(&mut self, event: &Event) {

        match &event.kind {
            EventKind::Bundle(bundle) => {
                let mut reader = bundle.get_element_reader();
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

    fn handle_element(&mut self, addr: SocketAddr, element: BundleElement) -> bool {

        let prefix = format!("[BASE/{addr}]");
        println!("{prefix} --> {element:?}");

        match element {
            BundleElement::Simple(id, _) => {
                println!("{prefix} --> Unknown #{id}");
                false
            }
            BundleElement::Reply(id, _) => {
                println!("{prefix} --> Unknown reply to #{id}");
                false
            }
        }

    }

    /// Allocate a new client and returns its session key.
    pub fn alloc_client(&mut self, addr: SocketAddr, bf: &Arc<Blowfish>) -> u32 {
        loop {
            let key = OsRng.next_u32();
            match self.clients_keys.entry(key) {
                Entry::Vacant(v) => {
                    self.app.set_channel(addr, bf.clone());
                    self.clients.insert(addr, BaseClient::new(addr, key, bf.clone()));
                    v.insert(addr);
                    break key
                }
                _ => continue
            }
        }
    }

}


/// Internal structure used to track a client through login process.
#[derive(Debug)]
struct LoginClient {
    addr: SocketAddr,
    blowfish: Option<Arc<Blowfish>>,
    challenge_complete: bool,
}

impl LoginClient {

    #[inline]
    fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            blowfish: None,
            challenge_complete: false,
        }
    }

}


/// Internal structure used to track a client logged in the base app.
#[derive(Debug)]
struct BaseClient {
    addr: SocketAddr,
    session_key: u32,
    blowfish: Arc<Blowfish>,
}

impl BaseClient {

    #[inline]
    fn new(addr: SocketAddr, session_key: u32, blowfish: Arc<Blowfish>) -> Self {
        Self {
            addr,
            session_key,
            blowfish,
        }
    }

}
