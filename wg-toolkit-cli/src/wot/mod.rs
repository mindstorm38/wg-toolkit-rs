//! Implementation of a simple demonstration WoT server.

pub mod gen;
pub mod proxy;
pub mod emulator;

use std::sync::Arc;
use std::fs;

use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};

use tracing::level_filters::LevelFilter;

use crate::{CliResult, WotArgs};


/// Entrypoint.
pub fn cmd_wot(args: WotArgs) -> CliResult<()> {

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::builder()
            .with_default_directive(LevelFilter::TRACE.into())
            .from_env_lossy())
        .init();

    // Start by decoding the private key...
    let encryption_key;
    if let Some(priv_key_path) = args.priv_key_path.as_deref() {

        let priv_key_content = fs::read_to_string(priv_key_path)
            .map_err(|e| format!("Failed to read private key at {}: {e}", priv_key_path.display()))?;

        encryption_key = Some(Arc::new(RsaPrivateKey::from_pkcs8_pem(&priv_key_content)
            .map_err(|e| format!("Failed to decode PKCS#8 private key: {e}"))?));

    } else {
        encryption_key = None;
    }

    if let Some(real_login_app) = args.real_login_app {

        let real_encryption_key;
        if let Some(pub_key_path) = args.real_pub_key_path.as_deref() {
            
            let pub_key_content = fs::read_to_string(pub_key_path)
                .map_err(|e| format!("Failed to read public key at {}: {e}", pub_key_path.display()))?;

            let pub_key = Arc::new(RsaPublicKey::from_public_key_pem(&pub_key_content)
                .map_err(|e| format!("Failed to decode PEM public key: {e}"))?);

            real_encryption_key = Some(pub_key);

        } else {
            real_encryption_key = None;
        }
        
        proxy::run(args.login_app, real_login_app, args.base_app, encryption_key, real_encryption_key)
        
    } else {
        emulator::run(args.login_app, args.base_app, encryption_key)
    }

}
