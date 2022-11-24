//! Splits a file in n files of x bytes or
//! Join two files (append destination in source).

use std::io::{Read, Write};

/// Split file in n bytes each
pub fn split(size_bytes: &str, filepath: &str) -> Result<(), crate::processor::error::SyncError> {
    let metadata_file: std::fs::Metadata;
    let remainder_size: usize;

    let mut bytes_read: usize;
    let mut blocks_files: usize;
    let mut index: usize;
    let mut split_file: std::fs::File;
    let mut destination_file: std::fs::File;

    let mut file_count: usize = 0;
    let mut buffer = vec![0; crate::processor::consts::BUFFER_SIZE];

    let size = size_bytes.parse::<usize>()?;

    #[inline]
    fn create_file(
        filepath: &str,
        count: usize,
    ) -> Result<std::fs::File, crate::processor::error::SyncError> {
        let destination = filepath.to_owned() + "." + &count.to_string();

        #[cfg(feature = "cli")]
        crate::processor::cli::create_msg(&destination);

        Ok(std::fs::File::create(destination)?)
    }

    if size < 1 {
        return Err(crate::processor::error::SyncError {
            code: crate::processor::consts::ERROR_FILE_SIZE,
            message: crate::processor::error::error_to_string(
                crate::processor::consts::ERROR_FILE_SIZE,
            ),
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    if !std::path::Path::new(&filepath).exists() {
        return Err(crate::processor::error::SyncError {
            code: crate::processor::consts::ERROR_SOURCE_FILE,
            message: crate::processor::error::error_to_string(
                crate::processor::consts::ERROR_SOURCE_FILE,
            ),
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    metadata_file = std::fs::metadata(filepath)?;

    if !metadata_file.is_file() {
        return Err(crate::processor::error::SyncError {
            code: crate::processor::consts::ERROR_SOURCE_FILE,
            message: crate::processor::error::error_to_string(
                crate::processor::consts::ERROR_SOURCE_FILE,
            ),
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    if metadata_file.len() < 1 {
        return Err(crate::processor::error::SyncError {
            code: crate::processor::consts::ERROR_SOURCE_FILE,
            message: crate::processor::error::error_to_string(
                crate::processor::consts::ERROR_SOURCE_FILE,
            ),
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    if metadata_file.len() <= size_bytes.parse::<u64>()? {
        return Err(crate::processor::error::SyncError {
            code: crate::processor::consts::ERROR_SPLIT_SIZE,
            message: crate::processor::error::error_to_string(
                crate::processor::consts::ERROR_SPLIT_SIZE,
            ),
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    split_file = std::fs::File::open(filepath)?;

    // Each file will not fit in buffer
    if size > crate::processor::consts::BUFFER_SIZE {
        blocks_files = size / crate::processor::consts::BUFFER_SIZE;
        remainder_size = size % crate::processor::consts::BUFFER_SIZE;

        loop {
            destination_file = create_file(filepath, file_count)?;
            buffer.resize(crate::processor::consts::BUFFER_SIZE, 0);

            for _ in 0..blocks_files {
                bytes_read = split_file.read(&mut buffer)?;
                if bytes_read < crate::processor::consts::BUFFER_SIZE {
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
    remainder_size =
        crate::processor::consts::BUFFER_SIZE - (crate::processor::consts::BUFFER_SIZE % size);
    blocks_files = crate::processor::consts::BUFFER_SIZE / size;
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
