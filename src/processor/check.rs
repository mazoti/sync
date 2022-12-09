//! Compares source and destination.
//! Both source and destination must be the same type
//! (files or folders).

use std::{io::Read, path::Path};

/// Compares every folder, file and byte
pub fn check(
    source: &str,
    destination: &str,
    buffer_size: usize,
) -> Result<(), crate::processor::SyncError> {
    /// Look for removed files or folders in source
    fn check_file_folder_add_removed(
        source: &str,
        destination: &str,
    ) -> Result<(), crate::processor::SyncError> {
        // Check for empty folder
        if std::fs::read_dir(source)?.next().is_none() {
            if std::fs::read_dir(destination)?.next().is_none() {
                return Ok(());
            }
            return Err(crate::processor::SyncError {
                code: crate::processor::error_diff_file_folder(),
                message: crate::processor::error_to_string(
                    crate::processor::error_diff_file_folder(),
                ),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        for path in std::fs::read_dir(source)? {
            let fullpath = path?.path().display().to_string();
            let fullpath_destination = fullpath.replace(source, destination);

            // Check file or symlink
            if !Path::new(&fullpath).is_dir() {
                if !Path::new(&fullpath_destination).exists() {
                    return Err(crate::processor::SyncError {
                        code: crate::processor::error_diff_file_folder(),
                        message: crate::processor::error_to_string(
                            crate::processor::error_diff_file_folder(),
                        ),
                        file: file!(),
                        line: line!(),
                        source: Some(source.to_string()),
                        destination: Some(destination.to_string()),
                    });
                }
                continue;
            }
            check_file_folder_add_removed(&fullpath, &fullpath_destination)?;
        }
        Ok(())
    }

    /// Checks if folder and file contents are the same
    fn check_file_folder(
        source: &str,
        destination: &str,
        buffer_size: usize,
    ) -> Result<(), crate::processor::SyncError> {
        // Check for empty folder
        if std::fs::read_dir(source)?.next().is_none() {
            if std::fs::read_dir(destination)?.next().is_none() {
                #[cfg(feature = "cli")]
                crate::processor::ok_msg(destination);
                return Ok(());
            }
            return Err(crate::processor::SyncError {
                code: crate::processor::error_diff_file_folder(),
                message: crate::processor::error_to_string(
                    crate::processor::error_diff_file_folder(),
                ),
                file: file!(),
                line: line!(),
                source: Some(source.to_string()),
                destination: Some(destination.to_string()),
            });
        }

        for path in std::fs::read_dir(source)? {
            let fullpath = path?.path().display().to_string();
            let fullpath_destination = fullpath.replace(source, destination);

            // Check file or symlink
            if !Path::new(&fullpath).is_dir() {
                check_file(&fullpath, &fullpath_destination, buffer_size)?;
                continue;
            }
            check_file_folder(&fullpath, &fullpath_destination, buffer_size)?;
        }
        Ok(())
    }

    /// Checks if file contents are the same
    fn check_file(
        source: &str,
        destination: &str,
        buffer_size: usize,
    ) -> Result<(), crate::processor::SyncError> {
        let mut src_file = std::fs::File::open(source)?;
        let mut dest_file = std::fs::File::open(destination)?;

        let mut src_buffer = vec![0; buffer_size];
        let mut dest_buffer = vec![0; buffer_size];

        loop {
            let src_bytes = src_file.read(&mut src_buffer)?;
            let dest_bytes = dest_file.read(&mut dest_buffer)?;

            if src_bytes != dest_bytes {
                return Err(crate::processor::SyncError {
                    code: crate::processor::error_diff_file_folder(),
                    message: crate::processor::error_to_string(
                        crate::processor::error_diff_file_folder(),
                    ),
                    file: file!(),
                    line: line!(),
                    source: Some(source.to_string()),
                    destination: Some(destination.to_string()),
                });
            }

            for i in 0..src_bytes {
                if src_buffer[i] != dest_buffer[i] {
                    return Err(crate::processor::SyncError {
                        code: crate::processor::error_diff_file_folder(),
                        message: crate::processor::error_to_string(
                            crate::processor::error_diff_file_folder(),
                        ),
                        file: file!(),
                        line: line!(),
                        source: Some(source.to_string()),
                        destination: Some(destination.to_string()),
                    });
                }
            }

            if src_bytes < buffer_size {
                break;
            }
        }

        #[cfg(feature = "cli")]
        crate::processor::ok_msg(destination);

        Ok(())
    }

    if !(Path::new(&source).exists() && Path::new(&destination).exists()) {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            message: crate::processor::error_to_string(crate::processor::error_source_folder()),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if source == destination {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_same_file_folder(),
            message: crate::processor::error_to_string(crate::processor::error_same_file_folder()),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    // source and destination exists
    if Path::new(&source).is_dir() {
        if Path::new(&destination).is_dir() {
            check_file_folder_add_removed(destination, source)?;
            check_file_folder_add_removed(source, destination)?;
            return check_file_folder(source, destination, buffer_size);
        }

        // source is a directory but destination not
        return Err(crate::processor::SyncError {
            code: crate::processor::error_dest_not_folder(),
            message: crate::processor::error_to_string(crate::processor::error_dest_not_folder()),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    // source is a file or symlink
    if Path::new(&destination).is_dir() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_dest_not_file(),
            message: crate::processor::error_to_string(crate::processor::error_dest_not_file()),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    check_file(source, destination, buffer_size)
}

//====================================== Unit Tests ======================================

#[cfg(test)]
mod tests {

    use crate::processor::tests::{Folder, TextFile};

    #[test]
    fn src_inexistent_dest_inexistent() {
        match crate::processor::check::check("none", "nothing") {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_SOURCE_FOLDER),
            Ok(_) => panic!("ERROR => src_inexistent_dest_inexistent"),
        }
    }

    #[test]
    fn src_folder_dest_folder_same() {
        let src_folder = Folder::new("src_folder_dest_folder_same");

        match crate::processor::check::check(&src_folder.path, &src_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_SAME_FILE_FOLDER),
            Ok(_) => panic!("ERROR => src_folder_dest_folder_same"),
        }
    }

    #[test]
    fn src_folder_empty_dest_folder_inexistent() {
        let src_folder = Folder::new("src_folder_empty_dest_folder_inexistent_SOURCE");
        match crate::processor::check::check(
            &src_folder.path,
            "src_folder_empty_dest_folder_inexistent_DESTINATION",
        ) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_SOURCE_FOLDER),
            Ok(_) => panic!("ERROR => src_folder_empty_dest_folder_inexistent"),
        }
    }

    #[test]
    fn src_folder_dest_file() {
        let src_folder = Folder::new("src_folder_dest_file");
        let dest_file = TextFile::new("src_folder_dest_file/file.txt", b"data\n");

        match crate::processor::check::check(&src_folder.path, &dest_file.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_DEST_NOT_FOLDER),
            Ok(_) => panic!("ERROR => src_folder_dest_file"),
        }
    }

    #[test]
    fn src_file_dest_folder() {
        let dest_folder = Folder::new("src_file_dest_folder");
        let src_file = TextFile::new("src_file_dest_folder/file.txt", b"data\n");

        match crate::processor::check::check(&src_file.path, &dest_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_DEST_NOT_FILE),
            Ok(_) => panic!("ERROR => src_file_dest_folder"),
        }
    }

    #[test]
    fn src_inexistent_dest_folder() {
        let dest_folder = Folder::new("src_inexistent_dest_folder");
        match crate::processor::check::check("none", &dest_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_SOURCE_FOLDER),
            Ok(_) => panic!("ERROR => src_inexistent_dest_folder"),
        }
    }

    #[test]
    fn src_file_empty_dest_file_empty() -> Result<(), crate::processor::error::SyncError> {
        let src_file = TextFile::new("src_file_empty_dest_file_empty_SOURCE.txt", b"");
        let dest_file = TextFile::new("src_file_empty_dest_file_empty_DESTINATION.txt", b"");

        crate::processor::check::check(&src_file.path, &dest_file.path)
    }

    #[test]
    fn src_file_dest_file_different() {
        let src_file = TextFile::new("src_file_dest_file_different_SOURCE.txt", b"data");
        let dest_file = TextFile::new("src_file_dest_file_different_DESTINATION.txt", b"data\n");

        match crate::processor::check::check(&src_file.path, &dest_file.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_DIFF_FILE_FOLDER),
            Ok(_) => panic!("ERROR => src_file_dest_file_different"),
        }
    }

    #[test]
    fn src_file_dest_file_equals() -> Result<(), crate::processor::error::SyncError> {
        let src_file = TextFile::new("src_file_dest_file_equals_SOURCE.txt", b"data\n");
        let dest_file = TextFile::new("src_file_dest_file_equals_DESTINATION.txt", b"data\n");

        crate::processor::check::check(&src_file.path, &dest_file.path)
    }

    #[test]
    fn src_folder_dest_inexistent() {
        let src_folder = Folder::new("src_folder_dest_inexistent");

        match crate::processor::check::check(&src_folder.path, "none") {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_SOURCE_FOLDER),
            Ok(_) => panic!("ERROR => src_folder_dest_inexistent"),
        }
    }

    #[test]
    fn src_folder_empty_dest_folder_empty() -> Result<(), crate::processor::error::SyncError> {
        let src_folder = Folder::new("src_folder_empty_dest_folder_empty_SOURCE");
        let dest_folder = Folder::new("src_folder_empty_dest_folder_empty_DESTINATION");

        crate::processor::check::check(&src_folder.path, &dest_folder.path)
    }

    #[test]
    fn src_folder_1_file_dest_folder_2_files() {
        let _root = Folder::new("src_folder_1_file_dest_folder_2_files");
        let src_folder = Folder::new("src_folder_1_file_dest_folder_2_files/source");
        let dest_folder = Folder::new("src_folder_1_file_dest_folder_2_files/destination");

        let _src_file = TextFile::new(
            "src_folder_1_file_dest_folder_2_files/source/file.txt",
            b"data",
        );
        let _dest_file = TextFile::new(
            "src_folder_1_file_dest_folder_2_files/destination/file.txt",
            b"data",
        );
        let _dest_file2 = TextFile::new(
            "src_folder_1_file_dest_folder_2_files/destination/file2.txt",
            b"data",
        );

        match crate::processor::check::check(&src_folder.path, &dest_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_DIFF_FILE_FOLDER),
            Ok(_) => panic!("ERROR => src_folder_1_file_dest_folder_2_files"),
        }
    }

    #[test]
    fn src_inexistent_dest_file() {
        let dest_file = TextFile::new("src_inexistent_dest_file.txt", b"data");

        match crate::processor::check::check("none", &dest_file.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_SOURCE_FOLDER),
            Ok(_) => panic!("ERROR => src_inexistent_dest_file"),
        }
    }

    #[test]
    fn src_file_dest_inexistent() {
        let src_file = TextFile::new("src_file_dest_inexistent.txt", b"data");
        match crate::processor::check::check(&src_file.path, "none") {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_SOURCE_FOLDER),
            Ok(_) => panic!("ERROR => src_file_dest_inexistent"),
        }
    }

    #[test]
    fn src_file_dest_file_same() {
        let src_file = TextFile::new("src_file_dest_file_same.txt", b"data");
        match crate::processor::check::check(&src_file.path, &src_file.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_SAME_FILE_FOLDER),
            Ok(_) => panic!("ERROR => src_file_dest_file_same"),
        }
    }

    #[test]
    fn src_folder_1_file_dest_folder_empty() {
        let _root = Folder::new("src_folder_1_file_dest_folder_empty");
        let src_folder = Folder::new("src_folder_1_file_dest_folder_empty/source");
        let dest_folder = Folder::new("src_folder_1_file_dest_folder_empty/destination");

        let _src_file = TextFile::new(
            "src_folder_1_file_dest_folder_empty/source/file.txt",
            b"data",
        );

        match crate::processor::check::check(&src_folder.path, &dest_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_DIFF_FILE_FOLDER),
            Ok(_) => panic!("ERROR => src_folder_1_file_dest_folder_empty"),
        }
    }

    #[test]
    fn src_empty_dest_folder_1_file() {
        let src_folder = Folder::new("src_empty_dest_folder_1_file_SOURCE");
        let dest_folder = Folder::new("src_empty_dest_folder_1_file_DESTINATION");

        let _dest_file =
            TextFile::new("src_empty_dest_folder_1_file_DESTINATION/file.txt", b"data");

        match crate::processor::check::check(&src_folder.path, &dest_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_DIFF_FILE_FOLDER),
            Ok(_) => panic!("ERROR => src_empty_dest_folder_1_file"),
        }
    }

    #[test]
    fn src_folder_1_file_dest_1_file_different() {
        let src_folder = Folder::new("src_folder_1_file_dest_1_file_different_SOURCE");
        let dest_folder = Folder::new("src_folder_1_file_dest_1_file_different_DESTINATION");

        let _src_file = TextFile::new(
            "src_folder_1_file_dest_1_file_different_SOURCE/file.txt",
            b"data",
        );
        let _dest_file = TextFile::new(
            "src_folder_1_file_dest_1_file_different_DESTINATION/file.txt",
            b"data\n",
        );

        match crate::processor::check::check(&src_folder.path, &dest_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_DIFF_FILE_FOLDER),
            Ok(_) => panic!("ERROR => src_folder_1_file_dest_1_file_different"),
        }
    }

    #[test]
    fn src_folder_1_file_dest_1_file_equals() -> Result<(), crate::processor::error::SyncError> {
        let src_folder = Folder::new("src_folder_1_file_dest_1_file_equals_SOURCE");
        let dest_folder = Folder::new("src_folder_1_file_dest_1_file_equals_DESTINATION");

        let _src_file = TextFile::new(
            "src_folder_1_file_dest_1_file_equals_SOURCE/file.txt",
            b"data",
        );
        let _dest_file = TextFile::new(
            "src_folder_1_file_dest_1_file_equals_DESTINATION/file.txt",
            b"data",
        );

        crate::processor::check::check(&src_folder.path, &dest_folder.path)
    }

    #[test]
    fn src_2_folders_2_files_dest_2_folders_2_files_1_file_different() {
        let src_folder =
            Folder::new("src_2_folders_2_files_dest_2_folders_2_files_1_file_different_SOURCE");
        let dest_folder = Folder::new(
            "src_2_folders_2_files_dest_2_folders_2_files_1_file_different_DESTINATION",
        );

        let _src_folder2 =
            Folder::new("src_2_folders_2_files_dest_2_folders_2_files_1_file_different_SOURCE/2");
        let _dest_folder2 = Folder::new(
            "src_2_folders_2_files_dest_2_folders_2_files_1_file_different_DESTINATION/2",
        );

        let _src_file1 = TextFile::new(
            "src_2_folders_2_files_dest_2_folders_2_files_1_file_different_SOURCE/2/file1.txt",
            b"data",
        );
        let _src_file2 = TextFile::new(
            "src_2_folders_2_files_dest_2_folders_2_files_1_file_different_SOURCE/2/file2.txt",
            b"data",
        );

        let _dest_file1 = TextFile::new(
            "src_2_folders_2_files_dest_2_folders_2_files_1_file_different_DESTINATION/2/file1.txt",
            b"data",
        );
        let _dest_file2 = TextFile::new(
            "src_2_folders_2_files_dest_2_folders_2_files_1_file_different_DESTINATION/2/file2.txt",
            b"data\n",
        );

        match crate::processor::check::check(&src_folder.path, &dest_folder.path) {
            Err(err) => assert_eq!(err.code, crate::processor::consts::ERROR_DIFF_FILE_FOLDER),
            Ok(_) => {
                panic!("ERROR => src_2_folders_2_files_dest_2_folders_2_files_1_file_different")
            }
        }
    }

    #[test]
    fn src_2_folders_2_files_dest_2_folders_2_files_equals(
    ) -> Result<(), crate::processor::error::SyncError> {
        let src_folder = Folder::new("src_2_folders_2_files_dest_2_folders_2_files_equals_SOURCE");
        let dest_folder =
            Folder::new("src_2_folders_2_files_dest_2_folders_2_files_equals_DESTINATION");

        let _src_folder2 =
            Folder::new("src_2_folders_2_files_dest_2_folders_2_files_equals_SOURCE/2");
        let _dest_folder2 =
            Folder::new("src_2_folders_2_files_dest_2_folders_2_files_equals_DESTINATION/2");

        let _src_file1 = TextFile::new(
            "src_2_folders_2_files_dest_2_folders_2_files_equals_SOURCE/2/file1.txt",
            b"data",
        );
        let _src_file2 = TextFile::new(
            "src_2_folders_2_files_dest_2_folders_2_files_equals_SOURCE/2/file2.txt",
            b"data",
        );

        let _dest_file1 = TextFile::new(
            "src_2_folders_2_files_dest_2_folders_2_files_equals_DESTINATION/2/file1.txt",
            b"data",
        );
        let _dest_file2 = TextFile::new(
            "src_2_folders_2_files_dest_2_folders_2_files_equals_DESTINATION/2/file2.txt",
            b"data",
        );

        crate::processor::check::check(&src_folder.path, &dest_folder.path)
    }
}
