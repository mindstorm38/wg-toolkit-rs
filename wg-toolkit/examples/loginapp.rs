use std::io::{Cursor, Read};
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

use wgtk::net::filter::BlockReader;
use wgtk::net::element::FixedElementCodec;
use wgtk::net::filter::blowfish::BlowfishFilter;
use wgtk::net::packet::Packet;


fn main() {

    // FROM INJECTED: 5cdf18f1802c0000b7d4a7c4987b22697344656d6f4163636f756e74223a66616c73652c226e616d65223a224d696e6473746f726d33385f222c2273656375726974795f6d7367223a226f6c645f70617373222c22746f6b656e32223a223531383835383130353a383939343636333534363338343637343333383a323434373437323736353430323431353434333037383833343930363435383033343132373539227d000000
 
    let bf = Blowfish::new_from_slice(&[0x90, 0x8c, 0x85, 0xbe, 0x34, 0xe2, 0x2e, 0x7f, 0xa6, 0x42, 0x56, 0xd5, 0x07, 0x91, 0xba, 0x9d]).unwrap();

    let packet_data = b"\x49\x4c\x5f\xa8\x00\x00\xff\xad\x00\x00\x00\x4f\x92\x00\x00\x01\xd6\x81\x15\x0f\x7a\xe4\x1c\x75\xec\xe8\x80\xc7\x4a\x13\x52\x0c\xa8\x1b\x4f\x13\xe9\xab\x4d\x1d\xf2\x3c\x6e\x87\xff\xe2\x22\x06\xf7\x99\x9d\x25\x8c\xe4\xc7\x35\x60\xa6\x6f\xa0\x6e\x1a\x24\x59\x90\x77\x39\x3d\xd9\xb4\x0c\xa6\x21\x38\x07\xdc\x3c\xf4\x27\xb5\xfb\x58\x85\xc2\x8b\x4c\x0e\x9a\xf7\xfe\x4f\xca\xf7\xf8\x69\x29\xb8\x05\x1f\x6c\x2b\xd1\x14\xc8\xd9\x98\x09\x88\xd8\x39\x9c\x2e\x1c\x79\x83\xf3\xe9\xaf\x36\x85\x96\x51\x97\xb0\x8c\xf1\xc6\x65\x79\x5a\x00\x5c\x66\x32\xa6\x86\x81\x40\xd1\x5a\x27\xc3\xb9\x9a\xc8\x6a\x87\xc8\x8a\x75\xf5\x66\xbf\xbf\xfb\x6e\x84\x87\xf2\xa3\x44\x21\x53\x8f\xaa\xbc\xe0\x05\x64\xc0\x79\x90\xab\xa9\x0b\x8c\x2e\x96\xd0\x39\x8c\xbe\x22\x6d";
    let mut packet = Packet::new_boxed(true);
    packet.get_raw_data_mut()[..packet_data.len()].copy_from_slice(&packet_data[..]);
    packet.sync_state(packet_data.len());
    let bundle = Bundle::from_single(packet, true);
    let mut reader = bundle.get_element_reader();
    if let BundleElement::Reply(id, reader) = reader.next_element().unwrap() {
        let data = reader.read(&FixedElementCodec::<0>::new()).unwrap();
        let filter = BlowfishFilter::new(&bf);
        let mut filter_reader = BlockReader::new(Cursor::new(&data.element[1..]), filter);
        let mut buf = Vec::new();
        filter_reader.read_to_end(&mut buf).unwrap();
        println!("({}) {}", buf.len(), wgtk::util::get_hex_str_from(&buf, 200));
    }

    // let pubkey_path = env::var("WGT_PUBKEY_PATH").unwrap();
    let privkey_path = env::var("WGT_PRIVKEY_PATH").unwrap();
    // let pubkey_content = fs::read_to_string(pubkey_path).unwrap();
    let privkey_content = fs::read_to_string(privkey_path).unwrap();

    // let pubkey = Arc::new(RsaPublicKey::from_public_key_pem(pubkey_content.as_str()).unwrap());
    let privkey = Arc::new(RsaPrivateKey::from_pkcs8_pem(privkey_content.as_str()).unwrap());

    let login_codec = LoginRequestCodec::Server(privkey);
    let mut clients = HashMap::new();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 20016);
    let mut app = App::new(addr, true).unwrap();
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
                                    let easiness = 0.5;

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
