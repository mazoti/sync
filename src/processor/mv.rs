/// Moves a source file or source to destination file or source. Slower than OS move but safer
pub fn mv(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
    crate::processor::copy(source, destination)?;
    crate::processor::check(source, destination)?;

    #[cfg(feature = "cli")]
    crate::processor::remove_msg(source);

    if std::fs::metadata(source)?.is_file() {
        return Ok(std::fs::remove_file(source)?);
    }

    Ok(std::fs::remove_dir_all(source)?)
}
