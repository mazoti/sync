//! Splits a file in n files of m bytes each: files will be generated starting with ".0" extension (file.ext.0, file.ext.1...)

use std::io::{Read, Write};

/// Creates n files of size_bytes each using a buffer of buffer_size
pub fn split(
    size_bytes: &str,
    filepath: &str,
    buffer_size: u64,
) -> Result<(), crate::processor::SyncError> {
    let remainder_size: usize;

    let mut bytes_read: usize;
    let mut blocks_files: usize;
    let mut index: usize;
    let mut split_file: std::fs::File;
    let mut destination_file: std::fs::File;

    let mut file_count: usize = 0;

    let buffer_usize = buffer_size.try_into()?;
    let mut buffer = vec![0; buffer_usize];

    let size = size_bytes.parse::<usize>()?;

    #[inline]
    fn create_file(
        filepath: &str,
        count: usize,
    ) -> Result<std::fs::File, crate::processor::SyncError> {
        let destination = filepath.to_owned() + "." + &count.to_string();

        #[cfg(feature = "i18n")]
        crate::processor::create_msg(&destination);

        if std::path::Path::new(&destination).exists() {
            return Err(crate::processor::SyncError {
                code: crate::processor::ErrorCode::ErrorDestFile,
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
            code: crate::processor::ErrorCode::ErrorFileSize,
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    let metadata_file = std::fs::metadata(filepath)?;

    // Input must be a file
    if !metadata_file.is_file() {
        return Err(crate::processor::SyncError {
            code: crate::processor::ErrorCode::ErrorSourceFile,
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    // File cannot be empty
    if metadata_file.len() < 1 {
        return Err(crate::processor::SyncError {
            code: crate::processor::ErrorCode::ErrorSourceFile,
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    // File fits on split size
    if metadata_file.len() <= size_bytes.parse::<u64>()? {
        return Err(crate::processor::SyncError {
            code: crate::processor::ErrorCode::ErrorSplitSize,
            file: file!(),
            line: line!(),
            source: Some(size_bytes.to_string()),
            destination: Some(filepath.to_string()),
        });
    }

    split_file = std::fs::File::open(filepath)?;

    // Each file will not fit in buffer
    if size > buffer_usize {
        blocks_files = size / buffer_usize;
        remainder_size = size % buffer_usize;

        loop {
            destination_file = create_file(
                &std::fs::canonicalize(filepath)?
                    .into_os_string()
                    .into_string()?,
                file_count,
            )?;
            buffer.resize(buffer_usize, 0);

            for _ in 0..blocks_files {
                bytes_read = split_file.read(&mut buffer)?;

                // Last file
                if bytes_read < buffer_usize {
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
    remainder_size = buffer_usize - (buffer_usize % size);
    blocks_files = buffer_usize / size;
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
