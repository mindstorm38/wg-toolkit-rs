//! Provides various internal utilities.

use std::fmt::Write;

pub mod cursor;
pub mod fnv;
pub mod io;


pub fn str_from_escaped(data: &[u8]) -> String {
    let str_vec = data.iter()
        .copied()
        .flat_map(std::ascii::escape_default)
        .collect();
    unsafe { String::from_utf8_unchecked(str_vec) }
}


pub fn get_hex_str_from(data: &[u8], count: usize) -> String {
    let mut buf = String::new();
    for byte in data.iter().copied().take(count) {
        buf.write_fmt(format_args!("{:02X}", byte)).unwrap();
    }
    if data.len() > count {
        buf.push_str("..");
    }
    buf
}
