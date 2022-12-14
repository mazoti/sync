pub fn empty(folder: &str) -> Result<(), crate::processor::SyncError> {
    let mut count: usize;
    let mut folder_metadata: std::fs::Metadata;

    let mut fullpath: String = "".to_string();
    let folder_path = std::path::Path::new(&folder);

    if !folder_path.exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            message: crate::processor::error_to_string(crate::processor::error_source_folder()),
            file: file!(),
            line: line!(),
            source: Some(folder.to_string()),
            destination: Some(folder.to_string()),
        });
    }

    // Check file or symlink
    if !folder_path.is_dir() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            message: crate::processor::error_to_string(crate::processor::error_source_folder()),
            file: file!(),
            line: line!(),
            source: Some(folder.to_string()),
            destination: Some(folder.to_string()),
        });
    }

    count = 0;
    for path in std::fs::read_dir(folder)? {
        fullpath = path?.path().display().to_string();
        folder_metadata = std::fs::metadata(&fullpath)?;

        if folder_metadata.is_dir() {
            empty(&fullpath)?;
            count += 1;
            continue;
        }

        if folder_metadata.is_file() && folder_metadata.len() == 0 {
            crate::processor::empty_msg(&fullpath);
        }

        count += 1;
    }

    if count == 0 {
        crate::processor::empty_msg(&fullpath);
        return Ok(());
    }

    if count == 1 {
        crate::processor::one_item(&fullpath);
    }

    Ok(())
}
