//! Splits a file in n files of x bytes or
//! Join files of a folder: looks for a ".0" termination, creates a new file and appends all ".n" files

use std::io::{Read, Write};

// Joins all ".n" files of the folderpath
pub fn join(folderpath: &str, buffer_size: usize) -> Result<(), crate::processor::SyncError> {
    let mut tmp: String;
    let mut read_bytes: usize;
    let mut destination_file: std::fs::File;
    let mut source_file: std::fs::File;

    let mut count: usize = 0;
    let mut destination: String = "".to_string();
    let mut buffer = vec![0; buffer_size];

    if !std::fs::metadata(folderpath)?.is_dir() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
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
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        });
    }

    #[cfg(feature = "cli")]
    crate::processor::create_msg(&destination);

    if std::path::Path::new(&destination).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_dest_file(),
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

        #[cfg(feature = "cli")]
        crate::processor::loading_msg(&tmp);

        // Append opened file to destination
        source_file = std::fs::File::open(&tmp)?;
        loop {
            read_bytes = source_file.read(&mut buffer)?;

            // Last block
            if read_bytes < buffer_size {
                buffer.truncate(read_bytes);
                destination_file.write_all(&buffer)?;
                buffer.resize(buffer_size, 0);
                break;
            }

            destination_file.write_all(&buffer)?;
        }
        count += 1;
    }

    Ok(())
}

/// Split file in n bytes each
pub fn split(
    size_bytes: &str,
    filepath: &str,
    buffer_size: usize,
) -> Result<(), crate::processor::SyncError> {
    let remainder_size: usize;

    let mut bytes_read: usize;
    let mut blocks_files: usize;
    let mut index: usize;
    let mut split_file: std::fs::File;
    let mut destination_file: std::fs::File;

    let mut file_count: usize = 0;
    let mut buffer = vec![0; buffer_size];

    let size = size_bytes.parse::<usize>()?;

    #[inline(always)]
    fn create_file(
        filepath: &str,
        count: usize,
    ) -> Result<std::fs::File, crate::processor::SyncError> {
        let destination = filepath.to_owned() + "." + &count.to_string();

        #[cfg(feature = "cli")]
        crate::processor::create_msg(&destination);

        if std::path::Path::new(&destination).exists() {
            return Err(crate::processor::SyncError {
                code: crate::processor::error_dest_file(),
                file: file!(),
                line: line!(),
                source: None,
                destination: Some(destination),
            });
        }

        Ok(std::fs::File::create(destination)?)
    }

    if size < 1 {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_file_size(),
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    let metadata_file = std::fs::metadata(filepath)?;

    if !metadata_file.is_file() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    if metadata_file.len() < 1 {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    if metadata_file.len() <= size_bytes.parse::<u64>()? {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_split_size(),
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    split_file = std::fs::File::open(filepath)?;

    // Each file will not fit in buffer
    if size > buffer_size {
        blocks_files = size / buffer_size;
        remainder_size = size % buffer_size;

        loop {
            destination_file = create_file(filepath, file_count)?;
            buffer.resize(buffer_size, 0);

            for _ in 0..blocks_files {
                bytes_read = split_file.read(&mut buffer)?;
                if bytes_read < buffer_size {
                    buffer.truncate(bytes_read);
                    destination_file.write_all(&buffer)?;
                    return Ok(());
                }
                destination_file.write_all(&buffer)?;
            }

            // Last block
            buffer.truncate(remainder_size);
            bytes_read = split_file.read(&mut buffer)?;

            buffer.truncate(bytes_read);
            destination_file.write_all(&buffer)?;

            file_count += 1;
        }
    }

    // Each file fits n times in buffer
    remainder_size = buffer_size - (buffer_size % size);
    blocks_files = buffer_size / size;
    buffer.truncate(remainder_size);

    loop {
        bytes_read = split_file.read(&mut buffer)?;
        if bytes_read < remainder_size {
            break;
        }

        for i in 0..blocks_files {
            index = i * size;
            create_file(filepath, file_count)?.write_all(&buffer[index..(index + size)])?;
            file_count += 1;
        }
    }

    // Last block
    blocks_files = bytes_read / size;

    for i in 0..blocks_files {
        index = i * size;
        create_file(filepath, file_count)?.write_all(&buffer[index..(index + size)])?;
        file_count += 1;
    }

    // Last bytes from last blocks
    create_file(filepath, file_count)?.write_all(&buffer[blocks_files..(bytes_read % size)])?;

    Ok(())
}

//====================================== Unit Tests ======================================
