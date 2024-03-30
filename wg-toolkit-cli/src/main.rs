//! The CLI for wg-toolkit
//! 
//! Use cases:
//! $ wgtk pxml show <FILE> [-p <PATH>]
//! $ wgtk pxml edit <FILE> <PATH> <VALUE>
//! $ wgth res <PATH> ls
//! $ wgth res <PATH> read

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{arg, crate_authors, crate_description, crate_version, value_parser, Command};

mod pxml;
mod res;


fn main() -> ExitCode {

    let matches = Command::new("wgtk")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .disable_help_subcommand(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("pxml")
            .about("Packed XML read and write utilities")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand(Command::new("show")
                .about("Show a deserialized view of a given Packed XML file")
                .arg(arg!(path: -p --path <PATH> "Path to a specific value to show"))
                .arg(arg!(xml: -x --xml "Enable XML output style"))
                .arg(arg!(file: <FILE> "The Packed XML file to show")))
            .subcommand(Command::new("edit")
                .about("Edit a terminal value of a given Packed XML file")
                .arg(arg!(file: <FILE> "The Packed XML file to edit"))
                .arg(arg!(path: <PATH> "The path to the terminal value to edit"))
                .arg(arg!(value: <VALUE> "The new value"))))
        .subcommand(Command::new("res")
            .about("Resources flatten filesystem utilities")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .arg(arg!(res_dir: <PATH> "Path to the game's res/ directory")
                .value_parser(value_parser!(PathBuf)))
            .subcommand(Command::new("ls")
                .about("List directory contents")
                .arg(arg!(path: <PATH> "Path to the directory to list, no leading separator!"))
                .arg(arg!(recurse: -r --recurse [RECURSION] "Enable recursion listing of directories.")
                    .value_parser(value_parser!(u16))
                    .default_missing_value("10000")))
            .subcommand(Command::new("read")
                .about("Read a file and write its content on the standard output")
                .arg(arg!(path: <PATH> "Path to the file to read, no leading separator!"))))
        .get_matches();

    let res = match matches.subcommand() {
        Some(("pxml", matches)) => pxml::cmd_pxml(matches),
        Some(("res", matches)) => res::cmd_res(matches),
        _ => unreachable!()
    };

    if let Err(message) = res {
        eprintln!("{message}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
    
}

type CmdResult<T> = Result<T, String>;
