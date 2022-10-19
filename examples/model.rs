use std::io::{BufReader, Seek};
use std::path::Path;
use std::fs::File;
use std::env;

use wgtk::pxml::de::from_reader;
use wgtk::pxml::ser::to_writer;

use wgtk::model::read_model;

fn main() {

    let path_raw = env::var("WGT_MODEL_PATH").unwrap();
    let path = Path::new(&path_raw);
    let mut visual_file = File::open(path.with_extension("visual_processed")).unwrap();
    let primitives_file = File::open(path.with_extension("primitives_processed")).unwrap();

    let visual = from_reader(&mut visual_file).unwrap();
    println!("{:#?}", visual);

    to_writer(File::create(path.with_extension("visual_processed_2")).unwrap(), &*visual).unwrap();
    from_reader(File::open(path.with_extension("visual_processed_2")).unwrap()).unwrap();

    // visual_file.seek(std::io::SeekFrom::Start(0));
    // read_model(visual_file, primitives_file);

}
