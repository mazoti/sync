//! Contains the methods to process config files and folders

use std::io::{BufRead, BufReader, Write};

/// Creates a config file or appends full source full + "|" + full destination path
pub fn create(
    source: &str,
    destination: &str,
    config: &str,
) -> Result<(), crate::processor::SyncError> {
    let mut file: std::fs::File;

    if !std::path::Path::new(&source).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if source == destination {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_same_file_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if std::path::Path::new(&destination).exists() {
        if std::path::Path::new(&source).is_dir() && !std::path::Path::new(&destination).is_dir() {
            return Err(crate::processor::SyncError {
                code: crate::processor::error_dest_not_folder(),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        if std::path::Path::new(&source).is_file() && !std::path::Path::new(&destination).is_file()
        {
            return Err(crate::processor::SyncError {
                code: crate::processor::error_dest_not_file(),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }
    }

    // Config files must end with .config
    if !config.ends_with(".config") {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_config_ext_code(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if std::path::Path::new(&config).is_dir() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_config_folder_code(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    // Config file does not exist, create and add source|destination full paths
    if !std::path::Path::new(&config).is_file() {
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
            return Err(crate::processor::SyncError {
                code: crate::processor::error_parse_line(),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        if path[0] == source && path[1] == destination
            || path[0] == destination && path[1] == source
        {
            return Err(crate::processor::SyncError {
                code: crate::processor::error_config_duplicated(),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        #[cfg(feature = "i18n")]
        {
            if path[0] == source || path[1] == source {
                crate::processor::warning_msg(source);
            }
            if path[0] == destination || path[1] == destination {
                crate::processor::warning_msg(destination);
            }
        }
    }

    // source|destination not found, append in config file
    Ok(writeln!(file, "{}|{}", &source, &destination)?)
}

/// Process all sources to destinations found in the config file
pub fn process_file(
    process_function: fn(&str, &str) -> Result<(), crate::processor::SyncError>,
    config: &str,
) -> Result<(), crate::processor::SyncError> {
    #[cfg(feature = "i18n")]
    crate::processor::loading_msg(config);

    // Parse source and destination paths from config file
    for line in BufReader::new(std::fs::File::open(config)?).lines() {
        let data = line?;
        let path: Vec<&str> = data.split('|').collect();
        if path.len() != 2 {
            return Err(crate::processor::SyncError {
                code: crate::processor::error_parse_line(),
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
pub fn process_folder(
    process_function: fn(&str, &str) -> Result<(), crate::processor::SyncError>,
    folder: &str,
) -> Result<(), crate::processor::SyncError> {
    let mut thread_join_error: bool;
    let mut fullpath: String;
    let mut handle: std::thread::JoinHandle<i32>;

    let mut thread_pool = Vec::new();
    let mut exit_code = 0i32;

    #[cfg(feature = "i18n")]
    let mut display_help = true;

    for path in std::fs::read_dir(folder)? {
        fullpath = path?.path().display().to_string();
        if !std::fs::metadata(&fullpath)?.is_dir() && fullpath.ends_with(".config") {
            #[cfg(feature = "i18n")]
            {
                display_help = false;
            }

            handle = std::thread::spawn(move || -> i32 {
                if let Err(err) = process_file(process_function, &fullpath) {
                    return err.code;
                }
                crate::processor::no_error()
            });

            thread_pool.push(handle);
        }
    }

    thread_join_error = false;

    for handle in thread_pool {
        match handle.join() {
            Err(_) => thread_join_error = true,
            Ok(value) => {
                if value != 0 {
                    exit_code = value;
                    #[cfg(feature = "i18n")]
                    crate::processor::error_msg(
                        crate::processor::error_msgs()[value as usize],
                        0,
                        true,
                    );
                }
            }
        }
    }

    if thread_join_error {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_thread_join(),
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        });
    }

    if exit_code == 0 {
        #[cfg(feature = "i18n")]
        if display_help {
            return Err(crate::processor::SyncError {
                code: crate::processor::help(),
                file: file!(),
                line: line!(),
                source: None,
                destination: None,
            });
        }
        return Ok(());
    }

    Err(crate::processor::SyncError {
        code: exit_code,
        file: file!(),
        line: line!(),
        source: None,
        destination: None,
    })
}
