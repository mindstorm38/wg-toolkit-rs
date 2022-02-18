use std::net::UdpSocket;
use std::env;
use std::io::Write;
use rand::RngCore;

use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{FromPublicKey, FromPrivateKey}, PublicKeyParts, PublicKey, PaddingScheme};
use rand::rngs::OsRng;

fn main() {

    let pubkey_path = env::var("WGT_PUBKEY_PATH").unwrap();
    let privkey_path = env::var("WGT_PRIVKEY_PATH").unwrap();
    let pubkey_content = std::fs::read_to_string(pubkey_path).unwrap();
    let privkey_content = std::fs::read_to_string(privkey_path).unwrap();

    let pubkey = RsaPublicKey::from_public_key_pem(pubkey_content.as_str()).unwrap();
    let privkey = RsaPrivateKey::from_pkcs8_pem(privkey_content.as_str()).unwrap();

    println!("PUB RSA {} {:?}", pubkey.size() * 8, pubkey);
    println!("PRIV RSA {} {:?}", privkey.size() * 8, privkey);

    let mut rng = OsRng;
    let mut clear_data = [0; 256];
    rng.fill_bytes(&mut clear_data);
    let data = pubkey.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &clear_data).unwrap();
    println!("({}) {:?}", data.len(), data);
    // serv(&privkey);

}


fn serv(privkey: &RsaPrivateKey) {

    let mut sock = UdpSocket::bind("127.0.0.1:9788").unwrap();

    let mut buf = [0; 2048];

    loop {

        print!("WAITING... ");
        std::io::stdout().flush().unwrap();
        let (len, addr) = sock.recv_from(&mut buf).unwrap();
        let data = &buf[..len];
        println!("[{:?}] ({}) {:?}", addr, len, data);

        /*let clear_data = privkey.decrypt(PaddingScheme::OAEP {
            digest: Box::new(()),
            mgf_digest: Box::new(()),
            label: None
        }, &data[1..]);
        println!("=> {:?}", clear_data);*/

    }

}
