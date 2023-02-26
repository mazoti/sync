//! Contains all methods to validate files and folders

/// Looks for empty files, empty folders or folders with one file or one folder only
pub fn empty(folder: &str) -> Result<(), crate::processor::SyncError> {
    let mut fullpath: String;
    let mut folder_metadata: std::fs::Metadata;

    let mut count: usize = 0;
    let folder_path = std::path::Path::new(&folder);

    // input must be a folder
    if !(folder_path.exists() && folder_path.is_dir()) {
        return Err(crate::processor::SyncError {
            code: crate::processor::ErrorCode::ErrorSourceFolder,
            file: file!(),
            line: line!(),
            source: Some(folder.to_string()),
            destination: Some(folder.to_string()),
        });
    }

    for path in std::fs::read_dir(folder)? {
        fullpath = path?.path().display().to_string();
        folder_metadata = std::fs::metadata(&fullpath)?;

        count += 1;

        if folder_metadata.is_dir() {
            empty(&fullpath)?;
            continue;
        }

        if folder_metadata.is_file() && folder_metadata.len() == 0 {
            crate::processor::empty_msg(&fullpath);
        }
    }

    if count == 0 {
        crate::processor::empty_msg(folder);
        return Ok(());
    }

    if count == 1 {
        crate::processor::one_item_msg(folder);
    }

    Ok(())
}
