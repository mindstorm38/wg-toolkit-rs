use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::rc::Rc;

use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{FromPublicKey, FromPrivateKey}, PublicKeyParts};

use wgtk::net::proxy::{Proxy, ProxyListener, ProxyDirectTransfer, ProxySideOutput};
use wgtk::net::bundle::{Bundle, BundleElement, BundleAssembler};
use wgtk::net::packet::Packet;

use wgtk::net::element::login::{LoginRequestCodec, PingCodec, ChallengeCodec, ChallengeResponseCodec};
use wgtk::net::element::reply::{ReplyCodec};
use wgtk::net::element::{Var16ElementCodec, Var32ElementCodec};


fn main() {

    let server_addr = env::var("WG_SERVER").unwrap().parse().unwrap();
    let client_bind_addr = "0.0.0.0:9788".parse().unwrap();
    let server_bind_addr = "0.0.0.0:9789".parse().unwrap();

    let client_privkey_path = env::var("WG_CLIENT_PRIVKEY_PATH").unwrap();
    let server_pubkey_path = env::var("WG_SERVER_PUBKEY_PATH").unwrap();

    let client_privkey_str = std::fs::read_to_string(client_privkey_path).unwrap();
    let server_pubkey_str = std::fs::read_to_string(server_pubkey_path).unwrap();

    let client_privkey = RsaPrivateKey::from_pkcs8_pem(client_privkey_str.as_str()).unwrap();
    let server_pubkey = RsaPublicKey::from_public_key_pem(server_pubkey_str.as_str()).unwrap();

    println!("client privkey size: {}", client_privkey.size());
    println!("server pubkey size: {}", server_pubkey.size());

    let reply_tracker = RefCell::new(RequestTracker::default());

    let mut login_proxy = Proxy::bind(
        client_bind_addr,
        server_bind_addr,
        server_addr,
        LoginAppClientListener::new(&server_pubkey, &client_privkey, &reply_tracker),
        LoginAppServerListener::new(&reply_tracker)
    ).unwrap();

    loop {
        login_proxy.poll().unwrap();
    }

}


struct LoginAppClientListener<'ek, 'dk, 'rt> {
    asm: BundleAssembler,
    login_codec: LoginRequestCodec<'ek, 'dk>,
    reply_tracker: &'rt RefCell<RequestTracker>
}

impl<'ek, 'dk, 'rt> LoginAppClientListener<'ek, 'dk, 'rt> {
    pub fn new(server_pubkey: &'ek RsaPublicKey, client_privkey: &'dk RsaPrivateKey, reply_tracker: &'rt RefCell<RequestTracker>) -> Self {
        Self {
            asm: BundleAssembler::new(true),
            login_codec: LoginRequestCodec::new_encrypted(server_pubkey, client_privkey),
            reply_tracker
        }
    }
}

impl ProxyListener for LoginAppClientListener<'_, '_, '_> {

    fn received<O: ProxySideOutput>(&mut self, mut packet: Box<Packet>, len: usize, out: &mut O) -> std::io::Result<()> {

        if let Err(e) = packet.sync_state(len) {
            eprintln!("[CLIENT -> SERVER] Failed to sync packet state: {:?}", e);
        } else {
            // println!("[CLIENT -> SERVER] Received packet: {}", wgtk::util::get_hex_str_from(&packet.get_raw_data()[..packet.raw_len()], 1000));
            if let Some(bundle) = self.asm.try_assemble((), packet) {

                assert_eq!(bundle.len(), 1);

                let prefix = bundle.get_packets()[0].get_prefix().unwrap();
                println!("[CLIENT -> SERVER] Received bundle: {:?}", bundle.get_packets());
                println!("[CLIENT -> SERVER] Received packet: {}", wgtk::util::get_hex_str_from(&bundle.get_packets()[0].get_raw_data()[..bundle.get_packets()[0].raw_len()], 1000));

                let mut reader = bundle.get_element_reader();

                while let Some(elt) = reader.next_element() {
                    match elt {
                        BundleElement::Simple(LoginRequestCodec::ID, reader) => {
                            let login = reader.read(&self.login_codec).unwrap();
                            println!("[CLIENT -> SERVER] Received login: {:?}", login.element);
                            let request_id = login.request_id.unwrap();
                            let mut new_bundle = Bundle::new_empty(true);
                            new_bundle.add_request(LoginRequestCodec::ID, &self.login_codec, login.element, request_id);
                            new_bundle.get_packets_mut()[0].set_prefix(Some(prefix));
                            self.reply_tracker.borrow_mut().push_request(RequestSide::Client, request_id, LoginRequestCodec::ID);
                            new_bundle.finalize(&mut 0);
                            out.send_finalized_bundle(&new_bundle).unwrap();
                        }
                        BundleElement::Simple(PingCodec::ID, reader) => {
                            let ping = reader.read(&PingCodec).unwrap();
                            println!("[CLIENT -> SERVER] Received ping try: {}", ping.element);
                            self.reply_tracker.borrow_mut().push_request(RequestSide::Client, ping.request_id.unwrap(), PingCodec::ID);
                            out.send_finalized_bundle(&bundle).unwrap();
                        }
                        BundleElement::Simple(ChallengeResponseCodec::ID, reader) => {
                            let data = reader.read(&Var16ElementCodec::new()).unwrap();
                            println!("[CLIENT -> SERVER] Received challenge response: {}", wgtk::util::str_from_escaped(&data.element[..]));
                        }
                        BundleElement::Simple(id, _) => {
                            panic!("[CLIENT -> SERVER] Received unknown element: {}", id);
                        }
                        _ => {}
                    }
                }

            }
        }

        Ok(())

    }

}


struct LoginAppServerListener<'rt> {
    asm: BundleAssembler,
    reply_tracker: &'rt RefCell<RequestTracker>
}

impl<'rt> LoginAppServerListener<'rt> {
    pub fn new(reply_tracker: &'rt RefCell<RequestTracker>) -> Self {
        Self {
            asm: BundleAssembler::new(true),
            reply_tracker
        }
    }
}

impl ProxyListener for LoginAppServerListener<'_> {

    fn received<O: ProxySideOutput>(&mut self, mut packet: Box<Packet>, len: usize, out: &mut O) -> std::io::Result<()> {

        if let Err(e) = packet.sync_state(len) {
            eprintln!("[SERVER -> CLIENT] Failed to sync packet state: {:?}", e);
        } else {
            println!("[SERVER -> CLIENT] Received packet: {}", wgtk::util::get_hex_str_from(&packet.get_raw_data()[..packet.raw_len()], 1000));
            if let Some(bundle) = self.asm.try_assemble((), packet) {

                assert_eq!(bundle.len(), 1);

                let mut reader = bundle.get_element_reader();

                while let Some(elt) = reader.next_element() {
                    match elt {
                        BundleElement::Reply(request_id, reader) => {
                            println!("[SERVER -> CLIENT] Received reply (ID: {}):", request_id);
                            match self.reply_tracker.borrow_mut().pop_request(RequestSide::Client, request_id) {
                                Some(PingCodec::ID) => {
                                    let ping = reader.read(&PingCodec).unwrap();
                                    println!("[SERVER -> CLIENT] Received ping ack: {}", ping.element);
                                    out.send_finalized_bundle(&bundle).unwrap();
                                }
                                Some(LoginRequestCodec::ID) => {
                                    let challenge = reader.read(&ChallengeCodec).unwrap();
                                    println!("[SERVER -> CLIENT] Challenge: {:?}", challenge.element);
                                    out.send_finalized_bundle(&bundle).unwrap();

                                    /*let login_reply = reader.read(&Var32ElementCodec::new()).unwrap();
                                    let data = &login_reply.element;
                                    println!("[SERVER -> CLIENT] Login reply:");
                                    println!("                   Raw:   {}", wgtk::util::get_hex_str_from(&data[..], 1000));
                                    println!("                   ASCII: {}", wgtk::util::str_from_escaped(&data[..]));*/
                                }
                                _ => eprintln!("[SERVER -> CLIENT] Wrong request ID")
                            }
                        }
                        _ => unreachable!()
                    }
                }

                /*// We expect bundle to have only one element in login app.
                let mut iter = bundle.iter_raw_elements();
                match iter.next_id() {
                    Some(ReplyCodec::<()>::ID) => {

                        // match self.reply_tracker.borrow_mut().pop_request(RequestSide::Client, )

                        /*let reply = iter.next(&ReplyCodec::new(Var32ElementCodec::new())).unwrap();
                        let reply_id = reply.elt.reply_id;
                        let data = reply.elt.element;

                        println!("[SERVER -> CLIENT] Received reply (ID: {}):", reply_id);
                        println!("                   Raw:   {}", wgtk::util::get_hex_str_from(&data[..], 1000));
                        println!("                   ASCII: {}", wgtk::util::str_from_escaped(&data[..]));*/

                    }
                    Some(n) => panic!("[SERVER -> CLIENT] Unsupported element id: {}", n),
                    _ => unreachable!()
                }*/

            }

        }

        Ok(())

    }

}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum RequestSide {
    Client,
    Server
}

#[derive(Debug, Default)]
struct RequestTracker {
    reply_element_ids: HashMap<(RequestSide, u32), u8>
}

impl RequestTracker {

    fn push_request(&mut self, side: RequestSide, request_id: u32, element_id: u8) {
        self.reply_element_ids.insert((side, request_id), element_id);
    }

    fn pop_request(&mut self, side: RequestSide, request_id: u32) -> Option<u8> {
        self.reply_element_ids.remove(&(side, request_id))
    }

}