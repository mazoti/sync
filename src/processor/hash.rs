//! Hash functions used by the system to find duplicate files and folder security

use std::io::Write;

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

#[inline]
pub fn hash(
    hash_code: &str,
    path: &str,
    _buffer_size: u64,
) -> Result<(), crate::processor::SyncError> {
    if !(std::path::Path::new(path).exists() && std::path::Path::new(path).is_file()) {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: None,
            destination: Some(path.to_string()),
        });
    }

    if hash_code != sha256::try_digest(std::path::Path::new(path)).unwrap() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_diff_file_folder(),
            file: file!(),
            line: line!(),
            source: Some(hash_code.to_string()),
            destination: Some(path.to_string()),
        });
    }

    Ok(())
}

/// Creates a file with all paths and hashes of each file in source folder
pub fn hash_folder(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
    fn walk(
        source_folder: &str,
        mut file: &std::fs::File,
    ) -> Result<(), crate::processor::SyncError> {
        let mut fullpath: String;
        let mut hash_str: String;
        for path in std::fs::read_dir(source_folder)? {
            fullpath = std::fs::canonicalize(path?.path())?
                .into_os_string()
                .into_string()?;
            if !std::fs::metadata(&fullpath)?.is_dir() {
                hash_str = sha256_hash(&fullpath)?;
                writeln!(file, "{}|{}", &hash_str, &fullpath)?;
                continue;
            }

            // Create destination folder and copy directories recursively
            walk(&fullpath, file)?;
        }
        Ok(())
    }

    if !(std::path::Path::new(source).exists() && std::path::Path::new(source).is_dir()) {
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

    let hash_file: std::fs::File = std::fs::File::create(destination)?;
    walk(source, &hash_file)
}
