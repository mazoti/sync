//! Hash functions used by the system to find duplicate files and folder security

use std::{io::BufRead, io::BufReader, io::Write};

/// A stronger hash for the folder security
fn sha256_hash(filepath: &str) -> Result<String, crate::processor::SyncError> {
    if !std::path::Path::new(filepath).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: None,
            destination: Some(filepath.to_string()),
        });
    }

    if !std::path::Path::new(filepath).is_file() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: Some(filepath.to_string()),
            destination: None,
        });
    }

    Ok(sha256::try_digest(std::path::Path::new(filepath)).unwrap())
}

pub fn hash_file(file: &str) -> Result<(), crate::processor::SyncError> {
    if !std::path::Path::new(file).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: None,
            destination: Some(file.to_string()),
        });
    }

    if !std::path::Path::new(file).is_file() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: Some(file.to_string()),
            destination: None,
        });
    }

    Ok(println!(
        "{} => {}",
        sha256::try_digest(std::path::Path::new(file)).unwrap(),
        file
    ))
}

pub fn hash_folder(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
    fn create(
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
        Ok(writeln!(file, "{}|{}", &destination, &source)?)
    }

    fn hash(
        source_folder: &str,
        destination_file: &str,
    ) -> Result<(), crate::processor::SyncError> {
        let mut fullpath: String;
        let mut hash_str: String;
        for path in std::fs::read_dir(source_folder)? {
            fullpath = std::fs::canonicalize(path?.path())?
                .into_os_string()
                .into_string()?;
            if !std::fs::metadata(&fullpath)?.is_dir() {
                hash_str = sha256_hash(&fullpath)?;
                create(&fullpath, &hash_str, destination_file)?;
                continue;
            }

            // Create destination folder and copy directories recursively
            hash(&fullpath, destination_file)?;
        }
        Ok(())
    }

    if !std::path::Path::new(source).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if !std::path::Path::new(source).is_dir() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if std::path::Path::new(destination).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_dest_file(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    hash(source, destination)
}
