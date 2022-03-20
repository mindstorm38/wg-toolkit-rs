use std::env;

use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{FromPublicKey, FromPrivateKey}};

use wgtk::net::proxy::{Proxy, ProxyListener, ProxyDirectTransfer, ProxySideOutput};
use wgtk::net::bundle::{Bundle, BundleAssembler};
use wgtk::net::packet::Packet;

use wgtk::net::element::login::{LoginCodec, PingCodec};


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
        LoginAppClientListener::new(&server_pubkey, &client_privkey),
        LoginAppServerListener
    ).unwrap();

    loop {
        login_proxy.poll().unwrap();
    }

}


struct LoginAppClientListener<'ek, 'dk> {
    asm: BundleAssembler,
    login_codec: LoginCodec<'ek, 'dk>
}

impl<'ek, 'dk> LoginAppClientListener<'ek, 'dk> {
    pub fn new(server_pubkey: &'ek RsaPublicKey, client_privkey: &'dk RsaPrivateKey) -> Self {
        Self {
            asm: BundleAssembler::new(true),
            login_codec: LoginCodec::new_encrypted(server_pubkey, client_privkey)
        }
    }
}

impl ProxyListener for LoginAppClientListener<'_, '_> {

    fn received<O: ProxySideOutput>(&mut self, mut packet: Box<Packet>, len: usize, out: &mut O) -> std::io::Result<()> {

        if let Err(e) = packet.sync_state(len) {
            eprintln!("[CLIENT -> SERVER] Failed to sync packet state: {:?}", e);
        } else {
            if let Some(bundle) = self.asm.try_assemble((), packet) {

                assert_eq!(bundle.len(), 1);

                let prefix = bundle.get_packets()[0].get_prefix().unwrap();
                println!("[CLIENT -> SERVER] Prefix: {prefix}");

                // We expect bundle to have only one element in login app.
                let mut iter = bundle.iter_raw_elements();
                match iter.next_id() {
                    Some(LoginCodec::ID) => {

                        let login = iter.next(&self.login_codec).unwrap();
                        println!("[CLIENT -> SERVER] Received login: {:?}", login.elt);

                        assert!(login.reply_id.is_some());

                        let mut new_bundle = Bundle::new_empty(true);
                        new_bundle.add_element(LoginCodec::ID, &self.login_codec, login.elt, true);

                        let mut reply_id = login.reply_id.unwrap();
                        new_bundle.finalize(&mut 0, &mut reply_id);

                        assert_eq!(new_bundle.len(), 1);
                        new_bundle.get_packets_mut()[0].set_prefix(Some(prefix));
                        println!("- {:?}", new_bundle.get_packets()[0]);

                        out.send_finalized_bundle(&new_bundle).unwrap();

                    }
                    Some(PingCodec::ID) => {
                        let ping = iter.next(&PingCodec).unwrap().elt;
                        println!("[CLIENT -> SERVER] Received ping try: {}", ping);
                        out.send_finalized_bundle(&bundle).unwrap();
                    }
                    _ => {}
                }

            }
        }

        Ok(())

    }

}


struct LoginAppServerListener;

impl ProxyListener for LoginAppServerListener {

    fn received<O: ProxySideOutput>(&mut self, packet: Box<Packet>, len: usize, out: &mut O) -> std::io::Result<()> {
        let data = &packet.get_raw_data()[..len];
        println!("[SERVER -> CLIENT] Transfer: {}", wgtk::util::get_hex_str_from(data, 100));
        out.send_data(data)
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
