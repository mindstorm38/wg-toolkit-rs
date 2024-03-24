//! Game's resources fetching and indexing.

pub mod package;

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use std::{fs, io};

use indexmap::IndexMap;

use package::{PackageReader, PackageFileReader};


/// Name of the directory storing packages in the "res/" directory.
const PACKAGES_DIR_NAME: &'static str = "packages";


/// A virtual read-only filesystem you can use to walk through the game's resources. This
/// filesystem is designed to work really fast on systems where it will run for a long
/// time and take advantage of its internal cache, but it will also work on run-once
/// environment such as CLI where the first answer latency must be minimal.
pub struct ResFilesystem {
    /// Path to the "res/" directory.
    dir_path: PathBuf,
    /// Packages paths pending to be cached if needed.
    package_cache_pending: Vec<PathBuf>,
    /// Cache for opened package files.
    package_cache: IndexMap<PathBuf, PackageReader<File>>,
    /// A tree index for directories and where to find them.
    dir_tree: Vec<DirInfo>,
}

/// Various informations about indexed and cached directories.
#[derive(Default, Debug)]
struct DirInfo {
    /// Index of the parent directory (root is its own parent).
    parent: usize,
    /// Children directories and their index in the global tree.
    children: HashMap<String, usize>,
    /// This contains the list of packages paths where the directory is known to be 
    /// present. This list is not definitive and is updated as long as new packages are
    /// found to contains the directory.
    in_packages: Vec<usize>,
}

impl ResFilesystem {

    /// Create a new resources filesystem with the given options. This function is
    /// blocking while it is doing a rudimentary early indexing, so this may take some
    /// time.
    pub fn new<P: Into<PathBuf>>(dir_path: P) -> io::Result<Self> {

        let dir_path = dir_path.into();
        let mut package_cache_pending = Vec::new();

        for entry in fs::read_dir(dir_path.join(PACKAGES_DIR_NAME))? {
            
            let entry = entry?;
            let entry_type = entry.file_type()?;
            if !entry_type.is_file() {
                continue;
            }

            if !entry.file_name().as_encoded_bytes().ends_with(b".pkg") {
                continue;
            }

            package_cache_pending.push(entry.path());

        }

        Ok(Self { 
            dir_path,
            package_cache_pending,
            package_cache: IndexMap::new(),
            // The root is always present at index 0 in the tree.
            dir_tree: vec![DirInfo {
                parent: 0,
                children: HashMap::new(),
                in_packages: Vec::new(),
            }],
        })

    }

    /// Read a file from its path. The path should not start with the separator (`/`) and
    /// empty file name should not be empty.
    pub fn read(&mut self, file_path: &str) -> io::Result<PackageFileReader<'_, File>> {

        // Split the file name from the directories path.
        let (dir_path, file_name) = file_path.rsplit_once('/').unwrap_or(("", file_path));
        println!("file_path={file_path} file_name={file_name}");

        // Iteratively find (and create if not existing) directories in the tree.
        let mut dir_index = 0;
        for dir_name in dir_path.split('/') {

            let dir_info = &mut self.dir_tree[dir_index];
            println!("dir_name={dir_name}");

            if let Some(&index) = dir_info.children.get(dir_name) {
                // There is a subdirectory, just go in it.
                dir_index = index;
            } else {
                // Then we add the new dir info, and add it to its parent.
                let child_dir_index = self.dir_tree.len();
                self.dir_tree.push(DirInfo {
                    parent: dir_index,
                    children: HashMap::new(),
                    in_packages: Vec::new(),
                });
                self.dir_tree[dir_index].children.insert(dir_name.to_string(), child_dir_index);
                dir_index = child_dir_index;
            }

        }

        // TODO: Iteratively check in_packages for all directories down to root while not
        // checking twice for the same package (using a hashset?). If the file has not 
        // been found in those, check remaining packages and only then open pending 
        // packages to check them.

        // For each known packages, check if the file can be found, when it has been
        // found, just take the package index and the file index in it.
        let mut index = self.dir_tree[dir_index].in_packages.iter()
            .find_map(|&package_index| {
                let package = &self.package_cache[package_index];
                package.index_by_name(file_path).map(|file_index| (package_index, file_index))
            });

        // If it has not been found, check in pending packages...
        while index.is_none() {

            // If no index but the pending packages are empty, the file is not found.
            let Some(package_path) = self.package_cache_pending.pop() else {
                return Err(io::Error::from(io::ErrorKind::NotFound));
            };

            // Try to open package and parse its header, then find the file index.
            let package = PackageReader::new(File::open(&package_path)?)?;
            let file_index = package.index_by_name(file_path);

            // Then we insert it and retrieve its index within the index map.
            let (
                package_index, 
                prev_package
            ) = self.package_cache.insert_full(package_path, package);
            debug_assert!(prev_package.is_none());

            // Set the index to exit the loop.

            // If file has been found in this package, set the index to exit the loop,
            // and then update 'dir_info' and its parents, because they all contain.
            if let Some(file_index) = file_index {

                loop {
                    let dir_info = &mut self.dir_tree[dir_index];
                    dir_info.in_packages.push(package_index);
                    if dir_index == 0 {
                        break; // Root don't have parent directory.
                    }
                    dir_index = dir_info.parent;
                }

                index = Some((package_index, file_index));

            }

        }

        println!("self.dir_tree: {:?}", self.dir_tree);
        println!("self.package_cache: {:?}", self.package_cache);

        let (package_index, file_index) = index.unwrap();
        let package = &mut self.package_cache[package_index];

        match package.read_by_index(file_index) {
            Ok(reader) => Ok(reader),
            Err(e) if e.kind() == io::ErrorKind::NotFound => unreachable!(),
            Err(e) => Err(e)
        }

    }

}






//     /// Read a directory given a path. This method will success if at least
//     /// one of the packages (or root) actually contains the directory. If
//     /// not, a [`ResError::DirectoryNotFound`].
//     pub fn read_dir(&mut self, path: &str) -> ResResult<ResReadDir> {

//         // The canonic path needs to end with a slash, this save
//         // some computations and simplify further operations.
//         let mut canon_path = path.trim_start_matches('/').to_string();
//         if !canon_path.ends_with('/') { canon_path.push('/') }
//         // Redefine dir_path as immutable.
//         let canon_path = canon_path;

//         // Note that the directory index don't store the last '/'.
//         let mut index_path = &canon_path.as_str()[..canon_path.len() - 1];
        
//         loop {

//             if let Some(locs) = self.dir_index.get(index_path) {

//                 let mut root_read_dir = None;
//                 if locs.in_root {
//                     let file_path = self.dir_path.join(&canon_path);
//                     if file_path.is_dir() {
//                         root_read_dir = Some(fs::read_dir(file_path)?);
//                     }
//                 }

//                 let mut packages = Vec::new();
//                 for package in locs.in_packages.iter().rev() {
//                     // Get the opened package and check if it contains the directory.
//                     let pkg = self.package_cache.ensure(package, &self.dir_path)?;
//                     if let Some(dir_index) = pkg.index_from_name(&canon_path) {
//                         // The next file index is directly set to the file following the directory.
//                         packages.push((Arc::clone(&pkg), dir_index + 1));
//                     }
//                 }

//                 if root_read_dir.is_none() && packages.is_empty() {
//                     return Err(ResError::DirectoryNotFound);
//                 } else {
//                     return Ok(ResReadDir {
//                         dir_path: canon_path,
//                         root_read_dir,
//                         packages,
//                     })
//                 }

//             }

//             // If we are already on the root directory but it isn't indexed.
//             if index_path.is_empty() {
//                 return Err(ResError::DirectoryNotFound);
//             }

//             // If the current directory's path is not indexed,
//             // check for its parent. Once we reached 
//             if let Some(pos) = index_path.rfind('/') {
//                 index_path = &index_path[..pos];
//             } else {
//                 index_path = "";
//             }

//         }

//     }

//     pub fn open(&mut self, path: &str) -> ResResult<ResFile> {

//         let full_path = path.trim_matches('/');

//         let mut path = full_path;
//         loop {

//             let slash_index = path.rfind("/").unwrap_or(0);
//             let parent_path = &path[..slash_index];

//             if let Some(locs) = self.dir_index.get(parent_path) {
                
//                 if locs.in_root {
//                     let file_path = self.dir_path.join(full_path);
//                     if file_path.is_file() {
//                         let file = File::open(file_path)?;
//                         return Ok(ResFile(ResFileKind::System(file)));
//                     }
//                 }

//                 for package in &locs.in_packages {
//                     let pkg = self.package_cache.ensure(package, &self.dir_path)?;
//                     if let Some(pkg_file) = pkg.open_by_name(full_path)? {
//                         return Ok(ResFile(ResFileKind::Package(pkg_file)));
//                     }
//                 }

//                 break;

//             } else {
//                 // If the parent directory can't be found, check its parent.
//                 path = parent_path;
//                 if path.is_empty() {
//                     break;
//                 }
//             }

//         }

//         Err(ResError::FileNotFound)

//     }

// }


// impl PackageCache {

//     /// Internal method to ensure that a zip archive is opened.
//     fn ensure(&mut self, package: &String, dir_path: &PathBuf) -> pkg::ReadResult<&Arc<PackageReader<File>>> {
//         Ok(match self.inner.entry(package.clone()) {
//             Entry::Occupied(o) => o.into_mut(),
//             Entry::Vacant(v) => {
//                 let mut package_path = dir_path.join(PACKAGES_DIR_NAME);
//                 package_path.push(package);
//                 v.insert(Arc::new(PackageReader::new(File::open(package_path)?)?))
//             }
//         })
//     }

// }


// /// A seakable reader for a resource file. 
// pub struct ResFile(ResFileKind);

// /// Internal kind of resource file reader.
// enum ResFileKind {
//     /// System file.
//     System(File),
//     /// Package file.
//     Package(PackageFile<File>),
// }


// /// Iterator for a directory in resources.
// pub struct ResReadDir {
//     /// The full path to search for, in packages. 
//     /// **End with a slash.**
//     dir_path: String,
//     /// When some, this std IO [`ReadDir`] should be consumed
//     /// before fetching packages.
//     root_read_dir: Option<ReadDir>,
//     /// Packages to fetch, in reverse order, the last one is the
//     /// current package being read. Packages are associated to
//     /// the file index currently being read.
//     /// 
//     /// Because packages' files are ordered by directories, if 
//     /// the file index no longer points to a file of the dir,
//     /// this means that we finished reading the dir.
//     packages: Vec<(Arc<PackageReader<File>>, usize)>,
// }

// /// A directory entry returned from the [`ResReadDir`] iterator.
// #[derive(Debug)]
// pub struct ResDirEntry {
//     path: String,
//     dir: bool,
// }

// impl Iterator for ResReadDir {

//     type Item = ResResult<ResDirEntry>;

//     fn next(&mut self) -> Option<Self::Item> {

//         // FIXME: Remove duplicates from iteration.

//         /// Internal function used to convert an std IO [`DirEntry`] result
//         /// into a [`ResDirEntry`] result used by this iterator.
//         fn convert_entry(entry: io::Result<DirEntry>, full_path: &str) -> ResResult<ResDirEntry> {

//             let entry = entry?;
//             let file_type = entry.file_type()?;
//             let file_name_raw = entry.file_name();
//             let file_name = file_name_raw.to_str()
//                 .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "non-utf8 filename"))?;

//             Ok(ResDirEntry { 
//                 path: format!("{full_path}{file_name}"),
//                 dir: file_type.is_dir()
//             })

//         }
        
//         // The iterator state machine starts with the root read directory,
//         // if present. Once all root entries has been yielded, the 
//         if let Some(read_dir) = &mut self.root_read_dir {
//             if let Some(entry) = read_dir.next() {
//                 return Some(convert_entry(entry, &self.dir_path));
//             }
//             // If no entry has been found, set the read dir to None to
//             // prevent further iteration of it. Might also free so internal
//             // buffers.
//             self.root_read_dir = None;
//         }

//         // Try to get the next file meta to return.
//         // If no package is found, go to the next available package, if
//         // no more package is available, this is the end of the iterator.
//         loop {

//             if let Some((
//                 current_package, 
//                 file_index
//             )) = self.packages.last_mut() {

//                 while let Some(meta) = current_package.files().get(*file_index) {

//                     *file_index += 1;

//                     // We only take the file if it starts with our full path.
//                     if meta.file_name.starts_with(&self.dir_path[..]) {

//                         // Get the sub path after the common the directory path.
//                         let sub_path = &meta.file_name[self.dir_path.len()..];
//                         // Test if this is a file directly contained by the directory.
//                         let sub_direct = match sub_path.find('/') {
//                             Some(pos) => pos == sub_path.len() - 1, // Directory
//                             None => true // File
//                         };

//                         // Do not include terminal slash in entry's path.
//                         let no_slash_path = meta.file_name.strip_suffix('/');

//                         if sub_direct {
//                             return Some(Ok(ResDirEntry { 
//                                 dir: no_slash_path.is_some(),
//                                 path: no_slash_path.unwrap_or(&meta.file_name[..]).to_string(),
//                             }))
//                         }
                        
//                     } else {
//                         // If the current file don't start with the directory path,
//                         // we reached the end of the directory.
//                         break;
//                     }

//                 }

//                 // If we leave the previous loop without returning, this means that 
//                 // the current package is exhausted, so we pop it.
//                 if self.packages.pop().is_none() {
//                     return None; // Iterator end!
//                 }

//             } else {
//                 // No package remaining to read.
//                 return None;
//             }

//         }

//     }

// }

// impl ResDirEntry {

//     /// Get the full path of the entry.
//     #[inline]
//     pub fn path(&self) -> &str {
//         &self.path[..]
//     }

//     /// Get the file name of the entry.
//     pub fn name(&self) -> &str {
//         match self.path.rfind('/') {
//             Some(pos) => &self.path[pos + 1..],
//             None => &self.path[..]
//         }
//     }

//     #[inline]
//     pub fn is_dir(&self) -> bool {
//         self.dir
//     }

//     #[inline]
//     pub fn is_file(&self) -> bool {
//         !self.is_dir()
//     }

// }


// /// Result type aslias for [`ResError`].
// pub type ResResult<T> = Result<T, ResError>;

// /// Errors that can happen while interacting with the resources
// /// filesystem.
// #[derive(Debug, Error)]
// pub enum ResError {
//     /// The file was not found.
//     #[error("file not found")]
//     FileNotFound,
//     /// The directory was not found.
//     #[error("directory not found")]
//     DirectoryNotFound,
//     /// Package read error.
//     #[error("package error: {0}")]
//     Package(#[from] pkg::ReadError),
//     /// IO error.
//     #[error("io error: {0}")]
//     Io(#[from] io::Error),
// }
