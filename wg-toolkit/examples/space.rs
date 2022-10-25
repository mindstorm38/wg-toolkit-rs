use std::fs::File;
use std::env;

use wgtk::space::CompiledSpace;
use wgtk::space::section::{BWST, BWAL, BWCS, BWSG, BWT2};


fn main() {

    let path = env::var("WGT_SPACE_PATH").unwrap();
    let file = File::open(path).unwrap();

    let mut space = CompiledSpace::new(file).unwrap();

    for section in &space.bwtb.sections {
        println!("- {:?}", section);
    }

    let bwst: BWST = space.decode_section().unwrap();
    let bwal: BWAL = space.decode_section().unwrap();
    let bwcs: BWCS = space.decode_section().unwrap();
    let bwsg: BWSG = space.decode_section().unwrap();
    let bwt2: BWT2 = space.decode_section().unwrap();

    for chunk in &bwt2.chunks {
        println!("[{}/{}] {:?}", chunk.loc_x, chunk.loc_y, bwst.get_string(chunk.resource_fnv));
    }

}
