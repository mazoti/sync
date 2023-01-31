//! Sync and simulate functions implementations

/// Displays what a sync operation would do without any modification
#[cfg(feature = "i18n")]
pub fn simulate(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
    let fullpath_destination: String;
    let fullpath_source_copy: String;
    let fullpath_destination_copy: String;

    let handle: std::thread::JoinHandle<Result<(), crate::processor::SyncError>>;
    let update_result: Result<(), crate::processor::SyncError>;

    fn copy_folder_simulation(source: &str) -> Result<(), crate::processor::SyncError> {
        let mut fullpath: String;

        for path in std::fs::read_dir(source)? {
            fullpath = path?.path().display().to_string();
            if !std::fs::metadata(&fullpath)?.is_dir() {
                crate::processor::copy_msg_simulation(&fullpath);
                continue;
            }

            // Create destination folder and copy directories recursively
            copy_folder_simulation(&fullpath)?;
        }
        Ok(())
    }

    fn update_file_simulation(
        source: &str,
        destination: &str,
    ) -> Result<(), crate::processor::SyncError> {
        if std::fs::metadata(source)?.modified()? != std::fs::metadata(destination)?.modified()? {
            crate::processor::update_msg_simulation(destination);
        }
        Ok(())
    }

    /// Iterates over source folder adding and updating files and folders in destination
    /// and removes files and folders from destination not found in source
    fn update_simulation(
        source: &str,
        destination: &str,
    ) -> Result<(), crate::processor::SyncError> {
        let mut fullpath_source: String;
        let mut fullpath_destination: String;
        let mut file_folder: std::io::Result<std::fs::Metadata>;

        for path in std::fs::read_dir(source)? {
            fullpath_source = path?.path().display().to_string();
            fullpath_destination = fullpath_source.replace(source, destination);
            file_folder = std::fs::metadata(&fullpath_destination);

            if !std::fs::metadata(&fullpath_source)?.is_dir() {
                match file_folder {
                    Err(_) => {
                        crate::processor::copy_msg_simulation(&fullpath_destination);
                        return Ok(());
                    }
                    Ok(_) => update_file_simulation(&fullpath_source, &fullpath_destination)?,
                }
                continue;
            }

            // Folder does not exist
            if file_folder.is_err() {
                crate::processor::create_msg_simulation(&fullpath_destination);
                copy_folder_simulation(&fullpath_source)?;
                continue;
            }
            update_simulation(&fullpath_source, &fullpath_destination)?;
        }
        Ok(())
    }

    /// Iterate over destination folder and remove files and folders that doesn't exists in source
    fn remove_simulation(
        source: &str,
        destination: &str,
    ) -> Result<(), crate::processor::SyncError> {
        let mut fullpath_destination: String;
        let mut fullpath_source: String;
        let mut not_found: bool;

        for path in std::fs::read_dir(destination)? {
            fullpath_destination = path?.path().display().to_string();
            fullpath_source = fullpath_destination.replace(destination, source);
            not_found = !std::path::Path::new(&fullpath_source).exists();

            // File not found in source, remove in destination
            if !std::fs::metadata(&fullpath_destination)?.is_dir() {
                if not_found {
                    crate::processor::remove_msg_simulation(&fullpath_destination);
                }
                continue;
            }

            // Directory not found in source, remove in destination
            if not_found {
                crate::processor::remove_msg_simulation(&fullpath_destination);
                continue;
            }
            remove_simulation(&fullpath_source, &fullpath_destination)?;
        }
        Ok(())
    }

    if !std::path::Path::new(&source).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if source == destination {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_same_file_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    let fullpath_source = std::fs::canonicalize(source)?
        .into_os_string()
        .into_string()?;

    if std::path::Path::new(&source).is_dir() {
        if !std::path::Path::new(&destination).exists() {
            crate::processor::create_msg_simulation(destination);

            fullpath_destination = std::fs::canonicalize(source)?
                .into_os_string()
                .into_string()?;

            crate::processor::copy_msg_simulation(&fullpath_destination);
            return copy_folder_simulation(&fullpath_source);
        }

        if std::path::Path::new(&destination).is_dir() {
            fullpath_destination = std::fs::canonicalize(destination)?
                .into_os_string()
                .into_string()?;

            crate::processor::sync_msg_simulation(&fullpath_destination);

            fullpath_source_copy = String::from(&fullpath_source);
            fullpath_destination_copy = String::from(&fullpath_destination);

            // Remove files and folders in another thread
            handle = std::thread::spawn(move || -> Result<(), crate::processor::SyncError> {
                remove_simulation(&fullpath_source, &fullpath_destination)
            });

            update_result = update_simulation(&fullpath_source_copy, &fullpath_destination_copy);

            match handle.join() {
                Err(_) => {
                    return Err(crate::processor::SyncError {
                        code: crate::processor::error_thread_join(),
                        file: file!(),
                        line: line!(),
                        source: None,
                        destination: None,
                    });
                }
                Ok(value) => match value {
                    Err(_) => return value,
                    Ok(_) => return update_result,
                },
            }
        }

        return Err(crate::processor::SyncError {
            code: crate::processor::error_dest_not_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    // source is a file or symlink
    if !std::path::Path::new(&destination).exists() {
        crate::processor::copy_msg_simulation(destination);
        return Ok(());
    }

    if std::path::Path::new(&destination).is_dir() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_dest_not_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    // destination is a file or symlink
    update_file_simulation(source, destination)
}

/// Synchronizes source to destination without read or create a config file
pub fn sync(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
    let fullpath_destination: String;
    let fullpath_source_copy: String;
    let fullpath_destination_copy: String;

    let handle: std::thread::JoinHandle<Result<(), crate::processor::SyncError>>;
    let update_result: Result<(), crate::processor::SyncError>;

    /// Copy a file from source to destination, displays a message and checks for errors
    fn copy_file(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
        #[cfg(feature = "i18n")]
        crate::processor::copy_msg(destination);

        crate::processor::copy(source, destination)
    }

    /// Copy source folder to destination and all it's contents recursively
    fn copy_folder(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
        let mut fullpath: String;
        let mut fullpath_destination: String;

        for path in std::fs::read_dir(source)? {
            fullpath = path?.path().display().to_string();
            fullpath_destination = fullpath.replace(source, destination);

            // Copy file or symlink
            if !std::fs::metadata(&fullpath)?.is_dir() {
                copy_file(&fullpath, &fullpath_destination)?;
                continue;
            }

            // Create destination folder and copy directories recursively
            create_folder(&fullpath_destination)?;
            copy_folder(&fullpath, &fullpath_destination)?;
        }

        Ok(())
    }

    /// Displays a create message and creates a folder
    #[inline(always)]
    fn create_folder(folder: &str) -> Result<(), std::io::Error> {
        #[cfg(feature = "i18n")]
        crate::processor::create_msg(folder);
        std::fs::create_dir(folder)
    }

    /// Replaces the destination file if its different from source
    fn update_file(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
        if std::fs::metadata(source)?.modified()? == std::fs::metadata(destination)?.modified()? {
            return Ok(());
        }

        #[cfg(feature = "i18n")]
        crate::processor::update_msg(destination);

        crate::processor::copy(source, destination)
    }

    /// Displays a remove message and removes a file or folder from destination
    #[inline(always)]
    fn remove_all(
        file_folder: &str,
        remove_file_folder: fn(String) -> Result<(), std::io::Error>,
    ) -> Result<(), std::io::Error> {
        #[cfg(feature = "i18n")]
        crate::processor::remove_msg(file_folder);
        remove_file_folder(String::from(file_folder))
    }

    /// Iterates over source folder adding and updating files and folders in destination
    /// and removes files and folders from destination not found in source
    fn update(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
        let mut fullpath_source: String;
        let mut fullpath_destination: String;
        let mut file_folder: std::io::Result<std::fs::Metadata>;

        for path in std::fs::read_dir(source)? {
            fullpath_source = path?.path().display().to_string();
            fullpath_destination = fullpath_source.replace(source, destination);
            file_folder = std::fs::metadata(&fullpath_destination);

            if !std::fs::metadata(&fullpath_source)?.is_dir() {
                match file_folder {
                    Err(_) => copy_file(&fullpath_source, &fullpath_destination)?,
                    Ok(_) => update_file(&fullpath_source, &fullpath_destination)?, // File exists, update if necessary
                }
                continue;
            }

            // Folder does not exist
            if file_folder.is_err() {
                create_folder(&fullpath_destination)?;
                copy_folder(&fullpath_source, &fullpath_destination)?;
                continue;
            }

            update(&fullpath_source, &fullpath_destination)?;
        }
        Ok(())
    }

    /// Iterate over destination folder and remove files and folders that doesn't exists in source
    fn remove(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
        let mut fullpath_destination: String;
        let mut fullpath_source: String;
        let mut not_found: bool;

        for path in std::fs::read_dir(destination)? {
            fullpath_destination = path?.path().display().to_string();
            fullpath_source = fullpath_destination.replace(destination, source);
            not_found = !std::path::Path::new(&fullpath_source).exists();

            // File not found in source, remove in destination
            if !std::fs::metadata(&fullpath_destination)?.is_dir() {
                if not_found {
                    remove_all(&fullpath_destination, std::fs::remove_file)?;
                }
                continue;
            }

            // Directory not found in source, remove in destination
            if not_found {
                remove_all(&fullpath_destination, std::fs::remove_dir_all)?;
                continue;
            }

            remove(&fullpath_source, &fullpath_destination)?;
        }
        Ok(())
    }

    if !std::path::Path::new(&source).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if source == destination {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_same_file_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    let fullpath_source = std::fs::canonicalize(source)?
        .into_os_string()
        .into_string()?;

    if std::path::Path::new(&source).is_dir() {
        if !std::path::Path::new(&destination).exists() {
            create_folder(destination)?;
            fullpath_destination = std::fs::canonicalize(destination)?
                .into_os_string()
                .into_string()?;
            return copy_folder(&fullpath_source, &fullpath_destination);
        }

        if std::path::Path::new(&destination).is_dir() {
            // Remove files and folders first to free disk space, add and update files and folders
            fullpath_destination = std::fs::canonicalize(destination)?
                .into_os_string()
                .into_string()?;

            #[cfg(feature = "i18n")]
            crate::processor::sync_msg(&fullpath_destination);

            fullpath_source_copy = String::from(&fullpath_source);
            fullpath_destination_copy = String::from(&fullpath_destination);

            // Remove files and folders in another thread
            handle = std::thread::spawn(move || -> Result<(), crate::processor::SyncError> {
                remove(&fullpath_source, &fullpath_destination)
            });

            update_result = update(&fullpath_source_copy, &fullpath_destination_copy);

            match handle.join() {
                Err(_) => {
                    return Err(crate::processor::SyncError {
                        code: crate::processor::error_thread_join(),
                        file: file!(),
                        line: line!(),
                        source: None,
                        destination: None,
                    });
                }
                Ok(value) => match value {
                    Err(_) => return value,
                    Ok(_) => return update_result,
                },
            }
        }

        return Err(crate::processor::SyncError {
            code: crate::processor::error_dest_not_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    // source is a file or symlink
    if !std::path::Path::new(&destination).exists() {
        return copy_file(source, destination);
    }

    if std::path::Path::new(&destination).is_dir() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_dest_not_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    // destination is a file or symlink
    update_file(source, destination)
}

/// Synchronizes and checks every byte stopping only on success or Ctrl+C
pub fn force(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
    loop {
        if let Err(_err) = sync(source, destination) {
            #[cfg(feature = "i18n")]
            crate::processor::error_msg(&_err.to_string(), _err.code, false);
            continue;
        }

        if let Err(_err) = crate::processor::check(source, destination) {
            #[cfg(feature = "i18n")]
            crate::processor::error_msg(&_err.to_string(), _err.code, false);
            continue;
        }
        break;
    }
    Ok(())
}

//====================================== Unit Tests ======================================

#[cfg(test)]
mod tests {
    use crate::processor::tests::{Folder, TextFile};

    #[test]
    fn src_inexistent_dest_inexistent() {
        match crate::processor::sync("none", "nothing") {
            Err(err) => assert_eq!(err.code, crate::processor::error_source_folder()),
            Ok(_) => panic!("ERROR => src_inexistent_dest_inexistent"),
        }
    }

    #[test]
    fn src_folder_dest_folder_same() {
        let folder = Folder::new("src_folder_dest_folder_same");
        match crate::processor::sync(&folder.path, &folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::error_same_file_folder()),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_same"),
        }
    }

    #[test]
    fn src_folder_empty_dest_inexistent() -> Result<(), crate::processor::SyncError> {
        let _root = Folder::new("src_folder_empty_dest_inexistent");
        let folder_src = Folder::new("src_folder_empty_dest_inexistent/source");

        crate::processor::sync(
            &folder_src.path,
            "target/src_folder_empty_dest_inexistent/destination",
        )?;
        crate::processor::check(
            &folder_src.path,
            "target/src_folder_empty_dest_inexistent/destination",
        )
    }

    #[test]
    fn src_1_folder_1_file_dest_inexistent() -> Result<(), crate::processor::SyncError> {
        let _root = Folder::new("src_1_folder_1_file_dest_inexistent");
        let src_folder = Folder::new("src_1_folder_1_file_dest_inexistent/source");
        let _file = TextFile::new(
            "src_1_folder_1_file_dest_inexistent/source/file.txt",
            b"data",
        );

        crate::processor::sync(
            &src_folder.path,
            "target/src_1_folder_1_file_dest_inexistent/destination",
        )?;
        crate::processor::check(
            &src_folder.path,
            "target/src_1_folder_1_file_dest_inexistent/destination",
        )
    }

    #[test]
    fn src_2_folders_2_files_dest_inexistent() -> Result<(), crate::processor::SyncError> {
        let _root = Folder::new("src_2_folders_2_files_dest_inexistent");
        let src_folder = Folder::new("src_2_folders_2_files_dest_inexistent/source");
        let _src_folder2 = Folder::new("src_2_folders_2_files_dest_inexistent/source/2");

        let _file = TextFile::new(
            "src_2_folders_2_files_dest_inexistent/source/file.txt",
            b"data",
        );
        let _file2 = TextFile::new(
            "src_2_folders_2_files_dest_inexistent/source/2/file2.txt",
            b"data2",
        );

        crate::processor::sync(
            &src_folder.path,
            "target/src_2_folders_2_files_dest_inexistent/destination",
        )?;
        crate::processor::check(
            &src_folder.path,
            "target/src_2_folders_2_files_dest_inexistent/destination",
        )
    }

    #[test]
    fn src_2_folders_4_files_dest_inexistent() -> Result<(), crate::processor::SyncError> {
        let _root = Folder::new("src_2_folders_4_files_dest_inexistent");
        let src_folder = Folder::new("src_2_folders_4_files_dest_inexistent/source");
        let _src_folder2 = Folder::new("src_2_folders_4_files_dest_inexistent/source/2");

        let _src_file = TextFile::new(
            "src_2_folders_4_files_dest_inexistent/source/file.txt",
            b"data",
        );
        let _src_file2 = TextFile::new(
            "src_2_folders_4_files_dest_inexistent/source/file2.txt",
            b"data2",
        );
        let _src_file3 = TextFile::new(
            "src_2_folders_4_files_dest_inexistent/source/2/file3.txt",
            b"data3",
        );
        let _src_file4 = TextFile::new(
            "src_2_folders_4_files_dest_inexistent/source/2/file4.txt",
            b"data4",
        );

        crate::processor::sync(
            &src_folder.path,
            "target/src_2_folders_4_files_dest_inexistent/destination",
        )?;
        crate::processor::check(
            &src_folder.path,
            "target/src_2_folders_4_files_dest_inexistent/destination",
        )
    }

    #[test]
    fn src_folder_empty_dest_folder_1_file() -> Result<(), crate::processor::SyncError> {
        let src_folder = Folder::new("src_folder_empty_dest_folder_1_file_SOURCE");
        let dest_folder = Folder::new("src_folder_empty_dest_folder_1_file_DESTINATION");

        let _dest_file = TextFile::new(
            "src_folder_empty_dest_folder_1_file_DESTINATION/file.txt",
            b"data",
        );

        crate::processor::sync(&src_folder.path, &dest_folder.path)?;
        crate::processor::check(&src_folder.path, &dest_folder.path)
    }

    #[test]
    fn src_folder_empty_dest_folder_1_folder_empty() -> Result<(), crate::processor::SyncError> {
        let src_folder = Folder::new("src_folder_empty_dest_folder_1_folder_empty_SOURCE");
        let dest_folder = Folder::new("src_folder_empty_dest_folder_1_folder_empty_DESTINATION");
        let _dest_empty_folder =
            Folder::new("src_folder_empty_dest_folder_1_folder_empty_DESTINATION/empty_folder");

        crate::processor::sync(&src_folder.path, &dest_folder.path)?;
        crate::processor::check(&src_folder.path, &dest_folder.path)
    }

    #[test]
    fn src_folder_empty_dest_folder_1_folder_2_files() -> Result<(), crate::processor::SyncError> {
        let src_folder = Folder::new("src_folder_empty_dest_folder_1_folder_2_files_SOURCE");
        let dest_folder = Folder::new("src_folder_empty_dest_folder_1_folder_2_files_DESTINATION");
        let _dest_folder2 =
            Folder::new("src_folder_empty_dest_folder_1_folder_2_files_DESTINATION/2");

        let _dest_file = TextFile::new(
            "src_folder_empty_dest_folder_1_folder_2_files_DESTINATION/file.txt",
            b"data",
        );
        let _dest_file2 = TextFile::new(
            "src_folder_empty_dest_folder_1_folder_2_files_DESTINATION/2/file.txt",
            b"data",
        );

        crate::processor::sync(&src_folder.path, &dest_folder.path)?;
        crate::processor::check(&src_folder.path, &dest_folder.path)
    }

    #[test]
    fn src_folder_empty_dest_folder_2_folders_2_files() -> Result<(), crate::processor::SyncError> {
        let src_folder = Folder::new("src_folder_empty_dest_folder_2_folders_2_files_SOURCE");
        let dest_folder = Folder::new("src_folder_empty_dest_folder_2_folders_2_files_DESTINATION");
        let _dest_folder2_3 =
            Folder::new("src_folder_empty_dest_folder_2_folders_2_files_DESTINATION/2/3");

        let _dest_file1 = TextFile::new(
            "src_folder_empty_dest_folder_2_folders_2_files_DESTINATION/file1.txt",
            b"data",
        );
        let _dest_file2 = TextFile::new(
            "src_folder_empty_dest_folder_2_folders_2_files_DESTINATION/file2.txt",
            b"data",
        );
        let _dest_file3 = TextFile::new(
            "src_folder_empty_dest_folder_2_folders_2_files_DESTINATION/2/file1.txt",
            b"data",
        );
        let _dest_file4 = TextFile::new(
            "src_folder_empty_dest_folder_2_folders_2_files_DESTINATION/2/file2.txt",
            b"data",
        );
        let _dest_file5 = TextFile::new(
            "src_folder_empty_dest_folder_2_folders_2_files_DESTINATION/2/3/file1.txt",
            b"data",
        );
        let _dest_file6 = TextFile::new(
            "src_folder_empty_dest_folder_2_folders_2_files_DESTINATION/2/3/file2.txt",
            b"data",
        );

        crate::processor::sync(&src_folder.path, &dest_folder.path)?;
        crate::processor::check(&src_folder.path, &dest_folder.path)
    }

    #[test]
    fn src_folder_dest_file() {
        let src_folder = Folder::new("src_folder_dest_file");
        let dest_file = TextFile::new("src_folder_dest_file.txt", b"data");

        match crate::processor::sync(&src_folder.path, &dest_file.path) {
            Err(err) => assert_eq!(err.code, crate::processor::error_dest_not_folder()),
            Ok(_) => panic!("ERROR => src_folder_dest_file"),
        }
    }

    #[test]
    fn src_file_dest_folder() {
        let src_file = TextFile::new("src_file_dest_folder.txt", b"data");
        let dest_folder = Folder::new("src_file_dest_folder");

        match crate::processor::sync(&src_file.path, &dest_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::error_dest_not_folder()),
            Ok(_) => panic!("ERROR => src_file_dest_folder"),
        }
    }

    #[test]
    fn src_file_empty_dest_inexistent() -> Result<(), crate::processor::SyncError> {
        let _root = Folder::new("src_file_empty_dest_inexistent");
        let src_file = TextFile::new("src_file_empty_dest_inexistent/source.txt", b"");

        crate::processor::sync(
            &src_file.path,
            "target/src_file_empty_dest_inexistent/destination.txt",
        )?;
        crate::processor::check(
            &src_file.path,
            "target/src_file_empty_dest_inexistent/destination.txt",
        )
    }

    #[test]
    fn src_file_dest_inexistent() -> Result<(), crate::processor::SyncError> {
        let _root = Folder::new("src_file_dest_inexistent");
        let src_file = TextFile::new("src_file_dest_inexistent/source.txt", b"data");

        crate::processor::sync(
            &src_file.path,
            "target/src_file_dest_inexistent/destination.txt",
        )?;
        crate::processor::check(
            &src_file.path,
            "target/src_file_dest_inexistent/destination.txt",
        )
    }

    #[test]
    fn src_file_dest_different() -> Result<(), crate::processor::SyncError> {
        let _root = Folder::new("src_file_dest_different");
        let src_file = TextFile::new("src_file_dest_different/source.txt", b"data");
        let dest_file = TextFile::new("src_file_dest_different/destination.txt", b"data\n");

        crate::processor::sync(&src_file.path, &dest_file.path)?;
        crate::processor::check(&src_file.path, &dest_file.path)
    }
}
