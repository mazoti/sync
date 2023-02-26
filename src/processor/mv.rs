//! Safe move: removes destination, copies source file or folder to destination, compares source with the destination
//! and removes source file or folder

/// Moves a source file or folder to destination file or folder
pub fn mv(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
    crate::processor::copy(source, destination)?;
    crate::processor::check(source, destination)?;

    #[cfg(feature = "i18n")]
    crate::processor::remove_msg(
        &std::fs::canonicalize(source)?
            .into_os_string()
            .into_string()?,
    );

    if std::fs::metadata(source)?.is_file() {
        return Ok(std::fs::remove_file(source)?);
    }

    Ok(std::fs::remove_dir_all(source)?)
}
