//! Hash functions used by the system for folder security

use std::io::Write;

/// Calculates the SHA256 of the filepath and returns an hexadecimal string of the hash
fn sha256_hash(filepath: &str) -> Result<String, crate::processor::SyncError> {
    if !(std::path::Path::new(filepath).exists() && std::path::Path::new(filepath).is_file()) {
        return Err(crate::processor::SyncError {
            code: crate::processor::ErrorCode::ErrorSourceFile,
            file: file!(),
            line: line!(),
            source: None,
            destination: Some(filepath.to_string()),
        });
    }

    Ok(sha256::try_digest(std::path::Path::new(filepath)).unwrap())
}

/// Compares the SHA256 hash of the hash file with the hash of the system file
#[inline]
pub fn hash(
    hash_code: &str,
    path: &str,
    _buffer_size: u64,
) -> Result<(), crate::processor::SyncError> {
    if !(std::path::Path::new(path).exists() && std::path::Path::new(path).is_file()) {
        return Err(crate::processor::SyncError {
            code: crate::processor::ErrorCode::ErrorSourceFile,
            file: file!(),
            line: line!(),
            source: None,
            destination: Some(path.to_string()),
        });
    }

    if hash_code != sha256::try_digest(std::path::Path::new(path)).unwrap() {
        return Err(crate::processor::SyncError {
            code: crate::processor::ErrorCode::ErrorDiffFileFolder,
            file: file!(),
            line: line!(),
            source: Some(hash_code.to_string()),
            destination: Some(path.to_string()),
        });
    }

    Ok(())
}

/// Creates a file with all paths and hashes of each file of the source folder and subfolders
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
            code: crate::processor::ErrorCode::ErrorSourceFolder,
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if std::path::Path::new(destination).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::ErrorCode::ErrorDestFile,
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    walk(source, &std::fs::File::create(destination)?)
}
