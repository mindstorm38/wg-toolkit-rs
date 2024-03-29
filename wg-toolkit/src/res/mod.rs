//! Game's resources fetching and indexing.

pub mod package;

use std::collections::{hash_map, BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use std::{fs, io, mem};
use std::fs::{File, ReadDir};

use indexmap::IndexMap;

use package::{PackageReader, PackageFileReader};


/// Name of the directory storing packages in the "res/" directory.
const PACKAGES_DIR_NAME: &'static str = "packages";


/// A virtual read-only filesystem you can use to walk through the game's resources. This
/// filesystem is designed to work really fast on systems where it will run for a long
/// time and take advantage of its internal cache, but it will also work on run-once
/// environment such as CLI where the first answer latency must be minimal.
/// 
/// This filesystem is made to be shared between threads and immutably accessed.
/// 
/// Internally, this filesystem has a cache to improve response delay. The challenge is
/// that directories may reside in many packages, but files are present only in one
/// package.
#[derive(Debug)]
pub struct ResFilesystem {
    /// Path to the "res/" directory.
    dir_path: PathBuf,
    /// Pending packages to be opened and cached.
    pending_package_cache: Vec<PathBuf>,
    /// Cache for opened package files.
    package_cache: IndexMap<PathBuf, PackageInfo>,
    /// Cache for known files and directories.
    file_cache: NodeCache,
}

/// Various informations about a cached package.
#[derive(Debug)]
struct PackageInfo {
    /// The original package reader, that is cloned when file handles are returned.
    reader: PackageReader<File>,
    /// This is used while searching for a file, each packages that has been checked is
    /// marked with true, this avoid searching multiple times in the same package, this 
    /// should be reset to false after each search.
    excluded: bool,
}

impl ResFilesystem {

    /// Create a new resources filesystem with the given options. This function is
    /// blocking while it is doing a rudimentary early indexing, so this may take some
    /// time.
    pub fn new<P: Into<PathBuf>>(dir_path: P) -> io::Result<Self> {

        let dir_path = dir_path.into();
        let mut pending_package_cache = Vec::new();

        for entry in fs::read_dir(dir_path.join(PACKAGES_DIR_NAME))? {
            
            let entry = entry?;
            let entry_type = entry.file_type()?;
            if !entry_type.is_file() {
                continue;
            }

            if !entry.file_name().as_encoded_bytes().ends_with(b".pkg") {
                continue;
            }

            pending_package_cache.push(entry.path());

        }

        Ok(Self { 
            dir_path,
            pending_package_cache,
            package_cache: IndexMap::new(),
            file_cache: NodeCache::new(),
        })

    }

    pub fn read(&mut self, file_path: &str) -> io::Result<PackageFileReader<File>> {

        let mut packages = Vec::new();
        let state = self.file_cache.find_node_packages(file_path, &mut packages);

        // Iterate in reverse because most preferred packages are last.
        for node_package in packages.iter().rev() {

            let (package_path, package_info) = self.package_cache.get_index_mut(node_package.package_index).unwrap();
            
            // Ignore that package if it was already explored.
            if package_info.excluded {
                continue;
            }

            package_info.excluded = true;
            if !node_package.present {
                continue;
            }

            // Skip file indices up to the hinted index.
            let file_index = package_info.reader.names()
                .enumerate()
                .skip(node_package.file_index)
                .find(|&(_, file_name)| file_name == file_path)
                .map(|(file_index, _)| file_index);

            // If file is found, return an owned reader.
            if let Some(file_index) = file_index {

                // TODO: Add the file in the cache, found for the given package index.

                // NOTE: Error should not be "NotFound" here.
                let mut reader = package_info.reader.read_by_index(file_index)?;
                // Now that we have a borrowed reader, we want to clone and make an
                // owned one that will not borrow this filesystem. To do that we
                // simply re-open the file, expecting that it was not altered.
                return reader.try_clone_with(File::open(package_path)?);
                
            }

        }

        // If the file is not found but node packages set was exhaustive, then return no
        if state == NodePackageState::Exhaustive {
            return Err(io::ErrorKind::NotFound.into());
        }

        // Try to find it in not-yet-excluded packages.
        for (package_index, (package_path, package_info)) in self.package_cache.iter_mut().enumerate() {

            // If package is already excluded, don't fetch it.
            if package_info.excluded {
                continue;
            }

            let mut names_it = package_info.reader.names().enumerate()

        }

        Err(io::ErrorKind::NotFound.into())

    }

    // /// Read a directory, the path may end with a slash.
    // pub fn read_dir(&mut self, dir_path: &str) -> io::Result<()> {

    //     let mut result = Vec::new();
    //     let dir_path = dir_path.strip_suffix('/').unwrap_or(dir_path);
        
    //     if let Some(dir_info) = self.dir_cache.get(dir_path) {

    //         for &(package_index, package_in) in &dir_info.packages {
    //             // Unwrap because this only contains index to loaded packages.
    //             let package = self.package_cache[package_index].as_mut().unwrap();
    //             package.excluded = true;

    //             for file_path in package.reader.names() {
    //                 if file_path.as_bytes().starts_with(dir_path.as_bytes()) 
    //                 && file_path.as_bytes()[dir_path.len()] == b'/' {
    //                     let child_file_path = &file_path[dir_path.len() + 1..];
    //                     if let Some((child_dir_name, _)) = child_file_path[dir_path.len() + 1..].split_once('/') {
    //                         result.push(child_file_name)
    //                     } else {

    //                     }
    //                     let (child_file_name, _) = child_file_path[dir_path.len() + 1..].split_once('/').unwrap_or((child_file_path, ""));
                        
    //                 }
    //             }

    //             // Search if any file name starts with this directory name + '/', this
    //             // will be used later to iterate the file.
    //             let first_file_index = package.reader.names()
    //                 .position(|haystack| 
    //                     haystack.starts_with(dir_path) && 
    //                     haystack.as_bytes()[dir_path.len()] == b'/');
                
    //             // If the directory is found in the package.
    //             if let Some(first_file_index) = first_file_index {

    //             }

    //         }
    //     }

    //     Ok(())

    // }

}


// pub struct ResFileReader<'a> {
//     inner: PackageFileReader<'a, File>,
// }


// /// A iterator-like (not implementor though) read dir for a resource filesystem.
// pub struct ResReadDir<'a> {
//     /// Back reference to the filesystem.
//     fs: &'a mut ResFilesystem,
// }

// pub struct ResDirEntry {
//     /// Full path of the entry.
//     name: String,
// }

// impl<'a> ResReadDir<'a> {

//     pub fn next(&mut self) {

//     }

// }

// impl ResDirEntry {

//     #[inline]
//     pub fn path(&self) -> &str {
//         &self.name
//     }

// }


/// The node cache structure.
#[derive(Debug)]
struct NodeCache {
    /// Inner file informations tree.
    inner: Vec<NodeInfo>,
}

/// Cache information about a node.
#[derive(Debug)]
struct NodeInfo {
    /// Index of the parent directory file information.
    parent: usize,
    /// Specific kind of information.
    kind: NodeInfoKind,
}

/// Kind of cached node information, absent, file or directory node.
#[derive(Debug)]
enum NodeInfoKind {
    // The file is neither a file nor a directory and known to be absent.
    Absent,
    // Information about a file.
    File {
        // Index of the package that contains the file.
        package_index: usize,
        // Index of the file within the package.
        file_index: usize,
    },
    // Information about a directory.
    Directory {
        /// Indices to packages that contains that directory, associated to true or false
        /// depending on the known presence of the directory within that package.
        packages: Vec<NodePackage>,
        /// For each named children, there is an associated index in the information tree.
        /// 
        /// FIXME: This is not really optimal for size of the enum because it break size
        /// equilibrium between File and Directory, it could be externalized into 
        /// [`FileCache`] structure.
        children: HashMap<String, usize>,
    }
}

/// This represent the known location of a node in opened packages.
#[derive(Debug, Clone)]
struct NodePackage {
    /// Index of the package in the global package cache.
    package_index: usize,
    /// Index of the first file the 
    file_index: usize,
    /// True if the package contains the node, false if not.
    present: bool,
}

/// This represent the known state of a package for a node, see the method 
/// [`NodeCache::find_node_packages()`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodePackageState {
    /// Returned packages set isn't exhaustive, it may be necessary to open more packages.
    NonExhaustive,
    /// Returned packages set is exhaustive, if the node is not found in the given 
    /// packages then it should be considered not found.
    Exhaustive,
}

impl NodeCache {

    /// Create a new default file cache.
    fn new() -> Self {
        Self {
            inner: vec![NodeInfo { // root directory is present everywhere
                parent: 0,
                kind: NodeInfoKind::Directory { 
                    packages: vec![],
                    children: HashMap::new(),
                },
            }],
        }
    }

    /// Find a packages that may contain the given node pointed to by path, from the least
    /// to the most preferred one, the vector cleared.
    fn find_node_packages(&self, path: &str, ret_packages: &mut Vec<NodePackage>) -> NodePackageState {
        
        ret_packages.clear();

        // FIXME: Not necessary to avoid duplicates because we can use 'excluded' from 'PackageInfo'
        let mut returned_package_indices = HashMap::<usize, usize>::new();

        let mut current_index = 0;
        for part in path.split('/') {
            match self.inner[current_index].kind {
                NodeInfoKind::Absent => {
                    return NodePackageState::Exhaustive;
                }
                NodeInfoKind::File { package_index, file_index } => {
                    ret_packages.push(NodePackage {
                        package_index,
                        file_index,
                        present: true,
                    });
                    return NodePackageState::Exhaustive;
                }
                NodeInfoKind::Directory { ref packages, ref children } => {

                    for node_package in packages {
                        
                        // If the index exists, we just re-add the node package at the
                        // end, so we remove it here to re-add it afterward.
                        if let Some(index) = returned_package_indices.remove(&node_package.package_index) {
                            ret_packages.remove(index);
                        }

                        returned_package_indices.insert(node_package.package_index, ret_packages.len());
                        ret_packages.push(node_package.clone());

                    }

                    if let Some(&child_index) = children.get(part) {
                        current_index = child_index;
                    }

                }
            }
        }

        NodePackageState::NonExhaustive

    }

    /// Register a file to be in the given package.
    fn put_file(&mut self, path: &str, package_index: usize, file_index: usize) {

        let mut current_index = 0;
        for part in path.split('/') {

            

        }

    }

}





    // /// Read a file from its path. The path should not start with the separator (`/`) and
    // /// empty file name should not be empty.
    // pub fn read(&mut self, file_path: &str) -> io::Result<PackageFileReader<'_, File>> {

    //     // Iteratively find (and create if not existing) directories in the tree.
    //     let mut dir_index = 0;
    //     for dir_name in dir_path.split('/') {

    //         let dir_info = &mut self.dir_cache[dir_index];

    //         if let Some(&index) = dir_info.children.get(dir_name) {
    //             // There is a subdirectory, just go in it.
    //             dir_index = index;
    //         } else {
    //             // Then we add the new dir info, and add it to its parent.
    //             let child_dir_index = self.dir_cache.len();
    //             self.dir_cache.push(DirInfo {
    //                 parent: dir_index,
    //                 children: HashMap::new(),
    //                 in_packages: Vec::new(),
    //             });
    //             self.dir_cache[dir_index].children.insert(dir_name.to_string(), child_dir_index);
    //             dir_index = child_dir_index;
    //         }

    //     }

    //     // Save the top directory index, to be restored later.
    //     let top_dir_index = dir_index;

    //     // Iteratively fetch the directory tree, from the top level dir down to root.
    //     let mut index = None;
    //     while index.is_none() {

    //         let dir_info = &mut self.dir_cache[dir_index];
    //         for &(package_index, package_in) in &dir_info.in_packages {
    //             let package_info = &mut self.package_cache[package_index];
    //             if !package_info.visited {
    //                 package_info.visited = true;
    //                 if package_in {
    //                     index = package_info.reader.index_by_name(file_name).map(|file_index| (package_index, file_index));
    //                 }
    //             }
    //         }

    //         if dir_index == 0 {
    //             break; // Because root dir has no parent.
    //         }
    //         dir_index = dir_info.parent;

    //     }

    //     // At this point, if the file is not found we want to check the remaining 
    //     // packages that are not currently known to contains the directory. We also reset
    //     // to false the visited flag, because we don't use it later.
    //     for (package_index, package_info) in self.package_cache.values_mut().enumerate() {
    //         if !package_info.visited {
    //             index = package_info.reader.index_by_name(file_name).map(|file_index| (package_index, file_index));
    //             if index.is_some() {
                    
    //                 // Here we need to add this package to the directory info.
    //                 dir_index = top_dir_index;
    //                 loop {
    //                     let dir_info = &mut self.dir_cache[dir_index];
    //                     dir_info.in_packages.push(package_index);
    //                     if dir_index == 0 {
    //                         break; // Root don't have parent directory.
    //                     }
    //                     dir_index = dir_info.parent;
    //                 }

    //                 break;
    //             }
    //         }
    //         package_info.visited = false;
    //     }

    //     // // TODO: Iteratively check in_packages for all directories down to root while not
    //     // // checking twice for the same package (using a hashset?). If the file has not 
    //     // // been found in those, check remaining packages and only then open pending 
    //     // // packages to check them.

    //     // // For each known packages, check if the file can be found, when it has been
    //     // // found, just take the package index and the file index in it.
    //     // let mut index = self.dir_tree[dir_index].in_packages.iter()
    //     //     .find_map(|&package_index| {
    //     //         let package = &self.package_cache[package_index];
    //     //         package.index_by_name(file_path).map(|file_index| (package_index, file_index))
    //     //     });

    //     // // If it has not been found, check in pending packages...
    //     // while index.is_none() {

    //     //     // If no index but the pending packages are empty, the file is not found.
    //     //     let Some(package_path) = self.package_pending.pop() else {
    //     //         return Err(io::Error::from(io::ErrorKind::NotFound));
    //     //     };

    //     //     // Try to open package and parse its header, then find the file index.
    //     //     let package = PackageReader::new(File::open(&package_path)?)?;
    //     //     let file_index = package.index_by_name(file_path);

    //     //     // Then we insert it and retrieve its index within the index map.
    //     //     let (
    //     //         package_index, 
    //     //         prev_package
    //     //     ) = self.package_cache.insert_full(package_path, package);
    //     //     debug_assert!(prev_package.is_none());

    //     //     // Set the index to exit the loop.

    //     //     // If file has been found in this package, set the index to exit the loop,
    //     //     // and then update 'dir_info' and its parents, because they all contain.
    //     //     if let Some(file_index) = file_index {

    //     //         loop {
    //     //             let dir_info = &mut self.dir_tree[dir_index];
    //     //             dir_info.in_packages.push(package_index);
    //     //             if dir_index == 0 {
    //     //                 break; // Root don't have parent directory.
    //     //             }
    //     //             dir_index = dir_info.parent;
    //     //         }

    //     //         index = Some((package_index, file_index));

    //     //     }

    //     // }

    //     // println!("self.dir_tree: {:?}", self.dir_tree);
    //     // println!("self.package_cache: {:?}", self.package_cache);

    //     // let (package_index, file_index) = index.unwrap();
    //     // let package = &mut self.package_cache[package_index];

    //     // match package.read_by_index(file_index) {
    //     //     Ok(reader) => Ok(reader),
    //     //     Err(e) if e.kind() == io::ErrorKind::NotFound => unreachable!(),
    //     //     Err(e) => Err(e)
    //     // }

    // }






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
