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

mod consts;
mod error;

#[cfg(feature = "cli")]
mod i18n;

mod splitjoin;
mod sync;

#[cfg(feature = "cli")]
mod validate;

/// Error class with the message and code defined in consts.rs.
/// "code" is the number returned to operating system, "message" is the error displayed in stderr,
/// "file" is the source code file and "line" is the line number of the error
pub struct SyncError {
    pub code: i32,
    pub file: &'static str,
    pub line: u32,
    pub source: Option<String>,
    pub destination: Option<String>,
}

/// Creates a config file or appends source full path + "|" + destination full path
pub fn create(source: &str, destination: &str, config: &str) -> Result<(), SyncError> {
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
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(config)?;
        writeln!(file, "{}|{}", &source, &destination)?;
        return Ok(());
    }

    // Config file exists, look on each line for source|destination full paths
    // If it doesn't find it, append to the end of the file
    let mut file = std::fs::OpenOptions::new()
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
    writeln!(file, "{}|{}", &source, &destination)?;
    Ok(())
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

/// Process all config files found in folder asynchronously
fn process_folder(
    process_function: fn(&str, &str) -> Result<(), SyncError>,
    folder: &str,
) -> Result<(), SyncError> {
    let mut thread_pool = Vec::new();
    let mut exit_code = 0i32;

    #[cfg(feature = "cli")]
    let mut display_help = true;

    for path in std::fs::read_dir(folder)? {
        let fullpath = path?.path().display().to_string();
        if !std::fs::metadata(&fullpath)?.is_dir() && fullpath.ends_with(".config") {
            #[cfg(feature = "cli")]
            {
                display_help = false;
            }

            let handle = std::thread::spawn(move || -> i32 {
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

#[cfg(feature = "cli")]
#[inline(always)]
fn datetime() -> String {
    chrono::Local::now().format("%Y-%m-%d %T").to_string()
}

/// Compares every folder, file and byte
#[inline(always)]
pub fn check(source: &str, destination: &str) -> Result<(), SyncError> {
    check::check(source, destination, consts::BUFFER_SIZE)
}

#[inline(always)]
pub fn check_file(file_path: &str) -> Result<(), SyncError> {
    process_file(check, file_path)
}

#[inline(always)]
pub fn check_folder(folder_path: &str) -> Result<(), SyncError> {
    process_folder(check, folder_path)
}

#[cfg(feature = "cli")]
#[inline(always)]
pub fn command_msgs(code: usize) -> &'static str {
    i18n::command_msgs(code)
}

/// Displays a colored "Copy" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn copy_msg(file: &str) {
    cli::copy_msg(file);
}

/// Displays a colored "(SIMULATION) Copying" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn copy_msg_simulation(file: &str) {
    cli::copy_msg_simulation(file);
}

/// Displays a colored "Create" and the folder path
#[cfg(feature = "cli")]
#[inline(always)]
fn create_msg(folder: &str) {
    cli::create_msg(folder);
}

/// Displays a colored "(SIMULATION) Create" and the folder path
#[cfg(feature = "cli")]
#[inline(always)]
fn create_msg_simulation(folder: &str) {
    cli::create_msg_simulation(folder);
}

#[cfg(feature = "cli")]
#[inline(always)]
pub fn empty(folder: &str) -> Result<(), SyncError> {
    validate::empty(folder)
}

/// Displays a colored "Empty", the file path and a message in stdout
#[cfg(feature = "cli")]
#[inline(always)]
fn empty_msg(file_folder: &str) {
    cli::empty_msg(file_folder);
}

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

/// Displays a colored "ERROR", an error message in stderr and exit with the error code.
/// If user_input is "true", waits an "enter" from user keyboard
#[cfg(feature = "cli")]
#[inline(always)]
pub fn error_msg(msg: &str, code: i32, user_input: bool) -> i32 {
    cli::error_msg(msg, code, user_input)
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

#[inline(always)]
fn error_source_file() -> i32 {
    consts::ERROR_SOURCE_FILE
}

#[inline(always)]
fn error_source_folder() -> i32 {
    consts::ERROR_SOURCE_FOLDER
}

#[inline(always)]
fn error_split_size() -> i32 {
    consts::ERROR_SPLIT_SIZE
}

#[inline(always)]
fn error_system_time() -> i32 {
    consts::ERROR_SYSTEM_TIME
}

#[inline(always)]
fn error_thread_join() -> i32 {
    consts::ERROR_THREAD_JOIN
}

#[inline(always)]
pub fn force(source: &str, destination: &str) -> Result<(), SyncError> {
    sync::force(source, destination)
}

#[inline(always)]
pub fn force_file(file_path: &str) -> Result<(), SyncError> {
    process_file(sync::force, file_path)
}

#[inline(always)]
pub fn force_folder(folder_path: &str) -> Result<(), SyncError> {
    process_folder(sync::force, folder_path)
}

/// Displays a colored "Usage", the help message in stdout and exit with NO_ERROR code
#[cfg(feature = "cli")]
#[inline(always)]
pub fn help() -> i32 {
    cli::help()
}

#[cfg(feature = "cli")]
#[inline(always)]
pub fn help_code() -> i32 {
    consts::HELP
}

#[inline(always)]
pub fn join_folder(folderpath: &str) -> Result<(), SyncError> {
    splitjoin::join(folderpath, consts::BUFFER_SIZE)
}

/// Displays a colored "Loading" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn loading_msg(file: &str) {
    cli::loading_msg(file);
}

pub fn mv(source: &str, destination: &str) -> Result<(), SyncError> {
    sync(source, destination)?;
    check(source, destination)?;

    if std::fs::metadata(source)?.is_file() {
        std::fs::remove_file(source)?;
    } else {
        std::fs::remove_dir_all(source)?;
    }

    #[cfg(feature = "cli")]
    crate::processor::remove_msg(source);

    Ok(())
}

#[cfg(feature = "cli")]
#[inline(always)]
fn msg_help() -> &'static str {
    i18n::msg_help()
}

#[inline(always)]
pub fn no_error() -> i32 {
    consts::NO_ERROR
}

/// Displays a colored "Remove" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn ok_msg(file: &str) {
    cli::ok_msg(file)
}

/// Displays a colored "1 item" and the file or folder path
#[cfg(feature = "cli")]
#[inline(always)]
fn one_item(folder: &str) {
    cli::one_item(folder);
}

#[inline(always)]
fn parse_int_error() -> i32 {
    consts::PARSE_INT_ERROR
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

/// Displays the program name, version, URL and date/time (optional)
#[cfg(feature = "cli")]
#[inline(always)]
pub fn show_header(datetime: bool) {
    cli::show_header(datetime)
}

#[cfg(feature = "cli")]
#[inline(always)]
pub fn simulate(source: &str, destination: &str) -> Result<(), SyncError> {
    sync::simulate(source, destination)
}

#[cfg(feature = "cli")]
#[inline(always)]
pub fn simulate_file(file_path: &str) -> Result<(), SyncError> {
    process_file(sync::simulate, file_path)
}

#[cfg(feature = "cli")]
#[inline(always)]
pub fn simulate_folder(config: &str) -> Result<(), SyncError> {
    process_folder(sync::simulate, config)
}

#[inline(always)]
pub fn split(size_bytes: &str, filepath: &str) -> Result<(), SyncError> {
    splitjoin::split(size_bytes, filepath, consts::BUFFER_SIZE)
}

#[inline(always)]
pub fn sync(source: &str, destination: &str) -> Result<(), SyncError> {
    sync::sync(source, destination)
}

#[inline(always)]
pub fn sync_file(config: &str) -> Result<(), SyncError> {
    process_file(sync::sync, config)
}

#[inline(always)]
pub fn sync_folder(folder_path: &str) -> Result<(), SyncError> {
    process_folder(sync::sync, folder_path)
}

/// Displays a colored "Sync" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn sync_msg(file: &str) {
    cli::sync_msg(file);
}

/// Displays a colored "(SIMULATION) Sync" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn sync_msg_simulation(file: &str) {
    cli::sync_msg_simulation(file);
}

/// Displays a colored "Update" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn update_msg(file: &str) {
    cli::update_msg(file);
}

/// Displays a colored "(SIMULATION) Update" and the file path
#[cfg(feature = "cli")]
#[inline(always)]
fn update_msg_simulation(file: &str) {
    cli::update_msg_simulation(file);
}

/// Displays a colored "Warning", the file path and a message in stdout
#[cfg(feature = "cli")]
#[inline(always)]
pub fn warning_msg(file: &str) {
    cli::warning_msg(file);
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
