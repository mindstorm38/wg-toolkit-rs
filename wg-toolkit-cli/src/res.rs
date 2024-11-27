use std::io::{self, Write};
use std::path::PathBuf;
use std::fs::File;

use wgtk::res::{ResFilesystem, ResReadDir, ResReadFile};
use wgtk::util::SizeFmt;

use crate::{CliOptions, CliResult, ResArgs, ResCommand, ResCopyArgs, ResDokanArgs, ResListArgs, ResReadArgs};


/// Entrypoint.
pub fn cmd_res(opts: CliOptions, args: ResArgs) -> CliResult<()> {

    let fs = ResFilesystem::new(args.dir)
        .map_err(|e| format!("Failed to open resource filesystem, reason: {e}"))?;

    match args.cmd {
        ResCommand::List(args) => cmd_res_list(opts, args, &fs),
        ResCommand::Read(args) => cmd_res_read(opts, args, &fs),
        ResCommand::Copy(args) => cmd_res_copy(opts, args, &fs),
        ResCommand::Dokan(args) => cmd_res_dokan(opts, args, &fs),
    }

}

fn cmd_res_list(opts: CliOptions, args: ResListArgs, fs: &ResFilesystem) -> CliResult<()> {
    
    let path = args.path.as_str();
    let recurse = args.recurse.unwrap_or(Some(0)).unwrap_or(u16::MAX);

    let mut indent = String::new();
    let mut output = io::stdout().lock();

    print_dir(&mut output, fs, &mut indent, path, recurse, opts.human)
        .map_err(|e| format!("Can't find '{path}' resource directory, reason: {e}"))?;

    Ok(())

}

fn cmd_res_read(opts: CliOptions, args: ResReadArgs, fs: &ResFilesystem) -> CliResult<()> {

    let path = args.path.as_str();

    if opts.human {
        print!("Opening filesystem...\r");
        let _ = io::stdout().flush();
    }

    let mut read_file = fs.read(path)
        .map_err(|e| format!("Can't find '{path}' resource file, reason: {e}"))?;

    if opts.human {
        print!("                     \r");
    }

    io::copy(&mut read_file, &mut io::stdout().lock())
        .map_err(|e| format!("Failed to print file content to stdout, reason: {e}"))?;

    Ok(())

}

fn cmd_res_copy(_opts: CliOptions, args: ResCopyArgs, fs: &ResFilesystem) -> CliResult<()> {

    if !args.dest.is_dir() {
        return Err(format!("Destination directory {:?} does not exists.", args.dest));
    }

    // Internal function to copy a single file from its reader to destination path.
    // Source path is only used for printing.
    fn copy_file(mut read_file: ResReadFile, dest_path: PathBuf, source: &str) -> CliResult<()> {

        println!("{source}...");

        let mut dest_file = File::create(&dest_path)
            .map_err(|e| format!("Failed to create file to copy at {dest_path:?}, reason: {e}"))?;

        io::copy(&mut read_file, &mut dest_file)
            .map_err(|e| format!("Failed to copy file from '{source}' to {dest_path:?}, reason: {e}"))?;

        Ok(())

    }

    // Internal function to recursively copy a directory. Source path should not have
    // a trailing separator.
    fn copy_dir(fs: &ResFilesystem, read_dir: ResReadDir, source: &mut String, dest_path: PathBuf) -> CliResult<()> {

        println!("{source}/...");

        match std::fs::create_dir(&dest_path) {
            Ok(()) => {}
            Err(_) if dest_path.is_dir() => {} // Ignore if directory already exists.
            Err(e) => return Err(format!("Failed to create directory to copy in {dest_path:?}, reason: {e}")),
        }

        for entry in read_dir {

            let entry = entry.map_err(|e| format!("Failed to read entry, reason: {e}"))?;
            let entry_dest_path = dest_path.join(entry.name());
            
            let source_backup_len = source.len();
            source.push('/');
            source.push_str(entry.name());

            if entry.stat().is_dir() {
                
                let read_dir = fs.read_dir(&source)
                    .map_err(|e| format!("Failed to read directory entry '{source}', reason: {e}"))?;

                copy_dir(fs, read_dir, source, entry_dest_path)?;

            } else {

                let read_file = fs.read(&source)
                    .map_err(|e| format!("Failed to read a directory entry '{source}', reason: {e}"))?;

                copy_file(read_file, entry_dest_path, &source)?;

            }

            source.truncate(source_backup_len);

        }

        Ok(())

    }

    for source in args.source {

        // Extract the file name from the path, used if successfully copying.
        let file_name = source
            .strip_suffix('/').unwrap_or(&source)
            .rsplit_once('/').map(|(_, s)| s).unwrap_or(&source);

        let dest_path = args.dest.join(file_name);

        // Start by trying the path as a file (it will instantly fail if there is a 
        // leading or trailing separator anyway).
        if let Ok(read_file) = fs.read(&source) {
            copy_file(read_file, dest_path, &source)?;
            continue;
        }
        
        // The error here is generic because we don't know the expected type of entry.
        let read_dir = fs.read_dir(&source)
            .map_err(|e| format!("Can't find '{source}' resource file or directory to copy, reason: {e}"))?;

        // Make source mutable because we'll use it to print advancement and we want to
        // avoid string reallocation in loop...
        let mut source = source;
        if source.ends_with('/') {
            source.truncate(source.len() - 1);
        }

        copy_dir(fs, read_dir, &mut source, dest_path)?;

    }

    Ok(())

}

/// Print directory content
fn print_dir(output: &mut impl Write, fs: &ResFilesystem, indent: &mut String, dir_path: &str, recursion: u16, human: bool) -> io::Result<()> {

    if human && indent.is_empty() {
        let _ = write!(output, "Opening filesystem...\r");
        let _ = io::stdout().flush();
    }

    let mut list = fs.read_dir(dir_path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    if human && indent.is_empty() {
        let _ = write!(output, "                     \r");
    }

    list.sort_by_cached_key(|e| e.name().to_lowercase());

    let max_size;
    if human {
        max_size = list.iter()
            .map(|entry| entry.name().len())
            .max()
            .unwrap_or(0);
    } else {
        max_size = 0;
    }

    for entry in list {

        let entry_path = entry.path();

        if entry.stat().is_dir() {
            let _ = writeln!(output, "{indent}{}/", entry.name());
        } else if human { 
            let _ = writeln!(output, "{indent}{:<2$}  {}", entry.name(), SizeFmt(entry.stat().size()), max_size);
        } else {
            let _ = writeln!(output, "{indent}{} {}", entry.name(), entry.stat().size());
        }

        if recursion > 0 {
            indent.push_str("  ");
            let _ = print_dir(output, fs, indent, &entry_path, recursion - 1, human);
            indent.truncate(indent.len() - 2);
        }

    }

    Ok(())

}

fn cmd_res_dokan(_opts: CliOptions, args: ResDokanArgs, fs: &ResFilesystem) -> CliResult<()> {
    
    let handler = dokan_impl::Handler::new(fs.clone());

    let mount_point = widestring::U16CString::from_str(&args.mount_path)
        .map_err(|_| format!("Invalid mount point containing nul-character!"))?;

    let mount_options = dokan::MountOptions {
        ..Default::default()
    };

    dokan::init();
    
    let mut mounter = dokan::FileSystemMounter::new(&handler, &mount_point, &mount_options);

    let mounted = mounter.mount()
        .map_err(|e| format!("Failed to mount Dokan filesystem: {e}"))?;

    drop(mounted);

    dokan::shutdown();

    Ok(())

}

/// Internal module for isolating implementation of the dokan filesystem.
mod dokan_impl {

    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::SystemTime;
    use std::sync::Mutex;
    use std::io::{self, Read, Seek};

    use winapi::shared::ntstatus;
    use winapi::um::winnt;

    use dokan::{CreateFileInfo, DiskSpaceInfo, FileInfo, FileSystemHandler, FillDataError, FillDataResult, FindData, OperationInfo, OperationResult, VolumeInfo};
    use widestring::{U16CStr, U16CString};
    
    use wgtk::res::{ResFilesystem, ResReadDir, ResReadFile};

    pub struct Handler {
        pub fs: ResFilesystem,
        pub next_file_index: AtomicU64,
        pub file_indices: Mutex<HashMap<String, u64>>,
    }

    #[derive(Debug)]
    pub enum Context {
        File {
            file_path: String,
            file_index: u64,
            handle: ResReadFile,
            current_offset: u64,
            size: u64,
        },
        Dir {
            file_path: String,
            file_index: u64,
            handle: ResReadDir,
        },
    }

    impl Handler {
        pub fn new(fs: ResFilesystem) -> Self {
            Self {
                fs,
                next_file_index: AtomicU64::new(1),
                file_indices: Mutex::new(HashMap::new()),
            }
        }
    }

    impl<'c, 'h: 'c> FileSystemHandler<'c, 'h> for Handler {
        
        type Context = Mutex<Context>;

        fn create_file(
            &'h self,
            file_name: &U16CStr,
            _security_context: &dokan::IO_SECURITY_CONTEXT,
            _desired_access: winnt::ACCESS_MASK,
            _file_attributes: u32,
            _share_access: u32,
            create_disposition: u32,
            create_options: u32,
            _info: &mut dokan::OperationInfo<'c, 'h, Self>,
        ) -> OperationResult<CreateFileInfo<Self::Context>> {
            
            if create_disposition != dokan_sys::win32::FILE_OPEN {
                eprintln!("Invalid create_disposition: 0x{create_disposition:04X} (expecting FILE_OPEN)");
                return Err(ntstatus::STATUS_ACCESS_DENIED);
            }

            if create_options & dokan_sys::win32::FILE_DELETE_ON_CLOSE != 0 {
                eprintln!("Invalid create_options: 0x{create_options:04X} (unsupported FILE_DELETE_ON_CLOSE)");
                return Err(ntstatus::STATUS_INVALID_PARAMETER);
            }

            let file_path = {
                let mut buf = String::with_capacity(file_name.len());
                for (i, ch) in char::decode_utf16(file_name.as_slice().iter().copied()).enumerate() {
                    match ch {
                        Ok('\\') if i == 0 => continue,
                        Ok('\\') => buf.push('/'),
                        Ok(ch) => buf.push(ch),
                        Err(_) => return Err(ntstatus::STATUS_OBJECT_NAME_INVALID),
                    }
                }
                buf
            };

            match self.fs.stat(&file_path) {
                Ok(stat) => {

                    let file_index = *self.file_indices.lock().unwrap().entry(file_path.clone())
                        .or_insert_with(|| self.next_file_index.fetch_add(1, Ordering::Relaxed));

                    if stat.is_file() {

                        if create_options & dokan_sys::win32::FILE_DIRECTORY_FILE != 0 {
                            return Err(ntstatus::STATUS_NOT_A_DIRECTORY);
                        }

                        let file = match self.fs.read(&file_path) {
                            Ok(dir) => dir,
                            Err(e) => {
                                eprintln!("Failed to open file: {e} ({file_path})");
                                return Err(ntstatus::STATUS_UNSUCCESSFUL);
                            }
                        };

                        // eprintln!("Open file: {file_path}");

                        return Ok(CreateFileInfo {
                            context: Mutex::new(Context::File {
                                file_path,
                                file_index,
                                handle: file,
                                current_offset: 0,
                                size: stat.size(),
                            }),
                            is_dir: false,
                            new_file_created: false,
                        });

                    } else {

                        if create_options & dokan_sys::win32::FILE_NON_DIRECTORY_FILE != 0 {
                            return Err(ntstatus::STATUS_FILE_IS_A_DIRECTORY);
                        }

                        let dir = match self.fs.read_dir(&file_path) {
                            Ok(dir) => dir,
                            Err(e) => {
                                eprintln!("Failed to open dir: {e} ({file_path})");
                                return Err(ntstatus::STATUS_UNSUCCESSFUL);
                            }
                        };

                        // eprintln!("Open dir: {file_path}");

                        return Ok(CreateFileInfo {
                            context: Mutex::new(Context::Dir {
                                file_path,
                                file_index,
                                handle: dir,
                            }),
                            is_dir: true,
                            new_file_created: false,
                        });

                    }

                }
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    return Err(ntstatus::STATUS_OBJECT_NAME_NOT_FOUND);
                }
                Err(e) => {
                    eprintln!("Failed to stat: {e} ({file_path})");
                    return Err(ntstatus::STATUS_UNSUCCESSFUL);
                }
            }

        }
        
        fn cleanup(
            &'h self,
            _file_name: &U16CStr,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) {
        }
        
        fn close_file(
            &'h self,
            _file_name: &U16CStr,
            _info: &OperationInfo<'c, 'h, Self>,
            context: &'c Self::Context,
        ) {
            // match *context.lock().unwrap() {
            //     Context::File { ref file_path, .. } => {
            //         eprintln!("Close file: {file_path}");
            //     }
            //     Context::Dir { ref file_path, .. } => {
            //         eprintln!("Close dir: {file_path}");
            //     }
            // }
        }
        
        fn read_file(
            &'h self,
            _file_name: &U16CStr,
            offset: i64,
            buffer: &mut [u8],
            _info: &OperationInfo<'c, 'h, Self>,
            context: &'c Self::Context,
        ) -> OperationResult<u32> {

            let Context::File { 
                ref file_path,
                ref mut handle,
                ref mut current_offset,
                ..
            } = *context.lock().unwrap() else {
                return Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST);
            };

            if offset < 0 {
                eprintln!("Negative read file offset: {offset} ({file_path})");
                return Err(ntstatus::STATUS_INVALID_PARAMETER);
            }

            let offset = offset as u64;

            if *current_offset != offset {
                match handle.seek(io::SeekFrom::Start(offset)) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to seek file: {e}");
                        return Err(ntstatus::STATUS_UNSUCCESSFUL);
                    }
                }
                *current_offset = offset;
            }

            match handle.read(buffer) {
                Ok(len) => {
                    let len = len as u32;
                    *current_offset += len as u64;
                    Ok(len)
                },
                Err(e) => {
                    eprintln!("Failed to read file: {e}");
                    return Err(ntstatus::STATUS_UNSUCCESSFUL);
                }
            }

        }
        
        fn write_file(
            &'h self,
            _file_name: &U16CStr,
            _offset: i64,
            _buffer: &[u8],
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<u32> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }
        
        fn flush_file_buffers(
            &'h self,
            _file_name: &U16CStr,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }
        
        fn get_file_information(
            &'h self,
            _file_name: &U16CStr,
            _info: &OperationInfo<'c, 'h, Self>,
            context: &'c Self::Context,
        ) -> OperationResult<FileInfo> {
            
            // return Err(ntstatus::STATUS_NOT_IMPLEMENTED);

            let (attributes, file_index, file_size) = match *context.lock().unwrap() {
                Context::File { ref file_path, file_index, size, .. } => {
                    (winnt::FILE_ATTRIBUTE_NORMAL, file_index, size)
                }
                Context::Dir { ref file_path, file_index, .. } => {
                    (winnt::FILE_ATTRIBUTE_DIRECTORY, file_index, 0)
                }
            };

            Ok(FileInfo {
                attributes,
                creation_time: SystemTime::UNIX_EPOCH,
                last_access_time: SystemTime::UNIX_EPOCH,
                last_write_time: SystemTime::UNIX_EPOCH,
                file_size,
                number_of_links: 1,
                file_index,
            })
            
        }

        fn find_files(
            &'h self,
            _file_name: &U16CStr,
            mut fill_find_data: impl FnMut(&FindData) -> FillDataResult,
            _info: &OperationInfo<'c, 'h, Self>,
            context: &'c Self::Context,
        ) -> OperationResult<()> {
            
            let Context::Dir { 
                ref file_path,
                ref mut handle,
                ..
            } = *context.lock().unwrap() else {
                return Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST);
            };

            // eprintln!("Find files: {file_path}");

            for entry in handle {

                let Ok(entry) = entry else { continue };
                let Ok(entry_path) = U16CString::from_str(entry.name()) else { continue };
                
                let stat = entry.stat();
                let find_data = FindData {
                    attributes: if stat.is_dir() {
                        winnt::FILE_ATTRIBUTE_DIRECTORY
                    } else {
                        winnt::FILE_ATTRIBUTE_NORMAL
                    },
                    creation_time: SystemTime::UNIX_EPOCH,
                    last_access_time: SystemTime::UNIX_EPOCH,
                    last_write_time: SystemTime::UNIX_EPOCH,
                    file_size: stat.size(),
                    file_name: entry_path,
                };

                match fill_find_data(&find_data) {
                    Ok(()) => continue,
                    Err(FillDataError::NameTooLong) => continue,
                    Err(FillDataError::BufferFull) => return Err(ntstatus::STATUS_BUFFER_OVERFLOW),
                }
            }

            Ok(())

        }
        
        fn find_files_with_pattern(
            &'h self,
            _file_name: &U16CStr,
            _pattern: &U16CStr,
            _fill_find_data: impl FnMut(&FindData) -> FillDataResult,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_NOT_IMPLEMENTED)
        }
        
        fn set_file_attributes(
            &'h self,
            _file_name: &U16CStr,
            _file_attributes: u32,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }
        
        fn set_file_time(
            &'h self,
            _file_name: &U16CStr,
            _creation_time: dokan::FileTimeOperation,
            _last_access_time: dokan::FileTimeOperation,
            _last_write_time: dokan::FileTimeOperation,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }
        
        fn delete_file(
            &'h self,
            _file_name: &U16CStr,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }
        
        fn delete_directory(
            &'h self,
            _file_name: &U16CStr,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }
        
        fn move_file(
            &'h self,
            _file_name: &U16CStr,
            _new_file_name: &U16CStr,
            _replace_if_existing: bool,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }
        
        fn set_end_of_file(
            &'h self,
            _file_name: &U16CStr,
            _offset: i64,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }
        
        fn set_allocation_size(
            &'h self,
            _file_name: &U16CStr,
            _alloc_size: i64,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }
        
        fn lock_file(
            &'h self,
            _file_name: &U16CStr,
            _offset: i64,
            _length: i64,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }
        
        fn unlock_file(
            &'h self,
            _file_name: &U16CStr,
            _offset: i64,
            _length: i64,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_NOT_IMPLEMENTED)
        }
        
        fn get_disk_free_space(
            &'h self,
            _info: &OperationInfo<'c, 'h, Self>,
        ) -> OperationResult<DiskSpaceInfo> {
            Ok(DiskSpaceInfo {
                byte_count: 0,
                free_byte_count: 0,
                available_byte_count: 0,
            })
        }
        
        fn get_volume_information(
            &'h self,
            _info: &OperationInfo<'c, 'h, Self>,
        ) -> OperationResult<VolumeInfo> {
            Ok(VolumeInfo {
                name: U16CString::from_str("wgtk res").unwrap(),
                serial_number: 0,
                max_component_length: 255,  // ARBITRARY
                fs_flags: 
                    winnt::FILE_CASE_PRESERVED_NAMES |
                    winnt::FILE_CASE_SENSITIVE_SEARCH |
                    winnt::FILE_UNICODE_ON_DISK |
                    winnt::FILE_READ_ONLY_VOLUME,
                fs_name: U16CString::from_str("NTFS").unwrap(),
            })
        }
        
        fn mounted(
            &'h self,
            _mount_point: &U16CStr,
            _info: &OperationInfo<'c, 'h, Self>,
        ) -> OperationResult<()> {
            Ok(())
        }
        
        fn unmounted(&'h self, _info: &OperationInfo<'c, 'h, Self>) -> OperationResult<()> {
            Ok(())
        }
        
        fn get_file_security(
            &'h self,
            _file_name: &U16CStr,
            _security_information: u32,
            _security_descriptor: winnt::PSECURITY_DESCRIPTOR,
            _buffer_length: u32,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<u32> {
            Err(ntstatus::STATUS_NOT_IMPLEMENTED)
        }
        
        fn set_file_security(
            &'h self,
            _file_name: &U16CStr,
            _security_information: u32,
            _security_descriptor: winnt::PSECURITY_DESCRIPTOR,
            _buffer_length: u32,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }
        
        fn find_streams(
            &'h self,
            _file_name: &U16CStr,
            _fill_find_stream_data: impl FnMut(&dokan::FindStreamData) -> FillDataResult,
            _info: &OperationInfo<'c, 'h, Self>,
            _context: &'c Self::Context,
        ) -> OperationResult<()> {
            Err(ntstatus::STATUS_INVALID_DEVICE_REQUEST)
        }

    }

}
