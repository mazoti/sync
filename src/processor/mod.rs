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

#[cfg(feature = "cli")]
pub mod cli;

pub mod consts;
pub mod error;

mod check;
mod sync;

/// Creates a config file or appends source full path + "|" + destination full path
pub(crate) fn create(
    source: &str,
    destination: &str,
    config: &str,
) -> Result<(), error::SyncError> {
    if !Path::new(&source).exists() {
        return Err(error::SyncError {
            code: consts::ERROR_SOURCE_FOLDER,
            message: crate::processor::error::error_to_string(consts::ERROR_SOURCE_FOLDER),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if source == destination {
        return Err(error::SyncError {
            code: consts::ERROR_SAME_FILE_FOLDER,
            message: crate::processor::error::error_to_string(consts::ERROR_SAME_FILE_FOLDER),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if Path::new(&destination).exists() {
        if Path::new(&source).is_dir() && !Path::new(&destination).is_dir() {
            return Err(error::SyncError {
                code: consts::ERROR_DEST_NOT_FOLDER,
                message: crate::processor::error::error_to_string(consts::ERROR_DEST_NOT_FOLDER),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        if Path::new(&source).is_file() && !Path::new(&destination).is_file() {
            return Err(error::SyncError {
                code: consts::ERROR_DEST_NOT_FILE,
                message: crate::processor::error::error_to_string(consts::ERROR_DEST_NOT_FILE),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }
    }
    // Config files must end with .config
    if !config.ends_with(".config") {
        return Err(error::SyncError {
            code: consts::ERROR_CONFIG_EXT_CODE,
            message: crate::processor::error::error_to_string(consts::ERROR_CONFIG_EXT_CODE),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if Path::new(&config).is_dir() {
        return Err(error::SyncError {
            code: consts::ERROR_CONFIG_FOLDER_CODE,
            message: crate::processor::error::error_to_string(consts::ERROR_CONFIG_FOLDER_CODE),
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
            .open(&config)?;
        writeln!(file, "{}|{}", &source, &destination)?;
        return Ok(());
    }

    // Config file exists, look on each line for source|destination full paths
    // If it doesn't find it, append to the end of the file
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&config)?;
    for line in BufReader::new(&file).lines() {
        let data = line?;
        let path: Vec<&str> = data.split('|').collect();
        if path.len() != 2 {
            return Err(error::SyncError {
                code: consts::ERROR_PARSE_LINE,
                message: crate::processor::error::error_to_string(consts::ERROR_PARSE_LINE),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        if path[0] == source && path[1] == destination
            || path[0] == destination && path[1] == source
        {
            return Err(error::SyncError {
                code: consts::ERROR_CONFIG_DUPLICATED,
                message: crate::processor::error::error_to_string(consts::ERROR_CONFIG_DUPLICATED),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        #[cfg(feature = "cli")]
        {
            if path[0] == source || path[1] == source {
                cli::warning_msg(source);
            }
            if path[0] == destination || path[1] == destination {
                cli::warning_msg(destination);
            }
        }
    }

    // source|destination not found, append in config file
    writeln!(file, "{}|{}", &source, &destination)?;
    Ok(())
}

/// Process all sources to destinations found in the config file
fn process_file(
    process_function: fn(&str, &str) -> Result<(), crate::processor::error::SyncError>,
    config: &str,
) -> Result<(), crate::processor::error::SyncError> {
    #[cfg(feature = "cli")]
    crate::processor::cli::loading_msg(config);

    // Parse source and destination paths from config file
    for line in BufReader::new(std::fs::File::open(&config)?).lines() {
        let data = line?;
        let path: Vec<&str> = data.split('|').collect();
        if path.len() != 2 {
            return Err(crate::processor::error::SyncError {
                code: crate::processor::consts::ERROR_PARSE_LINE,
                message: crate::processor::error::error_to_string(
                    crate::processor::consts::ERROR_PARSE_LINE,
                ),
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
    process_function: fn(&str, &str) -> Result<(), crate::processor::error::SyncError>,
    folder: &str,
) -> Result<(), crate::processor::error::SyncError> {
    let mut thread_pool = Vec::new();
    let mut exit_code = 0i32;
    let mut display_help = true;

    for path in std::fs::read_dir(folder)? {
        let fullpath = path?.path().display().to_string();
        if !std::fs::metadata(&fullpath)?.is_dir() && fullpath.ends_with(".config") {
            display_help = false;

            let handle = std::thread::spawn(move || -> i32 {
                if let Err(err) = process_file(process_function, &fullpath) {
                    return err.code;
                }
                crate::processor::consts::NO_ERROR
            });

            thread_pool.push(handle);
        }
    }

    for handle in thread_pool {
        match handle.join() {
            Err(_) => {
                return Err(crate::processor::error::SyncError {
                    code: crate::processor::consts::ERROR_THREAD_JOIN,
                    message: crate::processor::error::error_to_string(
                        crate::processor::consts::ERROR_THREAD_JOIN,
                    ),
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
                    crate::processor::cli::error_msg(
                        crate::processor::consts::ERROR_MSGS[value as usize],
                        0,
                        true,
                    );
                }
            }
        }
    }

    if exit_code == 0 {
        if display_help {
            return Err(crate::processor::error::SyncError {
                code: crate::processor::consts::HELP,
                message: crate::processor::error::error_to_string(exit_code),
                file: file!(),
                line: line!(),
                source: None,
                destination: None,
            });
        }
        return Ok(());
    }

    Err(crate::processor::error::SyncError {
        code: exit_code,
        message: crate::processor::error::error_to_string(exit_code),
        file: file!(),
        line: line!(),
        source: None,
        destination: None,
    })
}

#[inline(always)]
pub fn check(source: &str, destination: &str) -> Result<(), crate::processor::error::SyncError> {
    crate::processor::check::check(source, destination)
}

#[inline(always)]
pub fn check_file(file_path: &str) -> Result<(), crate::processor::error::SyncError> {
    process_file(crate::processor::check::check, file_path)
}

#[inline(always)]
pub fn check_folder(folder_path: &str) -> Result<(), crate::processor::error::SyncError> {
    process_folder(crate::processor::check::check, folder_path)
}

#[inline(always)]
pub fn force(source: &str, destination: &str) -> Result<(), crate::processor::error::SyncError> {
    crate::processor::sync::force(source, destination)
}

#[inline(always)]
pub fn force_file(file_path: &str) -> Result<(), crate::processor::error::SyncError> {
    process_file(crate::processor::sync::force, file_path)
}

#[inline(always)]
pub fn force_folder(folder_path: &str) -> Result<(), crate::processor::error::SyncError> {
    process_folder(crate::processor::sync::force, folder_path)
}

#[cfg(feature = "cli")]
#[inline(always)]
pub fn simulate(source: &str, destination: &str) -> Result<(), crate::processor::error::SyncError> {
    crate::processor::sync::simulate(source, destination)
}

#[cfg(feature = "cli")]
#[inline(always)]
pub fn simulate_file(file_path: &str) -> Result<(), crate::processor::error::SyncError> {
    process_file(crate::processor::sync::simulate, file_path)
}

#[cfg(feature = "cli")]
#[inline(always)]
pub fn simulate_folder(config: &str) -> Result<(), crate::processor::error::SyncError> {
    process_folder(crate::processor::sync::simulate, config)
}

#[inline(always)]
pub fn sync(source: &str, destination: &str) -> Result<(), crate::processor::error::SyncError> {
    crate::processor::sync::sync(source, destination)
}

#[inline(always)]
pub fn sync_file(config: &str) -> Result<(), crate::processor::error::SyncError> {
    process_file(crate::processor::sync::sync, config)
}

#[inline(always)]
pub fn sync_folder(folder_path: &str) -> Result<(), crate::processor::error::SyncError> {
    process_folder(crate::processor::sync::sync, folder_path)
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
            Err(err) => assert_eq!(err.code, super::consts::ERROR_SOURCE_FOLDER),
            Ok(_) => panic!("ERROR => src_inexistent_dest_inexistent_config_inexistent"),
        }
    }

    #[test]
    fn src_folder_dest_folder_same_config_any() {
        let folder = Folder::new("src_folder_dest_folder_same_config_any");

        match crate::processor::create(&folder.path, &folder.path, "empty.config") {
            Err(err) => assert_eq!(err.code, super::consts::ERROR_SAME_FILE_FOLDER),
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
            Err(err) => assert_eq!(err.code, super::consts::ERROR_DEST_NOT_FOLDER),
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
            Err(err) => assert_eq!(err.code, super::consts::ERROR_DEST_NOT_FILE),
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
            Err(err) => assert_eq!(err.code, super::consts::ERROR_CONFIG_EXT_CODE),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_config_ext_error"),
        }
    }

    #[test]
    fn src_folder_dest_folder_config_folder() {
        let src_folder = Folder::new("src_folder_dest_folder_config_folder");
        let dest_folder = Folder::new("src_folder_dest_folder_config_folder.config");

        match crate::processor::create(&src_folder.path, &dest_folder.path, &dest_folder.path) {
            Err(err) => assert_eq!(err.code, super::consts::ERROR_CONFIG_FOLDER_CODE),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_config_folder"),
        }
    }

    #[test]
    fn src_folder_dest_folder_config_new() -> Result<(), crate::processor::error::SyncError> {
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
            Err(err) => assert_eq!(err.code, super::consts::ERROR_PARSE_LINE),
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
            Err(err) => assert_eq!(err.code, super::consts::ERROR_CONFIG_DUPLICATED),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_config_exists_src_dest"),
        }
    }

    #[test]
    fn src_folder_dest_folder_config_append_data() -> Result<(), crate::processor::error::SyncError>
    {
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
