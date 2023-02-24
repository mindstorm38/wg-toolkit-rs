//! Package file codec.
//! 
//! Packages are ZIP files with constrained flags and properties,
//! for example no encryption and no compression is needed.
//! 
//! Following official specification: 
//! https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT

use std::io::{self, Seek, Read, SeekFrom, BufReader, BufRead};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use thiserror::Error;

use crate::util::io::WgReadExt;


/// Signature for the Local File Header structure.
#[allow(unused)]
const LOCAL_FILE_HEADER_SIGNATURE: u32 = 0x04034b50;

/// Signature for the Central Directory Header structure.
const CENTRAL_DIRECTORY_HEADER_SIGNATURE: u32 = 0x02014b50;

/// Signature for the end of central directory.
const END_OF_CENTRAL_DIRECTORY_SIGNATURE: u32 = 0x06054b50;


/// A package-specialized ZIP header reader. This reader can be used for a
/// file-by-file Local File Header reading.
pub struct PackageMetaReader<R> {
    inner: R,
    idx: u32,
    len: u32,
}

/// Structure representing the metadata for a package file. It is returned
/// by [`PackageMetaReader`] when reading headers. 
/// 
/// This structure is also internally used by the [`PackageReader`] structure.
#[derive(Debug, Clone, Default)]
pub struct PackageFileMeta {
    /// Name of the package's file.
    pub file_name: String,
    /// Stored data size.
    pub data_size: u32,
    /// Stored data offset, can be used to seek to the first byte of the data.
    pub data_offset: u64,
    /// Offset to the local file header.
    pub header_offset: u64,
    /// CRC32 of the file's data.
    pub crc32: u32,
}

impl<R> PackageMetaReader<R> {

    #[inline]
    pub fn into_inner(self) -> R {
        self.inner
    }

}

impl<R: Read + Seek> PackageMetaReader<R> {
    
    pub fn new(mut reader: R) -> ReadResult<Self> {

        // Here we need to parse the "End of Central Directory".

        const HEADER_MIN_SIZE: u64 = 22;
        const HEADER_MAX_SIZE: u64 = 22 + u16::MAX as u64;

        let file_length = reader.seek(SeekFrom::End(0))?;
        
        // Here we try to find the position of the End of Central Directory.
        let mut eocd_pos = file_length.checked_sub(HEADER_MIN_SIZE).ok_or(ReadError::InvalidPackageStructure)?;
        let eocd_pos_bound = file_length.saturating_sub(HEADER_MAX_SIZE);

        // A successful return from this loop means we found the EoCD position.
        loop {

            reader.seek(SeekFrom::Start(eocd_pos))?;
            if reader.read_u32()? == END_OF_CENTRAL_DIRECTORY_SIGNATURE {
                break;
            }

            if eocd_pos == eocd_pos_bound {
                // If we didn't find signature on the lower bound.
                return Err(ReadError::InvalidPackageStructure);
            }

            eocd_pos = eocd_pos.checked_sub(1).ok_or(ReadError::InvalidPackageStructure)?;

        }

        // Here we finish parsing the EoCD (we are placed just after the directory signature).
        let disk_number = reader.read_u16()?;
        let disk_with_central_directory = reader.read_u16()?;

        if disk_number != disk_with_central_directory {
            // Multi-disk ZIP files are not valid packages.
            return Err(ReadError::InvalidPackageStructure);
        }

        let number_of_files_on_this_disk = reader.read_u16()?;
        let number_of_files = reader.read_u16()?;

        if number_of_files_on_this_disk != number_of_files {
            // Same as above, no multi-disk, so the number of files must be coherent.
            return Err(ReadError::InvalidPackageStructure);
        }

        let _central_directory_size = reader.read_u32()?;
        let central_directory_offset = reader.read_u32()?;

        let comment_length = reader.read_u16()?;
        if comment_length != 0 {
            // Not expecting comments on packages.
            return Err(ReadError::InvalidPackageStructure);
        }

        // Know we can start parsing all Central Directory Headers.
        // Seek to the first Central Directory Header, reading is ready.
        reader.seek(SeekFrom::Start(central_directory_offset as u64))?;

        Ok(Self { 
            inner: reader, 
            idx: 0,
            len: number_of_files as u32,
        })

    }

    /// Read the next file meta header.
    pub fn read_file_meta(&mut self) -> ReadResult<Option<PackageFileMeta>> {
        
        if self.idx >= self.len {
            return Ok(None);
        }

        self.idx += 1;
        let reader = &mut self.inner;

        if reader.read_u32()? != CENTRAL_DIRECTORY_HEADER_SIGNATURE {
            return Err(ReadError::InvalidFileSignature);
        }

        let _version_made_by = reader.read_u16()?;
        let _version_needed = reader.read_u16()?;
        let flags = reader.read_u16()?;
        let compression_method = reader.read_u16()?;
        let _last_mod_file_time = reader.read_u16()?;
        let _last_mod_file_date = reader.read_u16()?;
        let crc32 = reader.read_u32()?;
        let compressed_size = reader.read_u32()?;
        let uncompressed_size = reader.read_u32()?;
        let file_name_len = reader.read_u16()?;
        let extra_field_len = reader.read_u16()?;
        let file_comment_len = reader.read_u16()?;
        let _disk_number_start = reader.read_u16()?;
        let _internal_file_attrs = reader.read_u16()?;
        let _external_file_attrs = reader.read_u32()?;
        let relative_offset = reader.read_u32()?;

        let file_name = reader.read_string(file_name_len as _)?;
        
        if extra_field_len != 0 || file_comment_len != 0 {
            return Err(ReadError::InvalidFileFields(file_name));
        }

        if flags != 0 {
            // Wargaming packages' ZIPs don't have any flags:
            // no delayed crc32/size, no compression, no encryption
            return Err(ReadError::InvalidFileFlags(file_name));
        }

        if compression_method != 0 || compressed_size != uncompressed_size {
            return Err(ReadError::InvalidFileStorage(file_name));
        }

        Ok(Some(PackageFileMeta { 
            file_name,
            data_size: compressed_size,
            // Add 30 for minimum local file header size.
            // Extra field is checked to be zero-sized.
            data_offset: relative_offset as u64 + 30 + file_name_len as u64, 
            header_offset: relative_offset as u64,
            crc32,
        }))

    }

}


/// A package-specialized ZIP reader. The implementation is simplified
/// because Wargaming doesn't use compression or advanced storage methods.
pub struct PackageReader<R> {
    inner: Arc<Mutex<SharedReader<R>>>,
    files: Vec<PackageFileMeta>,
    files_rev: HashMap<String, usize>,
}

/// Internal structure used to share the seekable reader between file 
/// readers.
struct SharedReader<R> {
    inner: R,
    offset: u64,
}

impl<R> PackageReader<R>
where
    R: Read + Seek
{

    pub fn new(reader: R) -> ReadResult<Self> {
        
        let mut files = Vec::new();
        let mut files_rev = HashMap::new();

        let mut meta_reader = PackageMetaReader::new(reader)?;
        while let Some(meta) = meta_reader.read_file_meta()? {
            files_rev.insert(meta.file_name.clone(), files.len());
            files.push(meta);
        }

        let mut reader = meta_reader.into_inner();
        reader.seek(SeekFrom::Start(0))?;

        Ok(Self {
            inner: Arc::new(Mutex::new(SharedReader {
                inner: reader,
                offset: 0,
            })),
            files,
            files_rev,
        })

    }

    /// Returns the number of files stored in the package.
    #[inline]
    pub fn len(&self) -> usize {
        self.files.len()
    }

    #[inline]
    pub fn files(&self) -> &[PackageFileMeta] {
        &self.files[..]
    }

    /// Iterate over all names of files stored in this package.
    #[inline]
    pub fn file_names(&self) -> impl Iterator<Item = &'_ str> + '_ {
        self.files.iter().map(|meta| meta.file_name.as_str())
    }

    /// Get the index of a file from its name. None if not found.
    #[inline]
    pub fn index_from_name(&self, file_name: &str) -> Option<usize> {
        self.files_rev.get(file_name).copied()
    }

    /// Open a package file by its name.
    pub fn open_by_name(&self, file_name: &str) -> ReadResult<Option<PackageFile<R>>> {
        match self.files_rev.get(file_name) {
            Some(&idx) => self.open_by_index_raw(idx).map(Some),
            None => Ok(None)
        }
    }

    /// Open a package file by its index.
    pub fn open_by_index(&self, file_index: usize) -> ReadResult<Option<PackageFile<R>>> {
        if file_index < self.files.len() {
            self.open_by_index_raw(file_index).map(Some)
        } else {
            Ok(None)
        }
    }

    /// Internal function to open a package from its index, without checking index.
    fn open_by_index_raw(&self, file_index: usize) -> ReadResult<PackageFile<R>> {

        let meta = &self.files[file_index];

        if meta.data_size == 0 {
            return Err(ReadError::NoData);
        }

        // Here we just check that the Local File Header is corresponding.
        let mut reader = self.inner.lock().unwrap();
        // We directly go to the CRC-32 field (iffset 14).
        const LOCAL_HEADER_CRC32_OFFSET: u64 = 14;
        reader.offset = meta.header_offset + LOCAL_HEADER_CRC32_OFFSET;
        reader.inner.seek(SeekFrom::Start(meta.header_offset + LOCAL_HEADER_CRC32_OFFSET))?;
        if reader.inner.read_u32()? != meta.crc32 {
            return Err(ReadError::InvalidFileCrc32);
        }

        Ok(PackageFile {
            reader: BufReader::new(PackageFileInnerReader {
                inner: Arc::clone(&self.inner),
                start_offset: meta.data_offset,
                end_offset: meta.data_offset + meta.data_size as u64,
                current_offset: meta.data_offset,
            }),
        })

    }

}


pub struct PackageFile<R> {
    /// Delegate all read/seek operations to the [`BufReader`].
    reader: BufReader<PackageFileInnerReader<R>>,
}

/// Internal structure packed in a [`BufReader`] before being exposed to
/// the API of [`PackageFile<R>`].
struct PackageFileInnerReader<R> {
    inner: Arc<Mutex<SharedReader<R>>>,
    /// Start cursor offset for this reader.
    start_offset: u64,
    /// End cursor offset for this reader.
    end_offset: u64,
    /// Current cursor offset.
    current_offset: u64,
}

impl<R: Read + Seek> Read for PackageFileInnerReader<R> {

    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {

        // This read implementation is not simple because we have to
        // manage a shared reader, but this reader might not have its
        // cursor set to the position we should read.

        // Compute the remaining length for this file, directly return
        // zero if reached end of file.
        let remaining_len = (self.end_offset - self.current_offset) as usize;
        if remaining_len == 0 {
            return Ok(0);
        }

        let mut reader = self.inner.lock().unwrap();

        if reader.offset != self.current_offset {
            // Seek to the current reader's offset of not already at its position.
            reader.inner.seek(SeekFrom::Start(self.current_offset))?;
        }

        // If the given buffer has greater capacity than available
        // length, just resize buffer.
        if buf.len() > remaining_len {
            buf = &mut buf[..remaining_len];
        }

        let len = reader.inner.read(buf)?; 
        self.current_offset += len as u64;

        // We can infer and set the next reader offset.
        reader.offset = self.current_offset;

        Ok(len)

    }

}

impl<R: Read + Seek> Seek for PackageFileInnerReader<R> {

    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {

        let new_offset = match pos {
            SeekFrom::Start(pos) => self.start_offset.checked_add(pos),
            SeekFrom::End(pos) => u64::try_from(self.end_offset as i64 + pos).ok(),
            SeekFrom::Current(pos) => u64::try_from(self.current_offset as i64 + pos).ok()
        };

        if let Some(new_offset) = new_offset {
            if new_offset < self.end_offset {
                self.current_offset = new_offset;
                return Ok(self.current_offset - self.start_offset)
            }
        }

        Err(io::Error::new(io::ErrorKind::InvalidInput, "tried to seek before or beyond file"))

    }

    fn rewind(&mut self) -> io::Result<()> {
        self.current_offset = self.start_offset;
        Ok(())
    }

    fn stream_position(&mut self) -> io::Result<u64> {
        Ok(self.current_offset - self.start_offset)
    }

}

// This implementation just delegate read operations to
// the underlying buffered reader.
impl<R: Read + Seek> Read for PackageFile<R> {

    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.reader.read_exact(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.reader.read_vectored(bufs)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.reader.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.reader.read_to_string(buf)
    }

}

impl<R: Read + Seek> BufRead for PackageFile<R> {

    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }

}

impl<R: Read + Seek> Seek for PackageFile<R> {

    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.reader.seek(pos)
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        self.reader.stream_position()
    }

}


/// Result type alias for [`ReadError`] error type.
pub type ReadResult<T> = Result<T, ReadError>;

/// Errors that can happen while reading a package.
#[derive(Debug, Error)]
pub enum ReadError {
    /// Some of the core structures of the package are missing or not
    /// valid for packages.
    #[error("invalid package structure")]
    InvalidPackageStructure,
    /// A file has unexpected flags that should not be present for 
    /// packages' zip.
    #[error("invalid file flags for: {0}")]
    InvalidFileFlags(String),
    /// A file is stored in a invalid format, compressed or incoherent 
    /// compressed/uncompressed size.
    #[error("invalid file storage for: {0}")]
    InvalidFileStorage(String),
    /// A file has invalid fields, such as extra field and file comment
    /// who are not expected for packages' files.
    #[error("invalid file fields for: {0}")]
    InvalidFileFields(String),
    /// A file has an invalid signature.
    #[error("invalid file signature")]
    InvalidFileSignature,
    /// When opening a file, the CRC32 of Local File Header is not matching
    /// the one of the Central Directory Header.
    #[error("invalid local file header CRC32")]
    InvalidFileCrc32,
    /// Can't read a directory.
    #[error("the file has no data to read")]
    NoData,
    /// IO error while reading.
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}
