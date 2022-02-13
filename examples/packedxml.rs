use std::fs::File;
use std::env;
use xmltree::EmitterConfig;

use wgtk::xml::unpack_xml;


fn main() {

    let path = env::var("WGT_XML_PATH").unwrap();
    let mut file = File::open(path).unwrap();

    let res = unpack_xml(&mut file).unwrap();
    // println!("{:#?}", res);

    let xml_write_config = EmitterConfig::new()
        .perform_indent(true);

    let mut buf = Vec::new();
    res.write_with_config(&mut buf, xml_write_config).unwrap();

    let txt = String::from_utf8(buf).unwrap();
    println!("{}", txt);

}