use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::env;

use wgtk::model;

fn main() {

    let path_raw = env::var("WGT_MODEL_PATH").unwrap();
    let path = Path::new(&path_raw);
    let mut visual_file = File::open(path.with_extension("visual_processed")).unwrap();
    let primitives_file = File::open(path.with_extension("primitives_processed")).unwrap();

    let model = model::from_readers(visual_file, primitives_file).unwrap();

    // println!("{model:#?}");

    let mut obj_file = File::create("./test.obj").unwrap();

    let (rs, rsd) = model.get_render_set(0);
    let (vertices, primitives) = rsd.get_group(0);

    for v in vertices {
        writeln!(obj_file, "v {} {} {}", v.position.x, v.position.y, v.position.z);
    }
    for p in primitives {
        writeln!(obj_file, "f {a} {b} {c}", a = p.a + 1, b = p.b + 1, c = p.c + 1);
    }

}
