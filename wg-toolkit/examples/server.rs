use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use std::env;
use std::fs;

use rand::rngs::OsRng;
use rand::RngCore;

use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey};

use crypto_common::KeyInit;
use blowfish::Blowfish;

use wgtk::net::bundle::{BundleElement, Bundle};
use wgtk::net::app::{App, EventKind};

use wgtk::net::element::ping::PingCodec;
use wgtk::net::element::login::{
    LoginRequestCodec, 
    LoginResponseCodec, LoginResponse, LoginChallenge, 
    ChallengeResponseCodec, CuckooCycleResponseCodec, LoginSuccess
};


fn main() {

    let privkey_path = env::var("WGT_PRIVKEY_PATH").unwrap();
    let privkey_content = fs::read_to_string(privkey_path).unwrap();
    let privkey = Arc::new(RsaPrivateKey::from_pkcs8_pem(privkey_content.as_str()).unwrap());

    let login_codec = LoginRequestCodec::Server(privkey);
    let mut clients = HashMap::new();

    let mut app = App::new("0.0.0.0:20016".parse().unwrap(), true).unwrap();
    let mut events = Vec::new();

    loop {

        app.poll(&mut events, None).unwrap();

        for event in &events {

            print!("[{}] ", event.addr);

            let client = match clients.entry(event.addr) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(Client::default()),
            };

            match &event.kind {
                EventKind::Bundle(bundle) => {

                    println!("Received bundle ({}):", bundle.len());

                    let mut reader = bundle.get_element_reader();

                    while let Some(elt) = reader.next_element() {
                        match elt {
                            BundleElement::Simple(PingCodec::ID, reader) => {

                                let elt = reader.read(&PingCodec).unwrap();
                                println!("- Ping #{} -> #{}", elt.element, elt.request_id.unwrap());

                                let mut bundle = Bundle::new_empty(true);
                                bundle.add_reply(&PingCodec, elt.element, elt.request_id.unwrap());
                                app.send(&mut bundle, event.addr).unwrap();

                            }
                            BundleElement::Simple(LoginRequestCodec::ID, reader) => {

                                let elt = reader.read(&login_codec).unwrap();
                                println!("- Login {:?} -> #{:?}", elt.element, elt.request_id);

                                let mut bundle = Bundle::new_empty(true);

                                if !client.challenge_complete {
                                    
                                    let bf = client.blowfish.insert(Arc::new(Blowfish::new_from_slice(&elt.element.blowfish_key).unwrap()));

                                    let prefix_value = OsRng.next_u64();
                                    let prefix = format!("{prefix_value:>02X}");
                                    let easiness = 0.9;

                                    let challenge = LoginResponse::Challenge(LoginChallenge::CuckooCycle { 
                                        prefix, 
                                        max_nonce: ((1 << 20) as f32 * easiness) as _
                                    });

                                    bundle.add_reply(&LoginResponseCodec::Encrypted(bf.clone()), challenge, elt.request_id.unwrap());

                                } else {
                                    
                                    let bf = client.blowfish.as_ref().unwrap();

                                    let success = LoginResponse::Success(LoginSuccess {
                                        addr: "127.0.0.1:20017".parse().unwrap(),
                                        session_key: 12345678,
                                        server_message: String::new(),
                                    });

                                    // let success = LoginResponse::Error(LoginError::BaseAppOverload, "hey".to_string());

                                    bundle.add_reply(&LoginResponseCodec::Encrypted(bf.clone()), success, elt.request_id.unwrap());
                                    
                                }

                                app.send(&mut bundle, event.addr).unwrap();

                            }
                            BundleElement::Simple(ChallengeResponseCodec::ID, reader) => {
                                
                                let elt = reader.read(&ChallengeResponseCodec::new(CuckooCycleResponseCodec)).unwrap();
                                println!("- Challenge {:?}", elt.element);
                                client.challenge_complete = true;
                                
                            }
                            BundleElement::Simple(id, _) => {
                                println!("- Unknown simple #{id}");
                                break
                            }
                            BundleElement::Reply(id, _) => {
                                println!("- Unknown reply to #{id}")
                            }
                        }
                    }

                }
                EventKind::InvalidPacket { error, .. } => {
                    println!("Invalid packet: {error:?}");
                }
                EventKind::DiscardedPacket(_) => {
                    println!("Discarded packet");
                }
            }

        }

    }

}


/// Internal structure used to track a client through login process.
#[derive(Debug, Default)]
struct Client {
    blowfish: Option<Arc<Blowfish>>,
    challenge_complete: bool,
}
