use std::collections::HashMap;
use pelite::FileMap;
use pelite::pe64::{Pe, PeFile, PeObject};

use std::env;


fn main() {

    let exec_path = env::var("WGT_EXEC_PATH").unwrap();
    let exec_mmap = FileMap::open(&exec_path).unwrap();

    let exec = PeFile::from_bytes(&exec_mmap).unwrap();

    /*for dll in exec.imports().unwrap() {
        println!("{}", dll.dll_name().unwrap());
        for import in dll.int().unwrap() {
            println!("- {:?}", import.unwrap());
        }
    }*/

    let base = exec.optional_header().ImageBase as usize;

    for section in exec.section_headers() {
        let section_file_range = section.file_range();
        let section_virtual_range = section.virtual_range();
        let data = &exec.image()[section_file_range.start as usize..section_file_range.end as usize];
        match section.name_bytes() {
            b".rdata" => {

                let mut str_start = 0;
                let mut str_parsing = false;
                let mut str_length = 0;
                let mut str_count_alphanum = 0;

                let mut strings = HashMap::new();

                for (i, &b) in data.iter().enumerate() {

                    let chr = b as char;

                    if b != 0 {
                        if !str_parsing {
                            str_start = i;
                            str_count_alphanum = 0;
                            str_length = 0;
                            str_parsing = true;
                        }
                        if chr.is_ascii_alphanumeric() {
                            str_count_alphanum += 1;
                        }
                        str_length += 1;
                    } else if str_parsing {
                        let alphanum_ratio = str_count_alphanum as f32 / str_length as f32;
                        if alphanum_ratio > 0.5 {
                            let str_data = &data[str_start..i];
                            let str_ascii = String::from_utf8_lossy(str_data).into_owned();
                            strings.insert(str_ascii, base + section_virtual_range.start as usize + str_start);
                        }
                        str_parsing = false;
                    }

                    if i > 100000 {
                        break;
                    }

                }

                for (s, &a) in &strings {
                    println!("[0x{:X}] '{}'", a, s);
                }

            },
            _ => {}
        }

    }

}