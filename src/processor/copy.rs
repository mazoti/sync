//! Removes the destination (if it exists) and makes a copy from source.
//! Could use a buffer (if copy feature is enabled) or use operating system's copy

#[cfg(feature = "copy")]
use std::io::{Read, Write};

/// Copies a file from source to destination using the operating system's copy function or copy method copy_buffered.
/// On Linux and Unix sets the same modified date for source and destination
pub fn copy(
    source: &str,
    destination: &str,
    _buffer_size: u64,
) -> Result<(), crate::processor::SyncError> {
    #[cfg(feature = "copy")]
    #[inline(always)]
    fn feature_copy(
        source: &str,
        destination: &str,
        _buffer_size: u64,
    ) -> Result<u64, crate::processor::SyncError> {
        copy_buffered(source, destination, _buffer_size)
    }

    #[cfg(not(feature = "copy"))]
    #[inline(always)]
    fn feature_copy(
        source: &str,
        destination: &str,
        _buffer_size: u64,
    ) -> Result<u64, std::io::Error> {
        std::fs::copy(source, destination)
    }

    if feature_copy(source, destination, _buffer_size)? == std::fs::metadata(source)?.len() {
        // Make the modified date the same in source and destination (Unix and Linux only)
        //       #[cfg(not(windows))]
        //       {
        let file_source = std::fs::OpenOptions::new().write(true).open(source)?;
        let file_destination = std::fs::OpenOptions::new().write(true).open(destination)?;
        file_source.set_len(file_source.metadata()?.len())?;
        file_destination.set_len(file_destination.metadata()?.len())?;
        //      }

        return Ok(());
    }

    Err(crate::processor::SyncError {
        code: crate::processor::error_copy_file_folder(),
        file: file!(),
        line: line!(),
        source: Some(source.to_string()),
        destination: Some(destination.to_string()),
    })
}

/// Copies a file from source to destination like the operating system does but using a buffer with size defined in consts.rs
#[cfg(feature = "copy")]
fn copy_buffered(
    source: &str,
    destination: &str,
    buffer_size: u64,
) -> Result<u64, crate::processor::SyncError> {
    let mut bytes_read: usize;
    let mut file_size: u64;
    let mut file_size_result: u64;
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
    file_size = bytes_read.try_into()?;

    if bytes_read < buffer_usize {
        buffer.truncate(bytes_read);
        destination_file.write_all(&buffer)?;
        return Ok(file_size);
    }

    file_size_result = file_size;

    loop {
        destination_file.write_all(&buffer)?;
        bytes_read = source_file.read(&mut buffer)?;
        file_size = bytes_read.try_into()?;
        file_size_result += file_size;
        if bytes_read < buffer_usize {
            buffer.truncate(bytes_read);
            destination_file.write_all(&buffer)?;
            return Ok(file_size_result);
        }
    }
}
