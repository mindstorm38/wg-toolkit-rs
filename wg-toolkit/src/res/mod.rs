//! Game's resources fetching and indexing.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::{File, ReadDir, DirEntry};
use std::sync::Arc;
use std::{fs, io};

pub mod pkg;
use pkg::{PackageMetaReader, PackageReader, PackageFile};

use thiserror::Error;


/// Name of the directory storing packages in the "res/" directory.
const PACKAGES_DIR_NAME: &'static str = "packages";


/// Options used for opening and indexing the game's resources
/// filesystem.
#[derive(Debug, Clone)]
pub struct ResOptions {
    /// Max depth of directories to index.
    /// Default value to 3, optimal because vehicles
    /// and assets are divided at this level.
    pub index_max_depth: usize,
}

impl Default for ResOptions {
    fn default() -> Self {
        Self { 
            index_max_depth: 3 
        }
    }
}

/// A virtual filesystem you can use to walk through the game's
/// resources. 
pub struct ResFilesystem {
    /// Path the "res/" directory.
    dir_path: PathBuf,
    /// Cache for opened packages.
    package_cache: PackageCache,
    /// Indexing for directory, mapping their path to their
    /// location, these can be located in the root directory
    /// and/or many packages. Some directories may not have
    /// a mapping here, in case of missing directories, just
    /// refer to one of its parents.
    /// 
    /// If some directory is mapped in this index, all its
    /// parents are also mapped.
    /// 
    /// Keys are directory's path without the terminal slash.
    dir_index: HashMap<String, DirLocations>,
}

/// Cache for opened packages' archives.
struct PackageCache {
    inner: HashMap<String, Arc<PackageReader<File>>>,
}

/// List of locations for a top level directory in the index.
#[derive(Default, Debug)]
struct DirLocations {
    // The TLD is available in the root "res/" directory.
    in_root: bool,
    // Packages where the TLD is available.
    in_packages: Vec<String>,
}

impl ResFilesystem {

    pub fn new<P: Into<PathBuf>>(dir_path: P) -> io::Result<Self> {
        Self::with_options(dir_path, ResOptions::default())
    }

    pub fn with_options<P: Into<PathBuf>>(dir_path: P, options: ResOptions) -> io::Result<Self> {

        let dir_path = dir_path.into();
        let mut dir_index: HashMap<String, DirLocations> = HashMap::new();

        // If there are top-level file in root directory.
        let mut root_tlf = false;
        for entry in fs::read_dir(&dir_path)? {
            if let Ok(entry) = entry {
                let entry_type = entry.file_type()?;
                if entry_type.is_file() {
                    // Top-level file.
                    root_tlf = true;
                } else if entry_type.is_dir() {
                    // Top-level directory.
                    if let Some(dir_name) = entry.file_name().to_str() {
                        // Packages directory is special and should not be considered as existing.
                        if dir_name != PACKAGES_DIR_NAME {
                            dir_index.entry(dir_name.to_string()).or_default().in_root = true;
                        }
                    }
                }
            }
        }

        if root_tlf {
            // If there are files in root directory.
            dir_index.entry(String::new()).or_default().in_root = true;
        }

        for entry in fs::read_dir(dir_path.join(PACKAGES_DIR_NAME))? {
            if let Ok(entry) = entry {
                if entry.file_type()?.is_file() {
                    if let Some(package_name) = entry.file_name().to_str() {
                        if package_name.ends_with(".pkg") {
                            
                            println!("========= {package_name} =========");

                            let mut pkg = PackageMetaReader::new(File::open(entry.path())?).unwrap();

                            'files_it: 
                            while let Some(meta) = pkg.read_file_meta().unwrap() {
                                
                                let mut depth = 0;
                                for ch in meta.file_name.chars().rev() {
                                    if ch == '/' {
                                        if depth >= options.index_max_depth {
                                            // Do not index this directory.
                                            continue 'files_it;
                                        }
                                        depth += 1;
                                    } else if depth == 0 {
                                        // If the first character from the end if not a slash,
                                        // it's not a directory, so ignore the file.
                                        continue 'files_it;
                                    }
                                }

                                // Directory name without terminal slash.
                                let dir_name = &meta.file_name[..meta.file_name.len() - 1];
                                if let Some(locs) = dir_index.get_mut(dir_name) {
                                    locs.in_packages.push(package_name.to_string());
                                } else {
                                    dir_index.insert(dir_name.to_string(), DirLocations { 
                                        in_root: false, 
                                        in_packages: vec![package_name.to_string()],
                                    });
                                }

                            }

                        }
                    }
                }
            }
        }

        Ok(Self { 
            dir_path,
            package_cache: PackageCache {
                inner: HashMap::new(),
            },
            dir_index,
        })

    }

    pub fn read_dir(&mut self, path: &str) -> ResResult<ResReadDir> {

        // Directory paths must end with '/'.
        let full_path = path.trim_matches('/').to_string();
        let mut path = full_path.as_str();
        
        loop {

            if let Some(locs) = self.dir_index.get(path) {

                let mut root_read_dir = None;
                if locs.in_root {
                    let file_path = self.dir_path.join(&full_path);
                    if file_path.is_dir() {
                        root_read_dir = Some(fs::read_dir(file_path)?);
                    }
                }

                let mut packages = Vec::new();
                for package in locs.in_packages.iter().rev() {
                    let pkg = self.package_cache.ensure(package, &self.dir_path)?;
                    packages.push(Arc::clone(&pkg));
                }

                return Ok(ResReadDir {
                    dir_path: full_path,
                    root_read_dir,
                    packages,
                    package_next_file_index: 0,
                })

            }

            // If we are already on the root directory but it isn't indexed.
            if path.is_empty() {
                return Err(ResError::DirectoryNotFound);
            }

            // If the current directory's path is not indexed,
            // check for its parent. Once we reached 
            if let Some(pos) = path.rfind('/') {
                path = &path[..pos];
            } else {
                path = "";
            }

        }

    }

    pub fn open(&mut self, path: &str) -> ResResult<ResFile> {

        let full_path = path.trim_matches('/');

        let mut path = full_path;
        loop {

            let slash_index = path.rfind("/").unwrap_or(0);
            let parent_path = &path[..slash_index];

            if let Some(locs) = self.dir_index.get(parent_path) {
                
                if locs.in_root {
                    let file_path = self.dir_path.join(full_path);
                    if file_path.is_file() {
                        let file = File::open(file_path)?;
                        return Ok(ResFile(ResFileKind::System(file)));
                    }
                }

                for package in &locs.in_packages {
                    let pkg = self.package_cache.ensure(package, &self.dir_path)?;
                    if let Some(pkg_file) = pkg.open_by_name(full_path)? {
                        return Ok(ResFile(ResFileKind::Package(pkg_file)));
                    }
                }

                break;

            } else {
                // If the parent directory can't be found, check its parent.
                path = parent_path;
                if path.is_empty() {
                    break;
                }
            }

        }

        Err(ResError::FileNotFound)

    }

}


impl PackageCache {

    /// Internal method to ensure that a zip archive is opened.
    fn ensure(&mut self, package: &String, dir_path: &PathBuf) -> pkg::ReadResult<&Arc<PackageReader<File>>> {
        Ok(match self.inner.entry(package.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let mut package_path = dir_path.join(PACKAGES_DIR_NAME);
                package_path.push(package);
                v.insert(Arc::new(PackageReader::new(File::open(package_path)?)?))
            }
        })
    }

}


/// A seakable reader for a resource file. 
pub struct ResFile(ResFileKind);

/// Internal kind of resource file reader.
enum ResFileKind {
    /// System file.
    System(File),
    /// Package file.
    Package(PackageFile<File>),
}


/// Iterator for a directory in resources.
pub struct ResReadDir {
    /// The full path to search for, in packages. Should not end
    /// with a slash.
    dir_path: String,
    root_read_dir: Option<ReadDir>,
    /// Packages to fetch, in reverse order, the last one is the
    /// current package being read.
    packages: Vec<Arc<PackageReader<File>>>,
    /// Next file index to read on the current package (see 
    /// `packages`).
    package_next_file_index: usize,
}

/// A directory entry returned from the [`ResReadDir`] iterator.
#[derive(Debug)]
pub struct ResDirEntry {
    path: String,
    dir: bool,
}

impl Iterator for ResReadDir {

    type Item = ResResult<ResDirEntry>;

    fn next(&mut self) -> Option<Self::Item> {

        /// Internal function used to convert an std IO [`DirEntry`] result
        /// into a [`ResDirEntry`] result used by this iterator.
        fn convert_entry(entry: io::Result<DirEntry>, full_path: &str) -> ResResult<ResDirEntry> {

            let entry = entry?;
            let file_type = entry.file_type()?;
            let file_name_raw = entry.file_name();
            let file_name = file_name_raw.to_str()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "non-utf8 filename"))?;

            Ok(ResDirEntry { 
                path: format!("{full_path}/{file_name}"),
                dir: file_type.is_dir()
            })

        }
        
        // The iterator state machine starts with the root read directory,
        // if present. Once all root entries has been yielded, the 
        if let Some(read_dir) = &mut self.root_read_dir {
            if let Some(entry) = read_dir.next() {
                return Some(convert_entry(entry, &self.dir_path));
            }
            // If no entry has been found, set the read dir to None to
            // prevent further iteration of it. Might also free so internal
            // buffers.
            self.root_read_dir = None;
        }

        // Try to get the next file meta to return.
        // If no package is found, go to the next available package, if
        // no more package is available, this is the end of the iterator.
        loop {

            // Full length of the directory, + length of slash '/'.
            let dir_path_len = self.dir_path.len() + 1;

            if let Some(current_package) = self.packages.last() {
                while let Some(meta) = current_package.files().get(self.package_next_file_index) {

                    self.package_next_file_index += 1;

                    // We only take the file if it starts with our full path.
                    if meta.file_name.len() > dir_path_len && meta.file_name.starts_with(&self.dir_path[..]) {

                        // Get the sub path after the common the directory path.
                        let sub_path = &meta.file_name[dir_path_len..];
                        // Test if this is a file directly contained by the directory.
                        let sub_direct = match sub_path.find('/') {
                            Some(pos) => pos == sub_path.len() - 1, // Directory
                            None => true // File
                        };

                        // Do not include terminal slash in entry's path.
                        let no_slash_path = meta.file_name.strip_suffix('/');

                        if sub_direct {
                            return Some(Ok(ResDirEntry { 
                                dir: no_slash_path.is_some(),
                                path: no_slash_path.unwrap_or(&meta.file_name[..]).to_string(),
                            }))
                        }
                        
                    }

                }
            } else {
                // No package remaining to read, 
                return None;
            }
            
            match self.packages.pop() {
                Some(_) => self.package_next_file_index = 0,
                None => return None // Iterator end!
            }

        }

    }

}

impl ResDirEntry {

    /// Get the full path of the entry.
    #[inline]
    pub fn path(&self) -> &str {
        &self.path[..]
    }

    /// Get the file name of the entry.
    pub fn name(&self) -> &str {
        match self.path.rfind('/') {
            Some(pos) => &self.path[pos + 1..],
            None => &self.path[..]
        }
    }

    #[inline]
    pub fn is_dir(&self) -> bool {
        self.dir
    }

    #[inline]
    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

}


/// Result type aslias for [`ResError`].
pub type ResResult<T> = Result<T, ResError>;

/// Errors that can happen while interacting with the resources
/// filesystem.
#[derive(Debug, Error)]
pub enum ResError {
    /// The file was not found.
    #[error("file not found")]
    FileNotFound,
    /// The directory was not found.
    #[error("directory not found")]
    DirectoryNotFound,
    /// Package read error.
    #[error("package error: {0}")]
    Package(#[from] pkg::ReadError),
    /// IO error.
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}
