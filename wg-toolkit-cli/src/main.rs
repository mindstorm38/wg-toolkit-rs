//! The CLI for wg-toolkit
//! 
//! Use cases:
//! $ wgtk pxml show <FILE> [-p <PATH>]
//! $ wgtk pxml edit <FILE> <PATH> <VALUE>

use clap::{Command, arg, ArgMatches};

mod pxml;


fn main() {

    let matches = Command::new("wgtk")
        .version("0.1.0")
        .author("Théo Rozier <contact@theorozier.fr>")
        .about("WG Toolkit is a command line utility for codecs distributed by Wargaming.net")
        .disable_help_subcommand(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("pxml")
            .about("Packed XML read and write utilities")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand(Command::new("show")
                .about("Show a deserialized view of a given Packed XML file")
                .arg(arg!(-p --path <PATH> "Path to a specific value to show"))
                .arg(arg!(file: <FILE> "The Packed XML file to show")))
            .subcommand(Command::new("edit")
                .about("Edit a terminal value of a given Packed XML file")
                .arg(arg!(file: <FILE> "The Packed XML file to edit"))
                .arg(arg!(path: <PATH> "The path to the terminal value to edit"))
                .arg(arg!(value: <VALUE> "The new value"))))
        .get_matches();

    match matches.subcommand() {
        Some(("pxml", matches)) => cmd_pxml(matches),
        _ => unreachable!()
    }
    
}


fn cmd_pxml(matches: &ArgMatches) {

    match matches.subcommand() {
        Some(("show", matches)) => pxml::cmd_pxml_show(matches),
        Some(("edit", matches)) => pxml::cmd_pxml_edit(matches),
        _ => {}
    }

}
