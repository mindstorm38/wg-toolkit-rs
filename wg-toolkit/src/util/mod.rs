//! Provides various internal utilities.

use std::fmt::{self, Write};

pub mod cursor;
pub mod io;
pub mod fnv;


// /// Make a string from an escaped sequence of bytes.
// pub fn str_from_escaped(data: &[u8]) -> String {
//     let str_vec = data.iter()
//         .copied()
//         .flat_map(std::ascii::escape_default)
//         .collect();
//     unsafe { String::from_utf8_unchecked(str_vec) }
// }


/// A helper structure for beautiful printing of bytes. 
/// It provides format implementations for upper and
/// lower hex formatters (`{:x}`, `{:X}`).
pub struct BytesFmt<'a>(pub &'a [u8]);

impl fmt::UpperHex for BytesFmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            f.write_fmt(format_args!("{:02X}", byte))?;
        }
        Ok(())
    }
}

impl fmt::LowerHex for BytesFmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            f.write_fmt(format_args!("{:02x}", byte))?;
        }
        Ok(())
    }
}


pub struct TruncateFmt<F>(pub F, pub usize);

impl<F: fmt::Display> fmt::Display for TruncateFmt<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        buf.write_fmt(format_args!("{}", self.0))?;
        if buf.len() > self.1 {
            buf.truncate(self.1 - 2);
            buf.push_str("..");
        }
        f.write_str(&buf)
    }
}
