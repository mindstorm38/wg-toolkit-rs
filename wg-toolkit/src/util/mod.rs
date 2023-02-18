//! Provides various internal utilities.

use std::fmt::Write;

pub mod cursor;
pub mod io;
pub mod fnv;


/// Make a string from an escaped sequence of bytes.
pub fn str_from_escaped(data: &[u8]) -> String {
    let str_vec = data.iter()
        .copied()
        .flat_map(std::ascii::escape_default)
        .collect();
    unsafe { String::from_utf8_unchecked(str_vec) }
}


/// Make a string from an hexadecimal representation of a
/// sequence of bytes, and add '..' if the length is longer
/// than given count.
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
