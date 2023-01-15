use std::io::{Read, Write};

/// Copies a file from source to destination like the operating system does but using a buffer with size defined in consts
pub fn copy(
    source: &str,
    destination: &str,
    buffer_size: u64,
) -> Result<(), crate::processor::SyncError> {
    let mut bytes_read: usize;
    let mut source_file: std::fs::File;
    let mut destination_file: std::fs::File;

    let buffer_usize = buffer_size.try_into()?;
    let mut buffer = vec![0; buffer_usize];

    if source == destination {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_same_file_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if !(std::path::Path::new(&source).exists() && std::path::Path::new(&source).is_file()) {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if std::path::Path::new(&destination).exists() {
        // Destination is a folder, can't remove
        if !std::path::Path::new(&destination).is_file() {
            return Err(crate::processor::SyncError {
                code: crate::processor::error_dest_not_file(),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        // Remove destination file
        std::fs::remove_file(destination)?;
    }

    source_file = std::fs::File::open(source)?;
    destination_file = std::fs::File::create(destination)?;

    // File fits in buffer
    bytes_read = source_file.read(&mut buffer)?;
    if bytes_read < buffer_usize {
        buffer.truncate(bytes_read);
        return Ok(destination_file.write_all(&buffer)?);
    }

    loop {
        destination_file.write_all(&buffer)?;
        bytes_read = source_file.read(&mut buffer)?;
        if bytes_read < buffer_usize {
            buffer.truncate(bytes_read);
            return Ok(destination_file.write_all(&buffer)?);
        }
    }
}
