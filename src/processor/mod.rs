//! A backup and synchronization tool with safe move, split/join of files, hash folder security, duplicate/empty file
//! or folder finder, file/folder content comparison and more!

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
mod mv;
mod split;
mod sync;

#[derive(Clone)]
pub enum ErrorCode {
    /// Displays all commands and how to use them
    #[cfg(feature = "i18n")]
    Help = -1,

    /// No messages to display
    NoError = 0,

    /// Source and destination already in config file
    ErrorConfigDuplicated = 1,

    /// Config file not ended in .config
    ErrorConfigExtCode = 2,

    /// Config must be a .config text file
    ErrorConfigFolderCode = 3,

    /// Cannot copy file to destination folder
    ErrorCopyFileFolder = 4,

    /// Destination file exists
    ErrorDestFile = 5,

    /// Source is a file and destination is a folder
    ErrorDestNotFile = 6,

    /// Source is a folder and destination is a file
    ErrorDestNotFolder = 7,

    /// Files or folders are different
    ErrorDiffFileFolder = 8,

    /// File size must be positive
    ErrorFileSize = 9,

    /// Input or output error
    ErrorIO = 10,

    /// Operating system string error
    ErrorOSString = 11,

    /// Cannot convert number to integer
    ErrorParseInt = 12,

    /// Cannot parse line from config file
    ErrorParseLine = 13,

    /// Source and destination are the same
    ErrorSameFileFolder = 14,

    /// Source file not found
    ErrorSourceFile = 15,

    /// Source folder not found
    ErrorSourceFolder = 16,

    /// File does not need to split
    ErrorSplitSize = 17,

    /// System time error (ex. negative time difference)
    ErrorSystemTime = 18,

    /// Cannot join thread
    ErrorThreadJoin = 19,

    /// Cannot convert number to usize
    ErrorTryFromInt = 20,
}

/// Error class with the message and code defined in consts.rs:
/// "code" is the number returned to operating system,
/// "file" is the source code file,
/// "line" is the line number of the error,
/// "source" and "destination" are the files or the folders processed by the system
pub struct SyncError {
    pub code: ErrorCode,
    pub file: &'static str,
    pub line: u32,
    pub source: Option<String>,
    pub destination: Option<String>,
}

/// The way a ErrorCode will be shown on user screen
#[cfg(feature = "i18n")]
impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Help => write!(f, "{}", i18n::msgs::HELP_MSG)?,

            ErrorCode::NoError => {}

            ErrorCode::ErrorConfigDuplicated => {
                write!(f, "{}", i18n::msgs::ERROR_CONFIG_DUPLICATED)?
            }
            ErrorCode::ErrorConfigExtCode => write!(f, "{}", i18n::msgs::ERROR_CONFIG_EXT_CODE)?,
            ErrorCode::ErrorConfigFolderCode => {
                write!(f, "{}", i18n::msgs::ERROR_CONFIG_FOLDER_CODE)?
            }
            ErrorCode::ErrorCopyFileFolder => write!(f, "{}", i18n::msgs::ERROR_COPY_FILE_FOLDER)?,
            ErrorCode::ErrorDestFile => write!(f, "{}", i18n::msgs::ERROR_DEST_FILE)?,
            ErrorCode::ErrorDestNotFile => write!(f, "{}", i18n::msgs::ERROR_DEST_NOT_FILE)?,
            ErrorCode::ErrorDestNotFolder => write!(f, "{}", i18n::msgs::ERROR_DEST_NOT_FOLDER)?,
            ErrorCode::ErrorDiffFileFolder => write!(f, "{}", i18n::msgs::ERROR_DIFF_FILE_FOLDER)?,
            ErrorCode::ErrorFileSize => write!(f, "{}", i18n::msgs::ERROR_FILE_SIZE)?,
            ErrorCode::ErrorIO => write!(f, "{}", i18n::msgs::ERROR_IO)?,
            ErrorCode::ErrorOSString => write!(f, "{}", i18n::msgs::ERROR_OSSTRING)?,
            ErrorCode::ErrorParseInt => write!(f, "{}", i18n::msgs::ERROR_PARSE_INT)?,
            ErrorCode::ErrorParseLine => write!(f, "{}", i18n::msgs::ERROR_PARSE_LINE)?,
            ErrorCode::ErrorSameFileFolder => write!(f, "{}", i18n::msgs::ERROR_SAME_FILE_FOLDER)?,
            ErrorCode::ErrorSourceFile => write!(f, "{}", i18n::msgs::ERROR_SOURCE_FILE)?,
            ErrorCode::ErrorSourceFolder => write!(f, "{}", i18n::msgs::ERROR_SOURCE_FOLDER)?,
            ErrorCode::ErrorSplitSize => write!(f, "{}", i18n::msgs::ERROR_SPLIT_SIZE)?,
            ErrorCode::ErrorSystemTime => write!(f, "{}", i18n::msgs::ERROR_SYSTEM_TIME)?,
            ErrorCode::ErrorThreadJoin => write!(f, "{}", i18n::msgs::ERROR_THREAD_JOIN)?,
            ErrorCode::ErrorTryFromInt => write!(f, "{}", i18n::msgs::ERROR_TRY_FROM_INT)?,
        }

        Ok(())
    }
}

//====================================== i18n methods in ascending order ======================================

/// Displays "elapsed:"
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn elapse_msg() -> &'static str {
    i18n::msgs::ELAPSE_MSG
}

/// Displays "started"
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn start_msg() -> &'static str {
    i18n::msgs::START_MSG
}

//====================================== cli.rs methods in ascending order ======================================

/// Displays "Copying" and the file path
#[cfg(feature = "i18n")]
#[inline(always)]
fn copy_msg(path: &str) {
    cli::copy_msg(i18n::msgs::COPY_MSG, path)
}

/// Displays "(SIMULATION) Copying" and the file path
#[cfg(feature = "i18n")]
#[inline(always)]
fn copy_msg_simulation(path: &str) {
    cli::copy_msg_simulation(i18n::msgs::SIMULATION_MSG, i18n::msgs::COPY_MSG, path)
}

/// Displays "Creating" and the folder path
#[cfg(feature = "i18n")]
#[inline(always)]
fn create_msg(path: &str) {
    cli::create_msg(i18n::msgs::CREATE_MSG, path)
}

/// Displays "(SIMULATION) Creating" and the folder path
#[cfg(feature = "i18n")]
#[inline(always)]
fn create_msg_simulation(path: &str) {
    cli::create_msg_simulation(i18n::msgs::SIMULATION_MSG, i18n::msgs::CREATE_MSG, path)
}

/// Displays "DUPLICATED" with all duplicated file paths
#[cfg(feature = "i18n")]
#[inline]
pub fn duplicate_msgs(paths: Vec<&str>) {
    cli::duplicate_msgs(i18n::msgs::DUPLICATE_MSG, paths)
}

/// Displays "Empty", the file or folder path and a message in stdout
#[cfg(feature = "i18n")]
#[inline(always)]
fn empty_msg(path: &str) {
    cli::empty_msg(i18n::msgs::EMPTY_MSG, path);
}

/// Displays "ERROR", an error message in stderr and exit with the error code.
/// If user_input is "true", waits an "enter" from user keyboard
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn error_msg(message: &str, code: i32, user_input: bool) -> i32 {
    cli::error_msg(i18n::msgs::ERROR_MSG, message, code, user_input)
}

/// Displays "Usage", the help message in stdout and exit with HELP code
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn help() -> ErrorCode {
    cli::help(i18n::msgs::USAGE_MSG, i18n::msgs::HELP_MSG, ErrorCode::Help)
}

/// Displays "Loading" and the file path
#[cfg(feature = "i18n")]
#[inline(always)]
fn loading_msg(path: &str) {
    cli::loading_msg(i18n::msgs::LOADING_MSG, path)
}

/// Displays "Ok" and the file or folder path
#[cfg(feature = "i18n")]
#[inline(always)]
fn ok_msg(path: &str) {
    cli::ok_msg(i18n::msgs::OK_MSG, path)
}

/// Displays "(ONE ITEM)" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn one_item_msg(path: &str) {
    cli::one_item_msg(i18n::msgs::ONE_ITEM_MSG, path)
}

/// Displays "Removing" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn remove_msg(path: &str) {
    cli::remove_msg(i18n::msgs::REMOVE_MSG, path)
}

/// Displays "(SIMULATION) Removing" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn remove_msg_simulation(path: &str) {
    cli::remove_msg_simulation(i18n::msgs::SIMULATION_MSG, i18n::msgs::REMOVE_MSG, path)
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
    cli::sync_msg(i18n::msgs::SYNC_MSG, path)
}

/// Displays "(SIMULATION) Sync" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn sync_msg_simulation(path: &str) {
    cli::sync_msg_simulation(i18n::msgs::SIMULATION_MSG, i18n::msgs::SYNC_MSG, path)
}

/// Displays "Updating" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn update_msg(path: &str) {
    cli::update_msg(i18n::msgs::UPDATE_MSG, path)
}

/// Displays "(SIMULATION) Updating" and the path
#[cfg(feature = "i18n")]
#[inline(always)]
fn update_msg_simulation(path: &str) {
    cli::update_msg_simulation(i18n::msgs::SIMULATION_MSG, i18n::msgs::UPDATE_MSG, path)
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
    mv::mv(source, destination)
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
    check::check_all(source, destination, consts::CHECK_BUFFER_SIZE)
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
