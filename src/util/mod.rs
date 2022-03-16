//! Provides various internal utilities.

mod cursor;
pub use cursor::*;


pub fn str_from_escaped(data: &[u8]) -> String {
    let str_vec = data.iter()
        .copied()
        .flat_map(std::ascii::escape_default)
        .collect();
    unsafe { String::from_utf8_unchecked(str_vec) }
}
