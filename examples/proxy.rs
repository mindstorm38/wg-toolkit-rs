use std::env;

use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{FromPublicKey, FromPrivateKey}};

use wgtk::net::proxy::{Proxy, ProxyListener, ProxyDirectTransfer, ProxySideOutput};
use wgtk::net::bundle::{Bundle, BundleAssembler};
use wgtk::net::packet::Packet;

use wgtk::net::element::{RawElementFixed, RawElementVariable16};
use wgtk::net::element::login::{LoginElement, PingElement};


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

    let mut login_proxy = Proxy::bind(
        client_bind_addr,
        server_bind_addr,
        server_addr,
        LoginAppClientListener::new(client_privkey),
        ProxyDirectTransfer
    ).unwrap();

    loop {
        login_proxy.poll().unwrap();
    }

}


struct LoginAppClientListener {
    asm: BundleAssembler,
    client_privkey: RsaPrivateKey,
}

impl LoginAppClientListener {
    pub fn new(client_privkey: RsaPrivateKey) -> Self {
        Self {
            asm: BundleAssembler::new(true),
            client_privkey
        }
    }
}

impl ProxyListener for LoginAppClientListener {

    fn received<O: ProxySideOutput>(&mut self, mut packet: Box<Packet>, len: usize, out: &mut O) -> std::io::Result<()> {

        if let Err(e) = packet.sync_state(len, true) {
            eprintln!("[CLIENT -> SERVER] Failed to sync packet state: {:?}", e);
        } else {
            if let Some(bundle) = self.asm.try_assemble((), packet) {

                // We expect bundle to have only one element in login app.
                let mut iter = bundle.iter_raw_elements();
                match iter.next_id() {
                    Some(LoginElement::ID) => {
                        let login = iter.next_with_cfg(LoginElement::default(), &self.client_privkey).unwrap();
                        println!("[CLIENT -> SERVER] Received login: {:?}", login.elt);
                    }
                    Some(PingElement::ID) => {
                        let ping = iter.next(PingElement::default()).unwrap();
                        println!("[CLIENT -> SERVER] Received ping: {:?}", ping.elt);
                        out.send_finalized_bundle(&bundle).unwrap();
                    }
                    _ => {}
                }

            }
        }

        Ok(())

    }

}


/*struct LoginClientFilter;

impl ProxyFilter for LoginClientFilter {

    fn received_data(&mut self) -> bool {
        true
    }

    fn received_packet(&mut self, _packet: &Packet) {

    }

    fn received_bundle(&mut self, bundle: &Bundle) {

        let mut iter = bundle.iter_raw_elements();
        loop {
            match iter.next_id() {
                Some(LoginElement::ID) => {
                    let login = iter.next::<RawElementVariable16>().unwrap();
                    println!("login: ({}) {:?}", login.elt.0.len(), login.elt.0);
                },
                Some(PingElement::ID) => {
                    let ping = iter.next::<PingElement>().unwrap();
                    println!("Received ping: {:?}", ping.elt);
                }
                _ => break
            }
        }

    }

}*/
