//! Provides various internal utilities.

use std::fmt::{self, Write};

pub mod io;
pub mod fnv;
pub mod cuckoo;


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

/// A helper structure to truncate the output of some display implementor, adding 
/// trailing '..' if necessary.
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

pub struct SizeFmt(pub u64);

impl fmt::Display for SizeFmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            n @ 0..=999 => 
                write!(f, "{:>4} B", n),
            n @ 1_000..=999_999 => 
                write!(f, "{:>3} kB", n / 1_000),
            n @ 1_000_000..=999_999_999 => 
                write!(f, "{:>3} MB", n / 1_000_000),
            n @ 1_000_000_000..=999_999_999_999 => 
                write!(f, "{:>3} GB", n / 1_000_000_000),
            n =>
                write!(f, "{:>3} TB", n / 1_000_000_000_000),
        }
    }
}
