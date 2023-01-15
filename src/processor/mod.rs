//! A backup and synchronization tool
//!
//! Provides three ways of synchronization
//! - Direct: sync(source: &str, destination: &str)
//!
//! Removes, adds and updates files and folders in destination
//!
//! - By config file: file(config: &str)
//!
//! A config file is a text file with .config extension
//! and each line contains the full path to source file or folder,
//! the terminator "|" and the full path to destination file or folder.
//! Ex: "/home/user/data|/home/user/backup"
//!
//! - By folder: folder(folder: &str)
//!
//! Iterates the folder processing each .config file.
//! To check each byte of the whole process:
//!
//! - check(source: &str, destination: &str)
//!
//! To synchronize and check until no errors are found or user press Crtl+C:
//!
//! - force(source: &str, destination: &str)

use std::{io::BufRead, io::BufReader, io::Write, path::Path};

mod check;

#[cfg(feature = "cli")]
mod cli;

#[cfg(feature = "copy")]
mod copy;

mod consts;

#[cfg(feature = "cli")]
mod duplicate;

mod error;

mod hash;

#[cfg(feature = "cli")]
mod i18n;

mod join;
mod split;
mod sync;

#[cfg(feature = "cli")]
mod validate;

/// Error class with the message and code defined in consts.rs.
/// "code" is the number returned to operating system,
/// "file" is the source code file,
/// "line" is the line number of the error,
/// "source" and "destination" are the files processed by the system
pub struct SyncError {
    pub code: i32,
    pub file: &'static str,
    pub line: u32,
    pub source: Option<String>,
    pub destination: Option<String>,
}

// ============================================= Public methods in ascending order ==============

/// Compares every folder, file and byte
#[inline]
pub fn check(source: &str, destination: &str) -> Result<(), SyncError> {
    #[cfg(feature = "cli")]
    {
        check::check(source, destination, consts::CHECK_BUFFER_SIZE)?;
        crate::processor::ok_msg(destination);
        Ok(())
    }

    #[cfg(not(feature = "cli"))]
    check::check(source, destination, consts::CHECK_BUFFER_SIZE)
}

/// Compares every byte of two files from config to check if they are the same
#[inline(always)]
pub fn check_file(file_path: &str) -> Result<(), SyncError> {
    process_file(check, file_path)
}

/// Compares every file and every byte of two folders from config to check if they are the same
#[inline(always)]
pub fn check_folder(folder_path: &str) -> Result<(), SyncError> {
    process_folder(check, folder_path)
}

/// Copy a file from source to destination using a system function or copy module
pub fn copy(source: &str, destination: &str) -> Result<(), SyncError> {
    #[cfg(feature = "copy")]
    {
        copy::copy(source, destination, consts::COPY_BUFFER_SIZE)?;
        let file_source = std::fs::OpenOptions::new().write(true).open(source)?;
        let file_destination = std::fs::OpenOptions::new().write(true).open(destination)?;
        file_source.set_len(file_source.metadata()?.len())?;
        Ok(file_destination.set_len(file_destination.metadata()?.len())?)
    }

    #[cfg(not(feature = "copy"))]
    {
        if std::fs::copy(source, destination)? == std::fs::metadata(source)?.len() {
            // Make the modified date the same in source and destination (Unix and Linux only)
            #[cfg(not(windows))]
            {
                let file_source = std::fs::OpenOptions::new().write(true).open(source)?;
                let file_destination = std::fs::OpenOptions::new().write(true).open(destination)?;
                file_source.set_len(file_source.metadata()?.len())?;
                file_destination.set_len(file_destination.metadata()?.len())?;
            }
            return Ok(());
        }

        Err(crate::processor::SyncError {
            code: crate::processor::error_copy_file_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        })
    }
}

/// Creates a config file or appends full source full + "|" + full destination path
pub fn create(source: &str, destination: &str, config: &str) -> Result<(), SyncError> {
    let mut file: std::fs::File;

    if !Path::new(&source).exists() {
        return Err(SyncError {
            code: error_source_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if source == destination {
        return Err(SyncError {
            code: error_same_file_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if Path::new(&destination).exists() {
        if Path::new(&source).is_dir() && !Path::new(&destination).is_dir() {
            return Err(SyncError {
                code: error_dest_not_folder(),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        if Path::new(&source).is_file() && !Path::new(&destination).is_file() {
            return Err(SyncError {
                code: error_dest_not_file(),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }
    }

    // Config files must end with .config
    if !config.ends_with(".config") {
        return Err(SyncError {
            code: error_config_ext_code(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if Path::new(&config).is_dir() {
        return Err(SyncError {
            code: error_config_folder_code(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    // Config file does not exist, create and add source|destination full paths
    if !Path::new(&config).is_file() {
        return Ok(writeln!(
            std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(config)?,
            "{}|{}",
            &source,
            &destination
        )?);
    }

    // Config file exists, look on each line for source|destination full paths
    // If it doesn't find it, append to the end of the file
    file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(config)?;
    for line in BufReader::new(&file).lines() {
        let data = line?;
        let path: Vec<&str> = data.split('|').collect();
        if path.len() != 2 {
            return Err(SyncError {
                code: error_parse_line(),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        if path[0] == source && path[1] == destination
            || path[0] == destination && path[1] == source
        {
            return Err(SyncError {
                code: error_config_duplicated(),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        #[cfg(feature = "cli")]
        {
            if path[0] == source || path[1] == source {
                warning_msg(source);
            }
            if path[0] == destination || path[1] == destination {
                warning_msg(destination);
            }
        }
    }

    // source|destination not found, append in config file
    Ok(writeln!(file, "{}|{}", &source, &destination)?)
}

/// Displays all duplicated files found in the folder
#[cfg(feature = "cli")]
#[inline(always)]
pub fn duplicate(folder: &str) -> Result<(), SyncError> {
    duplicate::duplicate(folder)
}

/// Keep copying and checking until both operations succeeds
#[inline(always)]
pub fn force(source: &str, destination: &str) -> Result<(), SyncError> {
    sync::force(source, destination)
}

/// Run force on each file in config file
#[inline(always)]
pub fn force_file(file_path: &str) -> Result<(), SyncError> {
    process_file(sync::force, file_path)
}

/// Run force on each folder in config file
#[inline(always)]
pub fn force_folder(folder_path: &str) -> Result<(), SyncError> {
    process_folder(sync::force, folder_path)
}

//===============================================================================================================

#[inline(always)]
pub fn hash_file(path: &str) -> Result<(), SyncError> {
    hash::hash_file(path)
}

#[inline(always)]
pub fn hash_folder(source: &str, destination: &str) -> Result<(), SyncError> {
    hash::hash_folder(source, destination)
}

//===============================================================================================================

/// Joins all splitted files of the folder in one file of the same folder (does not delete any file)
#[inline(always)]
pub fn join_folder(folderpath: &str) -> Result<(), SyncError> {
    join::join(folderpath, consts::JOIN_BUFFER_SIZE)
}

/// Moves a source file or source to destination file or source. Slower than OS move but safer
pub fn mv(source: &str, destination: &str) -> Result<(), SyncError> {
    sync(source, destination)?;
    check(source, destination)?;

    #[cfg(feature = "cli")]
    crate::processor::remove_msg(source);

    if std::fs::metadata(source)?.is_file() {
        return Ok(std::fs::remove_file(source)?);
    }

    Ok(std::fs::remove_dir_all(source)?)
}

/// Returns success (0) code to OS
#[inline(always)]
pub fn no_error() -> i32 {
    consts::NO_ERROR
}

/// Does not synchronize, only displays the messages of the sync operation will do
#[cfg(feature = "cli")]
#[inline(always)]
pub fn simulate(source: &str, destination: &str) -> Result<(), SyncError> {
    sync::simulate(source, destination)
}

/// Run simulate on each file in config file
#[cfg(feature = "cli")]
#[inline(always)]
pub fn simulate_file(file_path: &str) -> Result<(), SyncError> {
    process_file(sync::simulate, file_path)
}

/// Run simulate on each folder in config file
#[cfg(feature = "cli")]
#[inline(always)]
pub fn simulate_folder(config: &str) -> Result<(), SyncError> {
    process_folder(sync::simulate, config)
}

/// Splits a file in n files of size_bytes each
#[inline(always)]
pub fn split(size_bytes: &str, filepath: &str) -> Result<(), SyncError> {
    split::split(size_bytes, filepath, consts::SPLIT_BUFFER_SIZE)
}

/// Synchronizes a source file or folder with destination file or folder
#[inline(always)]
pub fn sync(source: &str, destination: &str) -> Result<(), SyncError> {
    sync::sync(source, destination)
}

/// Run sync on each file in config file
#[inline(always)]
pub fn sync_file(config: &str) -> Result<(), SyncError> {
    process_file(sync::sync, config)
}

/// Run sync on each folder in config file
#[inline(always)]
pub fn sync_folder(folder_path: &str) -> Result<(), SyncError> {
    process_folder(sync::sync, folder_path)
}

// ============================================= Public message (cli) methods in ascending order ==============

/// Displays a colored command word or words like "Creating", "Sync", "(ONE ITEM)"
#[cfg(feature = "cli")]
#[inline(always)]
pub fn command_msgs(code: usize) -> &'static str {
    i18n::command_msgs(code)
}

/// Displays the duplicate command message with all duplicate file paths
#[cfg(feature = "cli")]
#[inline]
pub fn duplicate_msgs(files: Vec<&str>) {
    for file in files {
        cli::duplicate_msg(file);
    }
    println!();
}

/// Displays all empty files, empty folders and folders with only one item
#[cfg(feature = "cli")]
#[inline(always)]
pub fn empty(folder: &str) -> Result<(), SyncError> {
    validate::empty(folder)
}

/// Displays a colored "ERROR", an error message in stderr and exit with the error code.
/// If user_input is "true", waits an "enter" from user keyboard
#[cfg(feature = "cli")]
#[inline(always)]
pub fn error_msg(msg: &str, code: i32, user_input: bool) -> i32 {
    cli::error_msg(msg, code, user_input)
}
/// Displays a colored "Usage", the help message in stdout and exit with NO_ERROR code
#[cfg(feature = "cli")]
#[inline(always)]
pub fn help() -> i32 {
    cli::help()
}

/// Returns help code to send to the system
#[cfg(feature = "cli")]
#[inline(always)]
pub fn help_code() -> i32 {
    consts::HELP
}

/// Displays the program name, version, URL and date/time (optional)
#[cfg(feature = "cli")]
#[inline(always)]
pub fn show_header(datetime: bool) {
    cli::show_header(datetime)
}

/// Displays a colored "Warning", the file path and a message in stdout
#[cfg(feature = "cli")]
#[inline(always)]
pub fn warning_msg(file: &str) {
    cli::warning_msg(file);
}
// ============================================= Private methods in ascending order ==============

/// Compares every byte of two files using a buffer
#[cfg(feature = "cli")]
#[inline(always)]
fn compare(source: &str, destination: &str) -> Result<(), SyncError> {
    check::check(source, destination, consts::CHECK_BUFFER_SIZE)
}

/// Formats a "%Y-%m-%d %T" datetime string
#[cfg(feature = "cli")]
#[inline(always)]
fn datetime() -> String {
    chrono::Local::now().format("%Y-%m-%d %T").to_string()
}

// Returns the hash configuration buffer size
#[cfg(feature = "cli")]
#[inline(always)]
fn get_hash_buffer_size() -> u64 {
    consts::HASH_BUFFER_SIZE
}

/// Process all sources to destinations found in the config file
fn process_file(
    process_function: fn(&str, &str) -> Result<(), SyncError>,
    config: &str,
) -> Result<(), SyncError> {
    #[cfg(feature = "cli")]
    loading_msg(config);

    // Parse source and destination paths from config file
    for line in BufReader::new(std::fs::File::open(config)?).lines() {
        let data = line?;
        let path: Vec<&str> = data.split('|').collect();
        if path.len() != 2 {
            return Err(SyncError {
                code: error_parse_line(),
                file: file!(),
                line: line!(),
                source: None,
                destination: None,
            });
        }

        process_function(path[0], path[1])?;
    }

    Ok(())
}

/// Process all config files found in parallel
fn process_folder(
    process_function: fn(&str, &str) -> Result<(), SyncError>,
    folder: &str,
) -> Result<(), SyncError> {
    let mut fullpath: String;
    let mut handle: std::thread::JoinHandle<i32>;

    let mut thread_pool = Vec::new();
    let mut exit_code = 0i32;

    #[cfg(feature = "cli")]
    let mut display_help = true;

    for path in std::fs::read_dir(folder)? {
        fullpath = path?.path().display().to_string();
        if !std::fs::metadata(&fullpath)?.is_dir() && fullpath.ends_with(".config") {
            #[cfg(feature = "cli")]
            {
                display_help = false;
            }

            handle = std::thread::spawn(move || -> i32 {
                if let Err(err) = process_file(process_function, &fullpath) {
                    return err.code;
                }
                no_error()
            });

            thread_pool.push(handle);
        }
    }

    for handle in thread_pool {
        match handle.join() {
            Err(_) => {
                return Err(SyncError {
                    code: error_thread_join(),
                    file: file!(),
                    line: line!(),
                    source: None,
                    destination: None,
                });
            }
            Ok(value) => {
                if value != 0 {
                    exit_code = value;
                    #[cfg(feature = "cli")]
                    error_msg(error_msgs()[value as usize], 0, true);
                }
            }
        }
    }

    if exit_code == 0 {
        #[cfg(feature = "cli")]
        if display_help {
            return Err(SyncError {
                code: help(),
                file: file!(),
                line: line!(),
                source: None,
                destination: None,
            });
        }
        return Ok(());
    }

    Err(SyncError {
        code: exit_code,
        file: file!(),
        line: line!(),
        source: None,
        destination: None,
    })
}

//====================================== Private error code methods in ascending order ======================================

#[inline(always)]
fn error_config_duplicated() -> i32 {
    consts::ERROR_CONFIG_DUPLICATED
}

#[inline(always)]
fn error_config_ext_code() -> i32 {
    consts::ERROR_CONFIG_EXT_CODE
}

#[inline(always)]
fn error_config_folder_code() -> i32 {
    consts::ERROR_CONFIG_FOLDER_CODE
}

#[cfg(not(feature = "copy"))]
#[inline(always)]
fn error_copy_file_folder() -> i32 {
    consts::ERROR_COPY_FILE_FOLDER
}

#[inline(always)]
fn error_dest_file() -> i32 {
    consts::ERROR_DEST_FILE
}

#[inline(always)]
fn error_dest_not_file() -> i32 {
    consts::ERROR_DEST_NOT_FILE
}

#[inline(always)]
fn error_dest_not_folder() -> i32 {
    consts::ERROR_DEST_NOT_FOLDER
}

#[inline(always)]
fn error_diff_file_folder() -> i32 {
    consts::ERROR_DIFF_FILE_FOLDER
}

#[inline(always)]
fn error_file_size() -> i32 {
    consts::ERROR_FILE_SIZE
}

#[inline(always)]
fn error_io() -> i32 {
    consts::ERROR_IO
}

#[cfg(feature = "cli")]
#[inline(always)]
fn error_msgs() -> &'static [&'static str] {
    i18n::error_msgs()
}

#[inline(always)]
fn error_parse_line() -> i32 {
    consts::ERROR_PARSE_LINE
}

#[inline(always)]
fn error_same_file_folder() -> i32 {
    consts::ERROR_SAME_FILE_FOLDER
}

/// Source file not found or not a file
#[inline(always)]
fn error_source_file() -> i32 {
    consts::ERROR_SOURCE_FILE
}

/// Source folder not found or not a folder
#[inline(always)]
fn error_source_folder() -> i32 {
    consts::ERROR_SOURCE_FOLDER
}

/// Any file must have a positive size to split
#[inline(always)]
fn error_split_size() -> i32 {
    consts::ERROR_SPLIT_SIZE
}

/// System time error like negative difference between two times
#[inline(always)]
fn error_system_time() -> i32 {
    consts::ERROR_SYSTEM_TIME
}

/// Thread join error (rendezvous) on processing folders
#[inline(always)]
fn error_thread_join() -> i32 {
    consts::ERROR_THREAD_JOIN
}

/// Error converting string to operating system string
#[inline(always)]
fn os_string_error() -> i32 {
    consts::ERROR_OSSTRING
}

/// Error converting string to number
#[inline(always)]
fn parse_int_error() -> i32 {
    consts::ERROR_PARSE_INT
}

/// Error converting integer to usize
#[inline(always)]
fn try_from_int_error() -> i32 {
    consts::ERROR_TRY_FROM_INT
}

//====================================== Private Message methods in ascending order ======================================

/// Displays a colored "Copy" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn copy_msg(filepath: &str) {
    cli::copy_msg(filepath);
}

/// Displays a colored "(SIMULATION) Copying" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn copy_msg_simulation(filepath: &str) {
    cli::copy_msg_simulation(filepath);
}

/// Displays a colored "Create" and the folder path
#[cfg(feature = "cli")]
#[inline(always)]
fn create_msg(folderpath: &str) {
    cli::create_msg(folderpath);
}

/// Displays a colored "(SIMULATION) Create" and the folder path
#[cfg(feature = "cli")]
#[inline(always)]
fn create_msg_simulation(folderpath: &str) {
    cli::create_msg_simulation(folderpath);
}

/// Displays a colored "Empty", the file path and a message in stdout
#[cfg(feature = "cli")]
#[inline(always)]
fn empty_msg(file_folder: &str) {
    cli::empty_msg(file_folder);
}

/// Displays a colored "Loading" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn loading_msg(filepath: &str) {
    cli::loading_msg(filepath);
}

/// Displays help in the selected language
#[cfg(feature = "cli")]
#[inline(always)]
fn msg_help() -> &'static str {
    i18n::msg_help()
}

/// Displays a colored "Remove" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn ok_msg(filepath: &str) {
    cli::ok_msg(filepath)
}

/// Displays a colored "1 item" and the folder path
#[cfg(feature = "cli")]
#[inline(always)]
fn one_item(folderpath: &str) {
    cli::one_item(folderpath);
}

/// Displays a colored "Remove" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn remove_msg(file: &str) {
    cli::remove_msg(file);
}

/// Displays a colored "(SIMULATION) Remove" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn remove_msg_simulation(file: &str) {
    cli::remove_msg_simulation(file);
}

/// Displays a colored "Sync" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn sync_msg(filepath: &str) {
    cli::sync_msg(filepath);
}

/// Displays a colored "(SIMULATION) Sync" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn sync_msg_simulation(filepath: &str) {
    cli::sync_msg_simulation(filepath);
}

/// Displays a colored "Update" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn update_msg(filepath: &str) {
    cli::update_msg(filepath);
}

/// Displays a colored "(SIMULATION) Update" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn update_msg_simulation(file: &str) {
    cli::update_msg_simulation(file);
}

//====================================== Unit Tests ======================================
#[cfg(test)]
mod tests {

    use std::io::{BufRead, BufReader, Write};

    pub struct Folder {
        pub path: String,
    }

    pub struct TextFile {
        pub path: String,
    }

    impl Folder {
        pub fn new(fullpath: &'static str) -> Folder {
            let folder = Folder {
                path: "target/".to_owned() + fullpath,
            };
            std::fs::create_dir_all(&folder.path).unwrap();
            folder
        }
    }

    impl Drop for Folder {
        fn drop(&mut self) {
            if std::path::Path::new(&self.path).exists() {
                std::fs::remove_dir_all(&self.path).unwrap();
            }
        }
    }

    impl TextFile {
        pub fn new(fullpath: &'static str, text: &[u8]) -> TextFile {
            let text_file = TextFile {
                path: "target/".to_owned() + fullpath,
            };
            let mut file = std::fs::File::create(&text_file.path).unwrap();
            file.write_all(text).unwrap();
            text_file
        }
    }

    impl Drop for TextFile {
        fn drop(&mut self) {
            if std::path::Path::new(&self.path).exists() {
                std::fs::remove_file(&self.path).unwrap();
            }
        }
    }

    #[test]
    fn src_inexistent_dest_inexistent_config_inexistent() {
        match crate::processor::create("none", "nothing", "empty.config") {
            Err(err) => assert_eq!(err.code, crate::processor::error_source_folder()),
            Ok(_) => panic!("ERROR => src_inexistent_dest_inexistent_config_inexistent"),
        }
    }

    #[test]
    fn src_folder_dest_folder_same_config_any() {
        let folder = Folder::new("src_folder_dest_folder_same_config_any");

        match crate::processor::create(&folder.path, &folder.path, "empty.config") {
            Err(err) => assert_eq!(err.code, crate::processor::error_same_file_folder()),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_same_config_any"),
        }
    }

    #[test]
    fn src_folder_dest_file_config_any() {
        let src_folder = Folder::new("src_folder_dest_file_config_any");
        let dest_file = TextFile::new("src_folder_dest_file_config_any/file.txt", b"data");

        match crate::processor::create(
            &src_folder.path,
            &dest_file.path,
            "src_folder_dest_file_config_any/config.config",
        ) {
            Err(err) => assert_eq!(err.code, crate::processor::error_dest_not_folder()),
            Ok(_) => panic!("ERROR => src_folder_dest_file_config_any"),
        }
    }

    #[test]
    fn src_file_dest_folder_config_any() {
        let dest_folder = Folder::new("src_file_dest_folder_config_any");
        let src_file = TextFile::new("src_file_dest_folder_config_any/file.txt", b"data");

        match crate::processor::create(
            &src_file.path,
            &dest_folder.path,
            "src_file_dest_folder_config_any/config.config",
        ) {
            Err(err) => assert_eq!(err.code, crate::processor::error_dest_not_file()),
            Ok(_) => panic!("ERROR => src_file_dest_folder_config_any"),
        }
    }

    #[test]
    fn src_folder_dest_folder_config_ext_error() {
        let src_folder = Folder::new("src_folder_dest_folder_config_ext_error");
        let dest_folder = Folder::new("src_folder_dest_folder_config_ext_error/destination");

        match crate::processor::create(
            &src_folder.path,
            &dest_folder.path,
            "src_folder_dest_folder_config_ext_error/config.conf",
        ) {
            Err(err) => assert_eq!(err.code, crate::processor::error_config_ext_code()),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_config_ext_error"),
        }
    }

    #[test]
    fn src_folder_dest_folder_config_folder() {
        let src_folder = Folder::new("src_folder_dest_folder_config_folder");
        let dest_folder = Folder::new("src_folder_dest_folder_config_folder.config");

        match crate::processor::create(&src_folder.path, &dest_folder.path, &dest_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::error_config_folder_code()),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_config_folder"),
        }
    }

    #[test]
    fn src_folder_dest_folder_config_new() -> Result<(), crate::processor::SyncError> {
        let src_folder = Folder::new("src_folder_dest_folder_config_new");
        let dest_folder = Folder::new("src_folder_dest_folder_config_new/destination");

        crate::processor::create(
            &src_folder.path,
            &dest_folder.path,
            "target/src_folder_dest_folder_config_new/new_config.config",
        )?;

        // Check file contents for source|destination
        let config_file =
            std::fs::File::open("target/src_folder_dest_folder_config_new/new_config.config")?;
        let reader = BufReader::new(config_file);
        let mut result = false;

        for line in reader.lines() {
            let data = line.unwrap();
            let data_config = String::from(&src_folder.path) + "|" + &dest_folder.path;
            result = data.eq(&data_config);
            break;
        }
        assert_eq!(result, true);
        Ok(())
    }

    #[test]
    fn src_folder_dest_folder_config_error_data() {
        let src_folder = Folder::new("src_folder_dest_folder_config_error_data");
        let dest_folder = Folder::new("src_folder_dest_folder_config_error_data/destination");
        let config_file = TextFile::new(
            "src_folder_dest_folder_config_error_data/error_data.config",
            b"data",
        );

        match crate::processor::create(&src_folder.path, &dest_folder.path, &config_file.path) {
            Err(err) => assert_eq!(err.code, crate::processor::error_parse_line()),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_config_error_data"),
        }
    }

    #[test]
    fn src_folder_dest_folder_config_exists_src_dest() {
        // Should find a SOURCE_TEST|destination and return an error
        let src_folder = Folder::new("src_folder_dest_folder_config_exists_src_dest");
        let dest_folder = Folder::new("src_folder_dest_folder_config_exists_src_dest/destination");
        let config_file = TextFile::new("src_folder_dest_folder_config_exists_src_dest/config.config", b"target/src_folder_dest_folder_config_exists_src_dest|target/src_folder_dest_folder_config_exists_src_dest/destination\nsource|destination");

        match crate::processor::create(&src_folder.path, &dest_folder.path, &config_file.path) {
            Err(err) => assert_eq!(err.code, crate::processor::error_config_duplicated()),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_config_exists_src_dest"),
        }
    }

    #[test]
    fn src_folder_dest_folder_config_append_data() -> Result<(), crate::processor::SyncError> {
        let src_folder = Folder::new("src_folder_dest_folder_config_append_data");
        let dest_folder = Folder::new("src_folder_dest_folder_config_append_data/destination");
        let config_file = TextFile::new(
            "src_folder_dest_folder_config_append_data/config_append_data.config",
            b"source|destination\nsource2|destination2\n",
        );

        crate::processor::create(&src_folder.path, &dest_folder.path, &config_file.path)?;

        let result_file = std::fs::File::open(
            "target/src_folder_dest_folder_config_append_data/config_append_data.config",
        )?;
        let count = BufReader::new(&result_file).lines().count();
        assert_eq!(count, 3);
        Ok(())
    }
}
