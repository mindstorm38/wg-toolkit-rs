use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::env;

use wgtk::model::read_model;

use serde_json::Value;
use quick_xml::de::{from_reader, from_str};

fn main() {

    let path_raw = env::var("WGT_MODEL_PATH").unwrap();
    let path = Path::new(&path_raw);
    let visual_file = File::open(path.with_extension("visual_processed")).unwrap();
    let primitives_file = File::open(path.with_extension("primitives_processed")).unwrap();

//     let res = from_str::<Value>(r#"<tag1 att1 = "test">
//     <tag2><!--Test comment-->Test</tag2>
//     <tag2>Test 2</tag2>
//  </tag1>"#);
//     println!("{res:#?}");

    read_model(visual_file, primitives_file);

}
