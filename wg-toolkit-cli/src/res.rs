use clap::ArgMatches;

use wgtk::res::ResFilesystem;

use super::CmdResult;


pub fn cmd_res_ls(matches: &ArgMatches) -> CmdResult<()> {

    let res_dir_path = matches.get_one::<String>("res").unwrap();
    let mut fs = ResFilesystem::new(res_dir_path).unwrap();

    let entries = fs.read_dir("gui/maps").unwrap();
    println!("Entries:");
    for entry in entries {
        let entry = entry.unwrap();
        println!("- {} ({}, dir: {})", entry.name(), entry.path(), entry.is_dir());
    }

    Ok(())

}
