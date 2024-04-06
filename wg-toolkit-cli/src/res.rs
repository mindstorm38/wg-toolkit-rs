use std::io;

use wgtk::res::ResFilesystem;

use super::{CliResult, ResArgs, ResCommand, ResListArgs, ResReadArgs};


/// Entrypoint.
pub fn cmd_res(args: ResArgs) -> CliResult<()> {

    let fs = ResFilesystem::new(args.dir)
        .map_err(|e| format!("Failed to open resource filesystem, reason: {e}"))?;

    match args.cmd {
        ResCommand::List(args) => cmd_res_ls(args, &fs),
        ResCommand::Read(args) => cmd_res_read(args, &fs),
    }

}

fn cmd_res_ls(args: ResListArgs, fs: &ResFilesystem) -> CliResult<()> {
    
    let path = args.path.as_str();
    let recurse = args.recurse.unwrap_or(Some(0)).unwrap_or(u16::MAX);

    let mut indent = String::new();
    print_dir(fs, &mut indent, path, recurse)
        .map_err(|e| format!("Can't find '{path}' resource directory, reason: {e}"))?;

    Ok(())

}

fn cmd_res_read(args: ResReadArgs, fs: &ResFilesystem) -> CliResult<()> {

    let path = args.path.as_str();

    let mut read_file = fs.read(path)
        .map_err(|e| format!("Can't find '{path}' resource file, reason: {e}"))?;

    io::copy(&mut read_file, &mut io::stdout().lock())
        .map_err(|e| format!("Failed to print file content to stdout, reason: {e}"))?;

    Ok(())

}

/// Print directory content
fn print_dir(fs: &ResFilesystem, indent: &mut String, dir_path: &str, recursion: u16) -> io::Result<()> {

    let mut list = fs.read_dir(dir_path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    list.sort_by(|e1, e2| Ord::cmp(e1.name(), e2.name()));

    for entry in list {

        print!("{indent}{}", entry.name());

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
