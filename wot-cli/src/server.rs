//! The World of Tanks client CLI.
//! 
//! It provides different tools for launching a server, containing login, base and cell
//! applications.

use std::net::SocketAddrV4;
use std::process::ExitCode;
use std::sync::Arc;
use std::fs;
use std::time::Duration;

use clap::{Command, ArgMatches, arg, crate_version, crate_authors, crate_description};

use rsa::pkcs8::DecodePrivateKey;
use rsa::RsaPrivateKey;

mod common;
mod login;
mod base;

use login::LoginApp;
use base::BaseApp;


fn main() -> ExitCode {

    let matches = Command::new("wots")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .disable_help_subcommand(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("simple")
            .about("Start a simple World of Tanks server, composed of a single login and base applications")
            .arg(arg!(login_app_bind: --loginapp <BIND> "The address to bind the loginapp server.").default_value("127.0.0.1:20016"))
            .arg(arg!(base_app_bind: --baseapp <BIND> "The address to bind the loginapp server.").default_value("127.0.0.1:20017"))
            .arg(arg!(priv_key_path: --privkey <PATH> "The path to the private key, used for loginapp encryption. Encryption is disabled if not provided.")))
        .get_matches();

    let res = match matches.subcommand() {
        Some(("simple", matches)) => cmd_simple(matches),
        _ => unreachable!()
    };

    if let Err(message) = res {
        eprintln!("{message}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
    
}
 

fn cmd_simple(matches: &ArgMatches) -> CmdResult<()> {

    let login_app_bind = matches.get_one::<String>("login_app_bind")
        .unwrap()
        .parse::<SocketAddrV4>()
        .map_err(|e| format!("Failed to parse loginapp bind address: {e}"))?;

    let base_app_bind = matches.get_one::<String>("base_app_bind")
        .unwrap()
        .parse::<SocketAddrV4>()
        .map_err(|e| format!("Failed to parse baseapp bind address: {e}"))?;

    let priv_key = match matches.get_one::<String>("priv_key_path") {
        Some(priv_key_path) => {

            let priv_key_content = fs::read_to_string(priv_key_path)
                .map_err(|e| format!("Failed to read private key at {priv_key_path}: {e}"))?;

            let priv_key = RsaPrivateKey::from_pkcs8_pem(&priv_key_content)
                .map_err(|e| format!("Failed to decode PKCS#8 private key: {e}"))?;

            Some(Arc::new(priv_key))

        }
        _ => None,
    };
    

    let mut login_app = LoginApp::new(login_app_bind, priv_key)
        .map_err(|e| format!("Failed to bind the loginapp: {e}"))?;

    let mut base_app = BaseApp::new(base_app_bind)
        .map_err(|e| format!("Failed to bind the baseapp: {e}"))?;

    println!("[LOGIN] Running on {:?}", login_app.app.addr());
    println!("[BASE] Running on {:?}", base_app.app.addr());

    let mut events = Vec::new();

    loop {
        
        login_app.app.poll(&mut events, Some(Duration::from_millis(10))).unwrap();
        for event in &events {
            login_app.handle(&event, &mut base_app);
        }

        base_app.app.poll(&mut events, Some(Duration::from_millis(10))).unwrap();
        for event in &events {
            base_app.handle(event);
        }

    }

}

type CmdResult<T> = Result<T, String>;
