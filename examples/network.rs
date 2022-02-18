use std::net::UdpSocket;
use std::env;
use std::io::Write;

use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{FromPublicKey, FromPrivateKey}, PublicKeyParts, PaddingScheme};
use sha1::Sha1;


// PING PACKET FORMAT:
// (016) [?, ?, ?, ?, ?, 0, 2, ?, 106, 0, 0, 0, 0, retry_index, ?, ?]

// LOGIN PACKET FORMAT:
// (???) [?*20, data*256*n, 2, 0]
// (???)

// STRING FORMAT:
// [len, data*len]

fn main() {

    let pubkey_path = env::var("WGT_PUBKEY_PATH").unwrap();
    let privkey_path = env::var("WGT_PRIVKEY_PATH").unwrap();
    let pubkey_content = std::fs::read_to_string(pubkey_path).unwrap();
    let privkey_content = std::fs::read_to_string(privkey_path).unwrap();

    let pubkey = RsaPublicKey::from_public_key_pem(pubkey_content.as_str()).unwrap();
    let privkey = RsaPrivateKey::from_pkcs8_pem(privkey_content.as_str()).unwrap();

    println!("PUB RSA {} {:?}", pubkey.size() * 8, pubkey);
    println!("PRIV RSA {} {:?}", privkey.size() * 8, privkey);

    /*let mut rng = OsRng;
    let mut clear_data = [0; 256];
    rng.fill_bytes(&mut clear_data);
    let data = pubkey.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &clear_data).unwrap();
    println!("({}) {:?}", data.len(), data);*/
    serv(&privkey);

}


fn serv(privkey: &RsaPrivateKey) {

    let sock = UdpSocket::bind("127.0.0.1:9788").unwrap();

    let mut buf = [0; 2048];

    loop {

        std::io::stdout().flush().unwrap();
        let (len, addr) = sock.recv_from(&mut buf).unwrap();
        let data = &buf[..len];

        println!("[{:?}]", addr);
        println!("<R> ({:03}) {:?}", len, data);

        if len > 20 {

            let cipher_data_all = &data[20..];
            let chunk_size = privkey.size();
            let chunk_count = cipher_data_all.len() / chunk_size;
            let cipher_size = chunk_size * chunk_count;
            let footer_data = &cipher_data_all[cipher_size..];
            let footer_size = footer_data.len();

            for i in 0..chunk_count {

                let cipher_data = &cipher_data_all[(i * chunk_size)..(i * chunk_size + chunk_size)];
                println!("<{}> ({:03}) {:?}", i, cipher_data.len(), cipher_data);

                let scheme = PaddingScheme::new_oaep::<Sha1>();
                let clear_data = privkey.decrypt(scheme, cipher_data);

                if let Ok(clear_data) = clear_data {

                    println!("    ({:03}) {:?}", clear_data.len(), clear_data);
                    let clear_data_str = clear_data.iter()
                        .copied()
                        .flat_map(std::ascii::escape_default)
                        .collect();
                    let clear_data_str = String::from_utf8(clear_data_str).unwrap();

                    println!("          {}", clear_data_str);

                }

            }

            println!("<F> ({:03}) {:?}", footer_size, footer_data);

        }

        /*let clear_data = privkey.decrypt(PaddingScheme::OAEP {
            digest: Box::new(()),
            mgf_digest: Box::new(()),
            label: None
        }, &data[1..]);
        println!("=> {:?}", clear_data);*/

    }

}
