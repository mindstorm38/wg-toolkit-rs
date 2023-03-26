//! The World of Tanks client CLI.
//! 
//! It's currently not ready but will ultimately provide an interactive user interface
//! to interact with the game just from CLI.

use std::process::ExitCode;

use clap::{Command, crate_version, crate_authors};

mod common;


fn main() -> ExitCode {

    let _matches = Command::new("wotc")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Command line utility for emulating a World of Tanks client")
        .disable_help_subcommand(true)
        .get_matches();

    ExitCode::SUCCESS
    
}
