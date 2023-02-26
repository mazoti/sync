//! Join files of a folder: looks for a ".0" termination, creates a new file and appends all ".n" files

use std::io::{Read, Write};

/// Joins all ".n" files of the folder path where n is integer and starts with 0
pub fn join(folderpath: &str, buffer_size: u64) -> Result<(), crate::processor::SyncError> {
    let mut tmp: String;
    let mut read_bytes: usize;
    let mut destination_file: std::fs::File;
    let mut source_file: std::fs::File;

    let mut count: usize = 0;
    let mut destination: String = "".to_string();

    let buffer_usize = buffer_size.try_into()?;

    let mut buffer = vec![0; buffer_usize];

    if !std::fs::metadata(folderpath)?.is_dir() {
        return Err(crate::processor::SyncError {
            code: crate::processor::ErrorCode::ErrorSourceFolder,
            file: file!(),
            line: line!(),
            source: Some(folderpath.to_string()),
            destination: None,
        });
    }

    // Look for the first file, it ends with ".0"
    for path in std::fs::read_dir(folderpath)? {
        tmp = path?.path().display().to_string();
        if tmp.ends_with(".0") {
            destination = tmp[..tmp.len() - 2].to_string();
            break;
        }
    }

    // First file not found
    if destination.is_empty() {
        return Err(crate::processor::SyncError {
            code: crate::processor::ErrorCode::ErrorSourceFile,
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        });
    }

    #[cfg(feature = "i18n")]
    crate::processor::create_msg(
        &std::fs::canonicalize(&destination)?
            .into_os_string()
            .into_string()?,
    );

    if std::path::Path::new(&destination).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::ErrorCode::ErrorDestFile,
            file: file!(),
            line: line!(),
            source: None,
            destination: Some(destination),
        });
    }

    destination_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&destination)?;

    loop {
        tmp = String::from(&destination) + "." + &count.to_string();
        if !std::path::Path::new(&tmp).exists() {
            break;
        }

        #[cfg(feature = "i18n")]
        crate::processor::loading_msg(
            &std::fs::canonicalize(&tmp)?
                .into_os_string()
                .into_string()?,
        );

        // Append opened file to destination
        source_file = std::fs::File::open(&tmp)?;
        loop {
            read_bytes = source_file.read(&mut buffer)?;

            // Last block
            if read_bytes < buffer_usize {
                buffer.truncate(read_bytes);
                destination_file.write_all(&buffer)?;
                buffer.resize(buffer_usize, 0);
                break;
            }

            destination_file.write_all(&buffer)?;
        }
        count += 1;
    }

    Ok(())
}
