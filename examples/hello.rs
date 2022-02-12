use wg_tools::space::CompiledSpace;
use wg_tools::space::section::{BWST, BWAL, BWCS, BWSG};

use std::fs::File;
use std::env;


fn main() {

    let path = env::var("WGT_EX_SPACE_PATH").unwrap();
    let mut file = File::open(path).unwrap();

    let mut space = CompiledSpace::new(file).unwrap();

    for section in &space.bwtb.sections {
        println!("- {:?}", section);
    }

    let bwst: BWST = space.decode_section().unwrap();
    let bwal: BWAL = space.decode_section().unwrap();
    let bwcs: BWCS = space.decode_section().unwrap();
    let bwsg: BWSG = space.decode_section().unwrap();

}
