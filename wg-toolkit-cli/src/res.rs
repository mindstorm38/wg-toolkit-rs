use std::io;
use std::path::PathBuf;

use clap::ArgMatches;

use wgtk::res::ResFilesystem;

use super::CmdResult;


pub fn cmd_res(matches: &ArgMatches) -> CmdResult<()> {

    let res_dir = matches.get_one::<PathBuf>("res_dir").unwrap();

    let res_fs = ResFilesystem::new(res_dir)
        .map_err(|e| format!("Failed to open resource filesystem, reason: {e}"))?;

    match matches.subcommand() {
        Some(("ls", matches)) => cmd_res_ls(matches, &res_fs),
        Some(("read", matches)) => cmd_res_read(matches, &res_fs),
        _ => unreachable!()
    }

}

fn cmd_res_ls(matches: &ArgMatches, fs: &ResFilesystem) -> CmdResult<()> {

    let path = matches.get_one::<String>("path").unwrap();
    let recurse = matches.get_one::<u16>("recurse").copied().unwrap_or(0);
    
    let mut indent = String::new();
    print_dir(fs, &mut indent, &path, recurse)
        .map_err(|e| format!("Can't find '{path}' resource directory, reason: {e}"))?;

    Ok(())

}

fn cmd_res_read(matches: &ArgMatches, fs: &ResFilesystem) -> CmdResult<()> {

    let path = matches.get_one::<String>("path").unwrap();

    let mut read_file = fs.read(path)
        .map_err(|e| format!("Can't find '{path}' resource file, reason: {e}"))?;

    io::copy(&mut read_file, &mut io::stdout().lock())
        .map_err(|e| format!("Failed to print file content to stdout, reason: {e}"))?;

    Ok(())

}

/// Print directory content
fn print_dir(fs: &ResFilesystem, indent: &mut String, dir_path: &str, recursion: u16) -> io::Result<()> {

    for entry in fs.read_dir(dir_path)? {

        print!("{indent}");

        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                continue;
            }
        };

        print!("{}", entry.name());

        if entry.is_dir() {
            print!("/");
        }

        println!();

        if recursion > 0 {
            indent.push_str("  ");
            let _ = print_dir(fs, indent, &entry.path(), recursion - 1);
            indent.truncate(indent.len() - 2);
        }

    }

    Ok(())

}
