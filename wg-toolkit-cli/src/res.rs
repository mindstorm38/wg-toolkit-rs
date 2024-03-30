use std::time::Instant;

use clap::ArgMatches;

use wgtk::res::ResFilesystem;

use super::CmdResult;


pub fn cmd_res_ls(matches: &ArgMatches) -> CmdResult<()> {

    let res_dir_path = matches.get_one::<String>("res").unwrap();
    let fs = ResFilesystem::new(res_dir_path).unwrap();

    let start = Instant::now();

    let entries = fs.read_dir("gui/maps/").unwrap();
    println!("Entries in 'gui/maps/':");
    for entry in entries {
        let entry = entry.unwrap();
        println!("{}", entry.file_name());
    }

    println!("Completed in {:?}", start.elapsed());

    let start = Instant::now();
    let reader = fs.read("vehicles/french/F72_AMX_30/AMX_30_hull_01_AM.dds").unwrap();
    println!("{reader:?}, completed in {:?}", start.elapsed());

    Ok(())

}


pub fn cmd_res_cat(matches: &ArgMatches) -> CmdResult<()> {

    let res_dir_path = matches.get_one::<String>("res").unwrap();
    let fs = ResFilesystem::new(res_dir_path).unwrap();

    println!("{:?}", fs.read("vehicles/french/F72_AMX_30/AMX_30_hull_01_AM.dds"));
    println!("{:?}", fs.read("gui/flash/fontconfig.xml"));

    // let entries = fs.read_dir("gui/maps").unwrap();
    // println!("Entries:");
    // for entry in entries {
    //     let entry = entry.unwrap();
    //     println!("- {} ({}, dir: {})", entry.name(), entry.path(), entry.is_dir());
    // }

    Ok(())

}
