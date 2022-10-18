use std::path::Path;
use std::fs::File;
use std::env;

use wgtk::model::read_model;


fn main() {

    let path_raw = env::var("WGT_MODEL_PATH").unwrap();
    let path = Path::new(&path_raw);
    let visual_file = File::open(path.with_extension("visual_processed")).unwrap();
    let primitives_file = File::open(path.with_extension("primitives_processed")).unwrap();

    read_model(visual_file, primitives_file);

}
