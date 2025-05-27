//! A data tool with advanced file management capabilities including:
//! - Safe file moving operations
//! - File splitting and joining
//! - Directory hash-based security
//! - Duplicate and empty file/folder detection
//! - Content comparison utilities
//!
//! # Features
//! - `i18n`: Enables internationalization support
//! - Default: file management functionality

#[cfg(feature = "i18n")]
mod cli;
#[cfg(feature = "i18n")]
mod duplicate;
#[cfg(feature = "i18n")]
mod i18n;
#[cfg(feature = "i18n")]
mod validate;


mod check;
mod config;
mod consts;
mod copy;
mod error;
mod hash;
mod join;
mod r#move;
mod split;
mod sync;

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorCode {
    /// Displays all commands and how to use them
    #[cfg(feature = "i18n")]
    Help = -1,

    /// No messages to display
    NoError = 0,

    /// Source and destination already in config file
    ConfigDuplicated = 1,

    /// Config file not ended in .config
    ConfigExtCode = 2,

    /// Config must be a .config text file
    ConfigFolderCode = 3,

    /// Cannot copy file to destination folder
    CopyFileFolder = 4,

    /// Destination file exists
    DestFile = 5,

    /// Source is a file and destination is a folder
    DestNotFile = 6,

    /// Source is a folder and destination is a file
    DestNotFolder = 7,

    /// Files or folders are different
    DiffFileFolder = 8,

    /// File size must be positive
    FileSize = 9,

    /// Input or output error
    IO = 10,

    /// Operating system string error
    OSString = 11,

    /// Cannot convert number to integer
    ParseInt = 12,

    /// Cannot parse line from config file
    ParseLine = 13,

    /// Source and destination are the same
    SameFileFolder = 14,

    /// Source file not found
    SourceFile = 15,

    /// Source folder not found
    SourceFolder = 16,

    /// File does not need to split
    SplitSize = 17,

    /// System time error (ex. negative time difference)
    SystemTime = 18,

    /// Cannot join thread
    ThreadJoin = 19,

    /// Cannot convert number to usize
    TryFromInt = 20,
}

/// Error class with the message and code defined in consts.rs:
/// "code" is the number returned to operating system,
/// "file" is the source code file,
/// "line" is the line number of the error,
/// "source" and "destination" are the files or the folders processed by the system
pub struct SyncError {
    pub code: ErrorCode,
    #[cfg(feature = "i18n")]
    pub file: &'static str,
    #[cfg(feature = "i18n")]
    pub line: u32,
    #[cfg(feature = "i18n")]
    pub source: Option<String>,
    #[cfg(feature = "i18n")]
    pub destination: Option<String>,
}

/// The way a ErrorCode will be shown on user screen
#[cfg(feature = "i18n")]
impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Help => write!(f, "{}", i18n::message::HELP)?,

            ErrorCode::NoError => {}

            ErrorCode::ConfigDuplicated => {
                write!(f, "{}", i18n::message::errors::CONFIG_DUPLICATED)?
            }
            ErrorCode::ConfigExtCode => {
                write!(f, "{}", i18n::message::errors::CONFIG_EXT_CODE)?
            }
            ErrorCode::ConfigFolderCode => {
                write!(f, "{}", i18n::message::errors::CONFIG_FOLDER_CODE)?
            }
            ErrorCode::CopyFileFolder => {
                write!(f, "{}", i18n::message::errors::COPY_FILE_FOLDER)?
            }
            ErrorCode::DestFile => write!(f, "{}", i18n::message::errors::DEST_FILE)?,
            ErrorCode::DestNotFile => write!(f, "{}", i18n::message::errors::DEST_NOT_FILE)?,
            ErrorCode::DestNotFolder => {
                write!(f, "{}", i18n::message::errors::DEST_NOT_FOLDER)?
            }
            ErrorCode::DiffFileFolder => {
                write!(f, "{}", i18n::message::errors::DIFF_FILE_FOLDER)?
            }
            ErrorCode::FileSize => write!(f, "{}", i18n::message::errors::FILE_SIZE)?,
            ErrorCode::IO => write!(f, "{}", i18n::message::errors::IO)?,
            ErrorCode::OSString => write!(f, "{}", i18n::message::errors::OSSTRING)?,
            ErrorCode::ParseInt => write!(f, "{}", i18n::message::errors::PARSE_INT)?,
            ErrorCode::ParseLine => write!(f, "{}", i18n::message::errors::PARSE_LINE)?,
            ErrorCode::SameFileFolder => {
                write!(f, "{}", i18n::message::errors::SAME_FILE_FOLDER)?
            }
            ErrorCode::SourceFile => write!(f, "{}", i18n::message::errors::SOURCE_FILE)?,
            ErrorCode::SourceFolder => write!(f, "{}", i18n::message::errors::SOURCE_FOLDER)?,
            ErrorCode::SplitSize => write!(f, "{}", i18n::message::errors::SPLIT_SIZE)?,
            ErrorCode::SystemTime => write!(f, "{}", i18n::message::errors::SYSTEM_TIME)?,
            ErrorCode::ThreadJoin => write!(f, "{}", i18n::message::errors::THREAD_JOIN)?,
            ErrorCode::TryFromInt => write!(f, "{}", i18n::message::errors::TRY_FROM_INT)?,
        }

        Ok(())
    }
}

//====================================== i18n methods in ascending order ======================================

/// Displays "elapsed:"
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn elapsed_msg() -> &'static str {
    i18n::message::status::ELAPSED
}

/// Displays "started"
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn start_msg() -> &'static str {
    i18n::message::status::START
}

//====================================== cli.rs methods in ascending order ======================================

/// Displays "Copying" and the file path
#[cfg(feature = "i18n")]
#[inline(always)]
fn copy_msg(path: &str) {
    cli::copy_msg(i18n::message::status::COPY, path)
}

/// Displays "(SIMULATION) Copying" and the file path
#[cfg(feature = "i18n")]
#[inline(always)]
fn copy_msg_simulation(path: &str) {
    cli::copy_msg_simulation(
        i18n::message::status::SIMULATION,
        i18n::message::status::COPY,
        path,
    )
}

/// Displays "Creating" and the folder path
#[cfg(feature = "i18n")]
#[inline(always)]
fn create_msg(path: &str) {
    cli::create_msg(i18n::message::status::CREATE, path)
}

/// Displays "(SIMULATION) Creating" and the folder path
#[cfg(feature = "i18n")]
#[inline(always)]
fn create_msg_simulation(path: &str) {
    cli::create_msg_simulation(
        i18n::message::status::SIMULATION,
        i18n::message::status::CREATE,
        path,
    )
}

/// Displays "DUPLICATED" with all duplicated file paths
#[cfg(feature = "i18n")]
#[inline]
pub fn duplicate_msgs(paths: Vec<&str>) {
    cli::duplicate_msgs(i18n::message::status::DUPLICATE, paths)
}

/// Displays "Empty", the file or folder path and a message in stdout
#[cfg(feature = "i18n")]
#[inline(always)]
fn empty_msg(path: &str) {
    cli::empty_msg(i18n::message::status::EMPTY, path);
}

/// Displays "ERROR", an error message in stderr and exit with the error code.
/// If user_input is "true", waits an "enter" from user keyboard
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn error_msg(message: &str, code: i32, user_input: bool) -> i32 {
    cli::error_msg(i18n::message::status::ERROR, message, code, user_input)
}

/// Displays "Usage", the help message in stdout and exit with HELP code
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn help() -> ErrorCode {
    cli::help(
        i18n::message::status::USAGE,
        i18n::message::HELP,
        ErrorCode::Help,
    )
}

/// Displays "Loading" and the file path
#[cfg(feature = "i18n")]
#[inline(always)]
fn loading_msg(path: &str) {
    cli::loading_msg(i18n::message::status::LOADING, path)
}

/// Displays "Ok" and the file or folder path
#[cfg(feature = "i18n")]
#[inline(always)]
fn ok_msg(path: &str) {
    cli::ok_msg(i18n::message::status::OK, path)
}

/// Displays "(ONE ITEM)" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn one_item_msg(path: &str) {
    cli::one_item_msg(i18n::message::status::ONE_ITEM, path)
}

/// Displays "Removing" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn remove_msg(path: &str) {
    cli::remove_msg(i18n::message::status::REMOVE, path)
}

/// Displays "(SIMULATION) Removing" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn remove_msg_simulation(path: &str) {
    cli::remove_msg_simulation(
        i18n::message::status::SIMULATION,
        i18n::message::status::REMOVE,
        path,
    )
}

/// Displays the program name, version, URL and the datetime (optional)
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn show_header(datetime: bool) {
    cli::show_header(datetime)
}

/// Displays "Sync" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn sync_msg(path: &str) {
    cli::sync_msg(i18n::message::status::SYNC, path)
}

/// Displays "(SIMULATION) Sync" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn sync_msg_simulation(path: &str) {
    cli::sync_msg_simulation(
        i18n::message::status::SIMULATION,
        i18n::message::status::SYNC,
        path,
    )
}

/// Displays "Updating" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn update_msg(path: &str) {
    cli::update_msg(i18n::message::status::UPDATE, path)
}

/// Displays "(SIMULATION) Updating" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn update_msg_simulation(path: &str) {
    cli::update_msg_simulation(
        i18n::message::status::SIMULATION,
        i18n::message::status::UPDATE,
        path,
    )
}

/// Compares every folder, file and byte
#[inline(always)]
pub fn check(source: &str, destination: &str) -> Result<(), SyncError> {
    check::check(source, destination, consts::CHECK_BUFFER_SIZE)
}

/// Compares every byte of two files from config to check if they are the same
#[inline(always)]
pub fn check_file(file_path: &str) -> Result<(), SyncError> {
    config::process_file(check, file_path)
}

/// Checks all .config files in parallel if there is anyone in the same folder
#[inline(always)]
pub fn check_folder(folder_path: &str) -> Result<(), SyncError> {
    config::process_folder(check, folder_path)
}

/// Copy a file from source to destination using the system function or the copy method
#[inline(always)]
pub fn copy(source: &str, destination: &str) -> Result<(), SyncError> {
    copy::copy(source, destination, consts::COPY_BUFFER_SIZE)
}

/// Creates a config file or appends full source full + "|" + full destination path
#[inline(always)]
pub fn create(source: &str, destination: &str, config: &str) -> Result<(), SyncError> {
    config::create(source, destination, config)
}

/// Displays all duplicated files found in the folder
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn duplicate(folder: &str) -> Result<(), SyncError> {
    duplicate::duplicate(folder)
}

/// Keeps copying and checking until both operations succeeds
#[inline(always)]
pub fn force(source: &str, destination: &str) -> Result<(), SyncError> {
    sync::force(source, destination)
}

/// Runs force on each file in config file
#[inline(always)]
pub fn force_file(file_path: &str) -> Result<(), SyncError> {
    config::process_file(sync::force, file_path)
}

/// Runs force in all .config files in parallel if there is anyone in the same folder
#[inline(always)]
pub fn force_folder(folder_path: &str) -> Result<(), SyncError> {
    config::process_folder(sync::force, folder_path)
}

/// Caculates path's hash and checks with hash code
#[inline(always)]
pub fn hash(hash_code: &str, path: &str) -> Result<(), SyncError> {
    hash::hash(hash_code, path, consts::HASH_BUFFER_SIZE)
}

/// Reads a hash file and checks files hashes
#[inline(always)]
pub fn hash_file(path: &str) -> Result<(), SyncError> {
    config::process_file(hash, path)
}

/// Creates a file with all file paths and hashes of the files in folder and it's subfolders
#[inline(always)]
pub fn hash_folder(folder: &str, file: &str) -> Result<(), SyncError> {
    hash::hash_folder(folder, file)
}

/// Joins all splitted files of the folder in one file of the same folder (does not delete any file)
#[inline(always)]
pub fn join_folder(folderpath: &str) -> Result<(), SyncError> {
    join::join(folderpath, consts::JOIN_BUFFER_SIZE)
}

/// Moves a source file or source to destination file or source. Slower than OS move but safer
#[inline(always)]
pub fn mv(source: &str, destination: &str) -> Result<(), SyncError> {
    r#move::mv(source, destination)
}

/// Does not synchronize, only displays the messages of what sync operations would do
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn simulate(source: &str, destination: &str) -> Result<(), SyncError> {
    sync::simulate(source, destination)
}

/// Runs simulate on each file in config file
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn simulate_file(file_path: &str) -> Result<(), SyncError> {
    config::process_file(sync::simulate, file_path)
}

/// Runs simulate in all .config files in parallel if there is anyone in the same folder
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn simulate_folder(config: &str) -> Result<(), SyncError> {
    config::process_folder(sync::simulate, config)
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

/// Runs sync on each file in config file
#[inline(always)]
pub fn sync_file(config: &str) -> Result<(), SyncError> {
    config::process_file(sync::sync, config)
}

/// Runs sync in all .config files in parallel if there is anyone in the same folder
#[inline(always)]
pub fn sync_folder(folder_path: &str) -> Result<(), SyncError> {
    config::process_folder(sync::sync, folder_path)
}

/// Displays all empty files, empty folders and folders with only one item
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn empty(folder: &str) -> Result<(), SyncError> {
    validate::empty(folder)
}

//====================================== Private methods in ascending order ======================================

/// Compares every byte of two files using a buffer
#[cfg(feature = "i18n")]
#[inline(always)]
fn compare(source: &str, destination: &str) -> Result<(), SyncError> {
    check::check_directories(source, destination, consts::CHECK_BUFFER_SIZE)
}

/// Formats a "%Y-%m-%d %T" datetime string
#[cfg(feature = "i18n")]
#[inline(always)]
fn datetime() -> String {
    chrono::Local::now().format("%Y-%m-%d %T").to_string()
}

/// Returns the hash configuration buffer size
#[cfg(feature = "i18n")]
#[inline(always)]
fn get_hash_buffer_size() -> u64 {
    consts::HASH_BUFFER_SIZE
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
            Err(err) => assert_eq!(err.code, crate::processor::ErrorCode::SourceFolder),
            Ok(_) => panic!("ERROR => src_inexistent_dest_inexistent_config_inexistent"),
        }
    }

    #[test]
    fn src_folder_dest_folder_same_config_any() {
        let folder = Folder::new("src_folder_dest_folder_same_config_any");

        match crate::processor::create(&folder.path, &folder.path, "empty.config") {
            Err(err) => assert_eq!(err.code, crate::processor::ErrorCode::SameFileFolder),
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
            Err(err) => assert_eq!(err.code, crate::processor::ErrorCode::DestNotFolder),
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
            Err(err) => assert_eq!(err.code, crate::processor::ErrorCode::DestNotFile),
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
            Err(err) => assert_eq!(err.code, crate::processor::ErrorCode::ConfigExtCode),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_config_ext_error"),
        }
    }

    #[test]
    fn src_folder_dest_folder_config_folder() {
        let src_folder = Folder::new("src_folder_dest_folder_config_folder");
        let dest_folder = Folder::new("src_folder_dest_folder_config_folder.config");

        match crate::processor::create(&src_folder.path, &dest_folder.path, &dest_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::ErrorCode::ConfigFolderCode),
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
            Err(err) => assert_eq!(err.code, crate::processor::ErrorCode::ParseLine),
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
            Err(err) => assert_eq!(err.code, crate::processor::ErrorCode::ConfigDuplicated),
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
