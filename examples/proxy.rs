use wgtk::net::proxy::Proxy;
use std::env;

fn main() {

    let server_addr = env::var("WG_SERVER").unwrap().parse().unwrap();
    let client_bind_addr = "0.0.0.0:9788".parse().unwrap();
    let server_bind_addr = "0.0.0.0:9789".parse().unwrap();

    let mut proxy = Proxy::bind(client_bind_addr, server_bind_addr, server_addr).unwrap();

    loop {
        proxy.poll().unwrap();
    }

}
