//! Implementation of a simple demonstration WoT server.

use std::net::SocketAddr;
use std::{fs, thread};
use std::sync::Arc;

use rsa::pkcs8::DecodePrivateKey;
use rsa::RsaPrivateKey;

use wgtk::net::app::{login, base};

use crate::{CliResult, WotArgs};


/// Entrypoint.
pub fn cmd_wot(args: WotArgs) -> CliResult<()> {

    let mut login_app = login::App::new(SocketAddr::V4(args.login_app))
        .map_err(|e| format!("Failed to bind login app: {e}"))?;

    let base_app = base::App::new(SocketAddr::V4(args.base_app))
        .map_err(|e| format!("Failed to bind base app: {e}"))?;

    if let Some(priv_key_path) = args.priv_key_path.as_deref() {

        let priv_key_content = fs::read_to_string(priv_key_path)
            .map_err(|e| format!("Failed to read private key at {}: {e}", priv_key_path.display()))?;

        let priv_key = Arc::new(RsaPrivateKey::from_pkcs8_pem(&priv_key_content)
            .map_err(|e| format!("Failed to decode PKCS#8 private key: {e}"))?);

        login_app.set_private_key(priv_key);

    }

    thread::scope(move |scope| {
        scope.spawn(move || run_login_app(login_app));
        scope.spawn(move || run_base_app(base_app));
    });

    Ok(())

}

fn run_login_app(mut app: login::App) {

    println!("[L] Running on: {}", app.addr());

    loop {
        let event = app.poll();
        println!("[L] Event: {event:?}");
    }

}

fn run_base_app(mut app: base::App) {

    println!("[B] Running on: {}", app.addr());

    loop {
        let event = app.poll();
        println!("[B] Event: {event:?}");
    }

}
