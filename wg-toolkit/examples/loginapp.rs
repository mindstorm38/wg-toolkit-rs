use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use std::env;
use std::fs;

use rand::rngs::OsRng;
use rand::RngCore;

use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{DecodePrivateKey, DecodePublicKey}};

use crypto_common::KeyInit;
use blowfish::Blowfish;

use wgtk::net::bundle::{BundleElement, Bundle};
use wgtk::net::app::{App, EventKind};

use wgtk::net::element::login::{PingCodec, LoginCodec, LoginResponseCodec, LoginResponse, LoginChallenge, LoginChallenge};


fn main() {

    let pubkey_path = env::var("WGT_PUBKEY_PATH").unwrap();
    let privkey_path = env::var("WGT_PRIVKEY_PATH").unwrap();
    let pubkey_content = fs::read_to_string(pubkey_path).unwrap();
    let privkey_content = fs::read_to_string(privkey_path).unwrap();

    let pubkey = Arc::new(RsaPublicKey::from_public_key_pem(pubkey_content.as_str()).unwrap());
    let privkey = Arc::new(RsaPrivateKey::from_pkcs8_pem(privkey_content.as_str()).unwrap());

    let login_codec = LoginCodec::Server(privkey);
    let mut blowfish_map = HashMap::new();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 20016);
    let mut app = App::new(addr, true).unwrap();
    let mut events = Vec::new();

    loop {

        app.poll(&mut events, None).unwrap();

        for event in &events {

            print!("[{}] ", event.addr);

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
                            BundleElement::Simple(LoginCodec::ID, reader) => {

                                let elt = reader.read(&login_codec).unwrap();
                                println!("- Login {:?} -> #{}", elt.element, elt.request_id.unwrap());

                                // Ensure that the blowfish key is existing.
                                let bf = match blowfish_map.entry(event.addr) {
                                    Entry::Occupied(o) => o.into_mut(), // TODO: Check coherence with login params
                                    Entry::Vacant(v) => {
                                        v.insert(Arc::new(Blowfish::new_from_slice(&elt.element.blowfish_key).unwrap()))
                                    }
                                };

                                let prefix_value = OsRng.next_u64();
                                let prefix = format!("{prefix_value:>02X}");
                                let easiness = 0.5;

                                let challenge = LoginResponse::Challenge(LoginChallenge::CuckooCycle { 
                                    prefix, 
                                    max_nonce: ((1 << 20) as f32 * easiness) as _
                                });

                                let mut bundle = Bundle::new_empty(true);
                                bundle.add_reply(&LoginResponseCodec::Encrypted(bf.clone()), challenge, elt.request_id.unwrap());
                                app.send(&mut bundle, event.addr).unwrap();

                            }
                            BundleElement::Simple(, )
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
