//! Contains the methods to process config files and folders

use std::io::{BufRead, BufReader};

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
