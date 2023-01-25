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
