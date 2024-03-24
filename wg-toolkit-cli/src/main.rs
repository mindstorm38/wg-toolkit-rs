//! The CLI for wg-toolkit
//! 
//! Use cases:
//! $ wgtk pxml show <FILE> [-p <PATH>]
//! $ wgtk pxml edit <FILE> <PATH> <VALUE>

use std::process::ExitCode;

use clap::{Command, ArgMatches, arg, crate_version, crate_authors, crate_description};

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
            .subcommand(Command::new("ls")
                .about("List directory contents")
                .arg(arg!(res: <RES> "Path to the game's res/ directory")))
            .subcommand(Command::new("cat")
                .about("Concatenate files and print on the standard output")
                .arg(arg!(res: <RES> "Path to the game's res/ directory"))))
        .get_matches();

    let res = match matches.subcommand() {
        Some(("pxml", matches)) => cmd_pxml(matches),
        Some(("res", matches)) => cmd_res(matches),
        _ => unreachable!()
    };

    if let Err(message) = res {
        eprintln!("{message}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
    
}


fn cmd_pxml(matches: &ArgMatches) -> CmdResult<()> {
    match matches.subcommand() {
        Some(("show", matches)) => pxml::cmd_pxml_show(matches),
        Some(("edit", matches)) => pxml::cmd_pxml_edit(matches),
        _ => unreachable!()
    }
}

fn cmd_res(matches: &ArgMatches) -> CmdResult<()> {
    match matches.subcommand() {
        Some(("ls", matches)) => res::cmd_res_ls(matches),
        Some(("cat", matches)) => res::cmd_res_cat(matches),
        _ => unreachable!()
    }
}

type CmdResult<T> = Result<T, String>;
