use std::env;

use wgtk::net::proxy::{Proxy, ProxySideFilter};
use wgtk::net::bundle::Bundle;
use wgtk::net::packet::Packet;

use wgtk::net::element::{RawElementFixed, RawElementVariable16};
use wgtk::net::element::login::{LoginElement, PingElement};


fn main() {

    let server_addr = env::var("WG_SERVER").unwrap().parse().unwrap();
    let client_bind_addr = "0.0.0.0:9788".parse().unwrap();
    let server_bind_addr = "0.0.0.0:9789".parse().unwrap();

    let mut login_proxy = Proxy::bind(
        client_bind_addr,
        server_bind_addr,
        server_addr,
        LoginClientFilter,
        ()
    ).unwrap();

    loop {
        login_proxy.poll().unwrap();
    }

}


struct LoginClientFilter;

impl ProxySideFilter for LoginClientFilter {

    fn immediate_transfer(&mut self) -> bool {
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

}
