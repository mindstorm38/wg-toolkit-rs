use std::env;

use pelite::FileMap;

use wgtk::exec::Disassembly;


fn main() {

    let exec_path = env::var("WGT_EXEC_PATH").unwrap();
    let exec_mmap = FileMap::open(&exec_path).unwrap();

    let mut dis = Disassembly::new(exec_mmap.as_ref()).unwrap();

    for section in dis.get_sections() {
        println!("- {}", section.name().unwrap());
    }

    dis.analyze_strings_in_section(b".rdata");

    for (&a, &s) in dis.get_strings() {
        if s == "__builtin__" {
            println!("[{a:X}] {s}");
        }
    }

    //dis.analyze_functions(b".text");

}
