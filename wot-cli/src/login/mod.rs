use std::collections::hash_map::Entry;
use std::net::{SocketAddr, SocketAddrV4};
use std::collections::HashMap;
use std::sync::Arc;
use std::io;

use blowfish::cipher::KeyInit;
use blowfish::Blowfish;
use rand::rngs::OsRng;
use rand::RngCore;
use rsa::RsaPrivateKey;

use wgtk::net::bundle::{ElementReader, Bundle};
use wgtk::net::socket::{WgSocket, Event, EventKind};
use wgtk::util::TruncateFmt;

use wgtk::net::element::login::{
    Ping,
    LoginRequest, LoginRequestEncryption,
    LoginResponse, LoginResponseEncryption,
    LoginChallenge, LoginSuccess,
    ChallengeResponse, CuckooCycleResponse,
};

use crate::base::BaseApp;


/// The state of the login app. This handles login requests, and if validated
/// create a client in the base app and send a success response with the base
/// app address.
pub struct LoginApp {
    /// The application.
    pub app: WgSocket,
    /// The RSA private key for login app.
    priv_key: Option<Arc<RsaPrivateKey>>,
    /// A client for the login app.
    clients: HashMap<SocketAddr, LoginClient>,
}

impl LoginApp {

    pub fn new(addr: SocketAddrV4, priv_key: Option<Arc<RsaPrivateKey>>) -> io::Result<Self> {
        Ok(Self {
            app: WgSocket::new(addr)?,
            priv_key,
            clients: HashMap::new(),
        })
    }

    pub fn handle(&mut self, event: &Event, base_app: &mut BaseApp) {
        match &event.kind {
            EventKind::Bundle(bundle) => {
                let mut reader = bundle.element_reader();
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

    fn handle_element(&mut self, addr: SocketAddr, element: ElementReader, base_app: &mut BaseApp) -> bool {

        let client = match self.clients.entry(addr) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(LoginClient::new(addr)),
        };

        let prefix = format!("[LOGIN/{}]", client.addr);

        match element {
            ElementReader::Top(Ping::ID, reader) => {
    
                let elt = reader.read_simple::<Ping>().unwrap();
                println!("{prefix} --> Ping #{}", elt.element.num);
                println!("{prefix} <-- Pong #{}", elt.element.num);
                
                self.app.send(Bundle::new()
                    .write_simple_reply(elt.element, elt.request_id.unwrap()), client.addr).unwrap();
                
                true
    
            }
            ElementReader::Top(LoginRequest::ID, reader) => {
    
                let encryption = match self.priv_key {
                    Some(ref key) => LoginRequestEncryption::Server(key.clone()),
                    None => LoginRequestEncryption::Clear,
                };

                let elt = reader.read::<LoginRequest>(&encryption).unwrap();

                println!("{prefix} --> Login {} / {}", TruncateFmt(&elt.element.username, 54), elt.element.password);
    
                // Ensure that blowfish key is set.
                let bf = client.blowfish.insert(Arc::new(Blowfish::new_from_slice(&elt.element.blowfish_key).unwrap()));
                let encryption = LoginResponseEncryption::Encrypted(bf.clone());

                let mut bundle = Bundle::new();
                
                if !client.challenge_complete {
                    
                    let cuckoo_prefix_value = OsRng.next_u64();
                    let cuckoo_prefix = format!("{cuckoo_prefix_value:>02X}");
                    let cuckoo_easiness = 0.9;
    
                    let challenge = LoginChallenge::CuckooCycle { 
                        prefix: cuckoo_prefix, 
                        max_nonce: ((1 << 20) as f32 * cuckoo_easiness) as _
                    };
    
                    println!("{prefix} <-- Cuckoo cycle challenge");

                    bundle.write_reply(
                        LoginResponse::Challenge(challenge), 
                        &encryption, 
                        elt.request_id.unwrap()
                    );

                } else {
    
                    // NOTE: We are currently not checking anything prior to connection.
                    // No password, no challenge is required.
    
                    let success = LoginSuccess {
                        addr: base_app.app.addr(),
                        login_key: base_app.alloc_pending_client(client.addr, &*bf),
                        server_message: String::new(),
                    };
    
                    println!("{prefix} <-- Success, addr: {}, login key: {}", success.addr, success.login_key);

                    bundle.write_reply(
                        LoginResponse::Success(success), 
                        &encryption, 
                        elt.request_id.unwrap()
                    );
                    
                }
    
                self.app.send(&mut bundle, client.addr).unwrap();
    
                true
    
            }
            ElementReader::Top(ChallengeResponse::ID, reader) => {
                let _ = reader.read_simple::<ChallengeResponse<CuckooCycleResponse>>().unwrap();
                println!("{prefix} --> Challenge response");
                client.challenge_complete = true;
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

}


/// Internal structure used to track a client through login process.
#[derive(Debug)]
pub struct LoginClient {
    addr: SocketAddr,
    blowfish: Option<Arc<Blowfish>>,
    challenge_complete: bool,
}

impl LoginClient {

    #[inline]
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            blowfish: None,
            challenge_complete: false,
        }
    }

}
