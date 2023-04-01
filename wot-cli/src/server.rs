//! The World of Tanks client CLI.
//! 
//! It provides different tools for launching a server, containing login, base and cell
//! applications.

use std::net::SocketAddrV4;
use std::process::ExitCode;
use std::sync::Arc;
use std::fs;
use std::time::Duration;

use clap::{Command, ArgMatches, arg, crate_version, crate_authors};

use rsa::pkcs8::DecodePrivateKey;
use rsa::RsaPrivateKey;

mod common;
mod login;
mod base;

use login::LoginApp;
use base::BaseApp;
use common::server_settings::ServerSettings;


/// The game version this server support by default.
const GAME_VERSION: &str = "eu_1.19.1_4";


fn main() -> ExitCode {

    let matches = Command::new("wots")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Command line utility made for serving a World of Tanks server")
        .disable_help_subcommand(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("simple")
            .about("Start a simple World of Tanks server, composed of a single login and base applications")
            .arg(arg!(login_app_bind: --loginapp <BIND> "The address to bind the loginapp server.").default_value("127.0.0.1:20016"))
            .arg(arg!(base_app_bind: --baseapp <BIND> "The address to bind the loginapp server.").default_value("127.0.0.1:20017"))
            .arg(arg!(priv_key_path: --privkey <PATH> "The path to the private key, used for loginapp encryption. Encryption is disabled if not provided."))
            .arg(arg!(game_version: --version <ID> "The version identifier sent to the client and checked by it.").default_value(GAME_VERSION)))
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

    let required_version = matches.get_one::<String>("game_version").unwrap();

    // let server_settings_bytes = &include_bytes!("../../test.txt")[..41363];
    // let server_settings: Box<ServerSettings> = serde_pickle::from_slice(server_settings_bytes, serde_pickle::DeOptions::new().decode_strings()).unwrap();
    let server_settings = Box::new(ServerSettings::default());

    let mut login_app = LoginApp::new(login_app_bind, priv_key)
        .map_err(|e| format!("Failed to bind the loginapp: {e}"))?;

    let mut base_app = BaseApp::new(base_app_bind, server_settings)
        .map_err(|e| format!("Failed to bind the baseapp: {e}"))?;

    base_app.set_required_version(required_version);
    println!("[ALL] Game version: {required_version}");

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
