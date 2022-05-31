//! The core of sync function implementation

use std::{io::BufRead, io::BufReader};

/// Copy source folder to destination and all it's contents recursively
fn copy_folder(source: &str, destination: &str) -> Result<(), crate::processor::error::SyncError> {
    for path in std::fs::read_dir(source)? {
        let fullpath = path?.path().display().to_string();
        let fullpath_destination = fullpath.replace(source, destination);

        // Copy file or symlink
        let metadata_source = std::fs::metadata(&fullpath)?;
        if !metadata_source.is_dir() {
            copy_file(&fullpath, &fullpath_destination)?;
            continue;
        }

        // Create destination folder and copy directories recursively
        create_folder(&fullpath_destination)?;
        copy_folder(&fullpath, &fullpath_destination)?;
    }

    Ok(())
}

/// Copy a file from source to destination, displays a message and checks for errors
#[inline]
fn copy_file(source: &str, destination: &str) -> Result<(), crate::processor::error::SyncError> {
    #[cfg(feature = "cli")]
    crate::processor::cli::copy_msg(destination);

    if std::fs::copy(&source, &destination)? == std::fs::metadata(&source)?.len() {
        // Make the modified date the same in source and destination (Unix and Linux only)
        #[cfg(not(windows))]
        {
            let file_source = std::fs::OpenOptions::new().write(true).open(source)?;
            let file_destination = std::fs::OpenOptions::new().write(true).open(destination)?;
            file_source.set_len(file_source.metadata()?.len())?;
            file_destination.set_len(file_destination.metadata()?.len())?;
        }
        return Ok(());
    }

    Err(crate::processor::error::SyncError {
        code: crate::processor::consts::ERROR_COPY_FILE_FOLDER,
        message: crate::processor::error::error_to_string(
            crate::processor::consts::ERROR_COPY_FILE_FOLDER,
        ),
        file: file!(),
        line: line!(),
        source: Some(source.to_string()),
        destination: Some(destination.to_string()),
    })
}

/// Displays a create message and creates a folder
#[inline]
fn create_folder(folder: &str) -> Result<(), std::io::Error> {
    #[cfg(feature = "cli")]
    crate::processor::cli::create_msg(folder);
    std::fs::create_dir(&folder)
}

/// Synchronizes source to destination without read or create a config file
pub fn sync(source: &str, destination: &str) -> Result<(), crate::processor::error::SyncError> {
    /// Displays a remove message and removes a file or folder from destination
    #[inline]
    fn remove_all(
        file_folder: &str,
        remove_file_folder: fn(String) -> Result<(), std::io::Error>,
    ) -> Result<(), std::io::Error> {
        #[cfg(feature = "cli")]
        crate::processor::cli::remove_msg(file_folder);
        remove_file_folder(String::from(file_folder))
    }

    /// Iterates over source folder adding and updating files and folders in destination
    /// and removes files and folders from destination not found in source
    fn update(source: &str, destination: &str) -> Result<(), crate::processor::error::SyncError> {
        for path in std::fs::read_dir(source)? {
            let fullpath_source = path?.path().display().to_string();
            let fullpath_destination = fullpath_source.replace(source, destination);
            let metadata_source = std::fs::metadata(&fullpath_source)?;
            let file_folder = std::fs::metadata(&fullpath_destination);

            if !metadata_source.is_dir() {
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
    fn remove(source: &str, destination: &str) -> Result<(), crate::processor::error::SyncError> {
        for path in std::fs::read_dir(destination)? {
            let fullpath_destination = path?.path().display().to_string();
            let fullpath_source = fullpath_destination.replace(destination, source);
            let not_found = !std::path::Path::new(&fullpath_source).exists();

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
        return Err(crate::processor::error::SyncError {
            code: crate::processor::consts::ERROR_SOURCE_FOLDER,
            message: crate::processor::error::error_to_string(
                crate::processor::consts::ERROR_SOURCE_FOLDER,
            ),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if source == destination {
        return Err(crate::processor::error::SyncError {
            code: crate::processor::consts::ERROR_SAME_FILE_FOLDER,
            message: crate::processor::error::error_to_string(
                crate::processor::consts::ERROR_SAME_FILE_FOLDER,
            ),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    let fullpath_source = std::fs::canonicalize(source)?
        .into_os_string()
        .into_string()
        .unwrap();

    if std::path::Path::new(&source).is_dir() {
        if !std::path::Path::new(&destination).exists() {
            create_folder(destination)?;
            let fullpath_destination = std::fs::canonicalize(destination)?
                .into_os_string()
                .into_string()
                .unwrap();
            return copy_folder(&fullpath_source, &fullpath_destination);
        }

        if std::path::Path::new(&destination).is_dir() {
            // Remove files and folders first to free disk space, add and update files and folders
            let fullpath_destination = std::fs::canonicalize(destination)?
                .into_os_string()
                .into_string()
                .unwrap();

            #[cfg(feature = "cli")]
            crate::processor::cli::sync_msg(&fullpath_destination);

            remove(&fullpath_source, &fullpath_destination)?;
            return update(&fullpath_source, &fullpath_destination);
        }

        return Err(crate::processor::error::SyncError {
            code: crate::processor::consts::ERROR_DEST_NOT_FOLDER,
            message: crate::processor::error::error_to_string(
                crate::processor::consts::ERROR_DEST_NOT_FOLDER,
            ),
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
        return Err(crate::processor::error::SyncError {
            code: crate::processor::consts::ERROR_DEST_NOT_FOLDER,
            message: crate::processor::error::error_to_string(
                crate::processor::consts::ERROR_DEST_NOT_FOLDER,
            ),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    // destination is a file or symlink
    update_file(source, destination)
}

/// Synchronizes all sources to destinations found in the config file
pub fn sync_file(config: &str) -> Result<(), crate::processor::error::SyncError> {
    #[cfg(feature = "cli")]
    crate::processor::cli::loading_msg(config);

    // Parse source and destination paths from config file
    for line in BufReader::new(std::fs::File::open(&config)?).lines() {
        let data = line?;
        let path: Vec<&str> = data.split('|').collect();
        if path.len() != 2 {
            return Err(crate::processor::error::SyncError {
                code: crate::processor::consts::ERROR_PARSE_LINE,
                message: crate::processor::error::error_to_string(
                    crate::processor::consts::ERROR_PARSE_LINE,
                ),
                file: file!(),
                line: line!(),
                source: None,
                destination: None,
            });
        }

        sync(path[0], path[1])?;
    }

    Ok(())
}

/// Synchronizes all config files found in folder
pub fn sync_folder(folder: &str) -> Result<(), crate::processor::error::SyncError> {
    let mut thread_pool = Vec::new();
    let mut exit_code = 0i32;
    let mut display_help = true;

    for path in std::fs::read_dir(folder)? {
        let fullpath = path?.path().display().to_string();
        if !std::fs::metadata(&fullpath)?.is_dir() && fullpath.ends_with(".config") {
            display_help = false;

            let handle = std::thread::spawn(move || -> i32 {
                if let Err(err) = sync_file(&fullpath) {
                    return err.code;
                }
                crate::processor::consts::NO_ERROR
            });

            thread_pool.push(handle);
        }
    }

    for handle in thread_pool {
        match handle.join() {
            Err(_) => {
                return Err(crate::processor::error::SyncError {
                    code: crate::processor::consts::ERROR_THREAD_JOIN,
                    message: crate::processor::error::error_to_string(
                        crate::processor::consts::ERROR_THREAD_JOIN,
                    ),
                    file: file!(),
                    line: line!(),
                    source: None,
                    destination: None,
                });
            }
            Ok(value) => {
                if value != 0 {
                    exit_code = value;
                    #[cfg(feature = "cli")]
                    crate::processor::cli::error_msg(
                        crate::processor::consts::ERROR_MSGS[value as usize],
                        0,
                    );
                }
            }
        }
    }

    if exit_code == 0 {
        if display_help {
            return Err(crate::processor::error::SyncError {
                code: crate::processor::consts::HELP,
                message: crate::processor::error::error_to_string(exit_code),
                file: file!(),
                line: line!(),
                source: None,
                destination: None,
            });
        }
        return Ok(());
    }

    Err(crate::processor::error::SyncError {
        code: exit_code,
        message: crate::processor::error::error_to_string(exit_code),
        file: file!(),
        line: line!(),
        source: None,
        destination: None,
    })
}

/// Replaces the destination file if its different from source
#[inline]
fn update_file(source: &str, destination: &str) -> Result<(), crate::processor::error::SyncError> {
    let metadata_source = std::fs::metadata(&source)?;

    if metadata_source.modified()? == std::fs::metadata(&destination)?.modified()? {
        return Ok(());
    }

    #[cfg(feature = "cli")]
    crate::processor::cli::update_msg(destination);

    if std::fs::copy(&source, &destination)? == metadata_source.len() {
        return Ok(());
    }

    Err(crate::processor::error::SyncError {
        code: crate::processor::consts::ERROR_COPY_FILE_FOLDER,
        message: crate::processor::error::error_to_string(
            crate::processor::consts::ERROR_COPY_FILE_FOLDER,
        ),
        file: file!(),
        line: line!(),
        source: Some(source.to_string()),
        destination: Some(destination.to_string()),
    })
}

//====================================== Unit Tests ======================================

#[cfg(test)]
mod tests {
    use crate::processor::tests::{Folder, TextFile};

    #[test]
    fn src_inexistent_dest_inexistent() {
        match crate::processor::sync::sync("none", "nothing") {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_SOURCE_FOLDER),
            Ok(_) => panic!("ERROR => src_inexistent_dest_inexistent"),
        }
    }

    #[test]
    fn src_folder_dest_folder_same() {
        let folder = Folder::new("src_folder_dest_folder_same");
        match crate::processor::sync::sync(&folder.path, &folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_SAME_FILE_FOLDER),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_same"),
        }
    }

    #[test]
    fn src_folder_empty_dest_inexistent() -> Result<(), crate::processor::error::SyncError> {
        let _root = Folder::new("src_folder_empty_dest_inexistent");
        let folder_src = Folder::new("src_folder_empty_dest_inexistent/source");

        crate::processor::sync::sync(
            &folder_src.path,
            "target/src_folder_empty_dest_inexistent/destination",
        )?;
        crate::processor::check::check(
            &folder_src.path,
            "target/src_folder_empty_dest_inexistent/destination",
        )
    }

    #[test]
    fn src_1_folder_1_file_dest_inexistent() -> Result<(), crate::processor::error::SyncError> {
        let _root = Folder::new("src_1_folder_1_file_dest_inexistent");
        let src_folder = Folder::new("src_1_folder_1_file_dest_inexistent/source");
        let _file = TextFile::new(
            "src_1_folder_1_file_dest_inexistent/source/file.txt",
            b"data",
        );

        crate::processor::sync::sync(
            &src_folder.path,
            "target/src_1_folder_1_file_dest_inexistent/destination",
        )?;
        crate::processor::check::check(
            &src_folder.path,
            "target/src_1_folder_1_file_dest_inexistent/destination",
        )
    }

    #[test]
    fn src_2_folders_2_files_dest_inexistent() -> Result<(), crate::processor::error::SyncError> {
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

        crate::processor::sync::sync(
            &src_folder.path,
            "target/src_2_folders_2_files_dest_inexistent/destination",
        )?;
        crate::processor::check::check(
            &src_folder.path,
            "target/src_2_folders_2_files_dest_inexistent/destination",
        )
    }

    #[test]
    fn src_2_folders_4_files_dest_inexistent() -> Result<(), crate::processor::error::SyncError> {
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

        crate::processor::sync::sync(
            &src_folder.path,
            "target/src_2_folders_4_files_dest_inexistent/destination",
        )?;
        crate::processor::check::check(
            &src_folder.path,
            "target/src_2_folders_4_files_dest_inexistent/destination",
        )
    }

    #[test]
    fn src_folder_empty_dest_folder_1_file() -> Result<(), crate::processor::error::SyncError> {
        let src_folder = Folder::new("src_folder_empty_dest_folder_1_file_SOURCE");
        let dest_folder = Folder::new("src_folder_empty_dest_folder_1_file_DESTINATION");

        let _dest_file = TextFile::new(
            "src_folder_empty_dest_folder_1_file_DESTINATION/file.txt",
            b"data",
        );

        crate::processor::sync::sync(&src_folder.path, &dest_folder.path)?;
        crate::processor::check::check(&src_folder.path, &dest_folder.path)
    }

    #[test]
    fn src_folder_empty_dest_folder_1_folder_empty(
    ) -> Result<(), crate::processor::error::SyncError> {
        let src_folder = Folder::new("src_folder_empty_dest_folder_1_folder_empty_SOURCE");
        let dest_folder = Folder::new("src_folder_empty_dest_folder_1_folder_empty_DESTINATION");
        let _dest_empty_folder =
            Folder::new("src_folder_empty_dest_folder_1_folder_empty_DESTINATION/empty_folder");

        crate::processor::sync::sync(&src_folder.path, &dest_folder.path)?;
        crate::processor::check::check(&src_folder.path, &dest_folder.path)
    }

    #[test]
    fn src_folder_empty_dest_folder_1_folder_2_files(
    ) -> Result<(), crate::processor::error::SyncError> {
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

        crate::processor::sync::sync(&src_folder.path, &dest_folder.path)?;
        crate::processor::check::check(&src_folder.path, &dest_folder.path)
    }

    #[test]
    fn src_folder_empty_dest_folder_2_folders_2_files(
    ) -> Result<(), crate::processor::error::SyncError> {
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

        crate::processor::sync::sync(&src_folder.path, &dest_folder.path)?;
        crate::processor::check::check(&src_folder.path, &dest_folder.path)
    }

    #[test]
    fn src_folder_dest_file() {
        let src_folder = Folder::new("src_folder_dest_file");
        let dest_file = TextFile::new("src_folder_dest_file.txt", b"data");

        match crate::processor::sync::sync(&src_folder.path, &dest_file.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_DEST_NOT_FOLDER),
            Ok(_) => panic!("ERROR => src_folder_dest_file"),
        }
    }

    #[test]
    fn src_file_dest_folder() {
        let src_file = TextFile::new("src_file_dest_folder.txt", b"data");
        let dest_folder = Folder::new("src_file_dest_folder");

        match crate::processor::sync::sync(&src_file.path, &dest_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_DEST_NOT_FOLDER),
            Ok(_) => panic!("ERROR => src_file_dest_folder"),
        }
    }

    #[test]
    fn src_file_empty_dest_inexistent() -> Result<(), crate::processor::error::SyncError> {
        let _root = Folder::new("src_file_empty_dest_inexistent");
        let src_file = TextFile::new("src_file_empty_dest_inexistent/source.txt", b"");

        crate::processor::sync::sync(
            &src_file.path,
            "target/src_file_empty_dest_inexistent/destination.txt",
        )?;
        crate::processor::check::check(
            &src_file.path,
            "target/src_file_empty_dest_inexistent/destination.txt",
        )
    }

    #[test]
    fn src_file_dest_inexistent() -> Result<(), crate::processor::error::SyncError> {
        let _root = Folder::new("src_file_dest_inexistent");
        let src_file = TextFile::new("src_file_dest_inexistent/source.txt", b"data");

        crate::processor::sync::sync(
            &src_file.path,
            "target/src_file_dest_inexistent/destination.txt",
        )?;
        crate::processor::check::check(
            &src_file.path,
            "target/src_file_dest_inexistent/destination.txt",
        )
    }

    #[test]
    fn src_file_dest_different() -> Result<(), crate::processor::error::SyncError> {
        let _root = Folder::new("src_file_dest_different");
        let src_file = TextFile::new("src_file_dest_different/source.txt", b"data");
        let dest_file = TextFile::new("src_file_dest_different/destination.txt", b"data\n");

        crate::processor::sync::sync(&src_file.path, &dest_file.path)?;
        crate::processor::check::check(&src_file.path, &dest_file.path)
    }
}
