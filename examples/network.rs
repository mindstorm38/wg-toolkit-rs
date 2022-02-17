use rsa::{RsaPublicKey, pkcs8::FromPublicKey, PublicKeyParts, PublicKey};
use std::env;


fn main() {

    let pubkey_path = env::var("WGT_PUBKEY_PATH").unwrap();
    let pubkey_content = std::fs::read_to_string(pubkey_path).unwrap();

    let pubkey = RsaPublicKey::from_public_key_pem(pubkey_content.as_str()).unwrap();

    println!("RSA{} {:#?}", pubkey.size(), pubkey);

}
