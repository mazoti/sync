//! Finds all duplicated or empty files in the folder

#[cfg(feature = "tree")]
use std::collections::BTreeMap as Map;

#[cfg(not(feature = "tree"))]
use std::collections::HashMap as Map;

use std::io::Read;

/// Finds and displays all duplicated files in folder path
pub fn duplicate(folderpath: &str) -> Result<(), crate::processor::SyncError> {
    let mut file_count: usize;
    let mut hash: u32;

    let mut size_filepath: Map<u64, String> = Default::default();
    let mut adler32_filepath: Map<u32, String> = Default::default();

    /// Add files to map by size first
    fn add_files(
        folder: &str,
        size_filepath: &mut Map<u64, String>,
    ) -> Result<(), crate::processor::SyncError> {
        let mut fullpath: String;
        let mut file_size: u64;

        // Check for empty folder
        if std::fs::read_dir(folder)?.next().is_none() {
            return Ok(());
        }

        for path in std::fs::read_dir(folder)? {
            fullpath = path?.path().display().to_string();

            // Add file to map
            if !std::path::Path::new(&fullpath).is_dir() {
                file_size = std::fs::metadata(&fullpath)?.len();

                match size_filepath.get(&file_size) {
                    Some(files) => {
                        size_filepath.insert(file_size, files.to_owned() + "|" + &fullpath)
                    }
                    None => size_filepath.insert(file_size, fullpath),
                };
                continue;
            }
            add_files(&fullpath, size_filepath)?;
        }
        Ok(())
    }

    /// A fast hash function: two files will only be compared if they have the same size and the same adler32 hash
    fn adler32(file: &str, buffer_size: u64) -> Result<u32, crate::processor::SyncError> {
        let mut bytes_read: usize;
        let mut i: usize;

        let buffer_usize: usize = buffer_size.try_into()?;
        let mut buffer = vec![0; buffer_usize];

        let mut a: u32 = 1;
        let mut b: u32 = 0;

        if !(std::path::Path::new(file).exists() && std::path::Path::new(file).is_file()) {
            return Err(crate::processor::SyncError {
                code: crate::processor::error_source_file(),
                file: file!(),
                line: line!(),
                source: None,
                destination: Some(file.to_string()),
            });
        }

        let mut hash_file = std::fs::File::open(file)?;

        if std::fs::metadata(file)?.len() <= buffer_size {
            bytes_read = hash_file.read(&mut buffer)?;
            i = 0;
            while i < bytes_read {
                a = (a + (buffer[i] as u32)) % 65521; // buffer[i] is u8, no problem here
                b = (b + a) % 65521;

                i += 1;
            }

            return Ok((b << 16) | a);
        }

        loop {
            bytes_read = hash_file.read(&mut buffer)?;
            i = 0;
            while i < bytes_read {
                a = (a + (buffer[i] as u32)) % 65521;
                b = (b + a) % 65521;
                i += 1;
            }
            if bytes_read < buffer_usize {
                break;
            }
        }

        Ok((b << 16) | a)
    }

    /// Compare all files with each other (in pairs)
    fn compare_all_files_contents(files: &str) {
        let mut i: usize;
        let mut files_vector: Vec<&str> = files.split('|').collect();

        while !files_vector.is_empty() {
            let mut duplicated_files: Vec<&str> = Default::default();

            if let Some(file) = files_vector.pop() {
                duplicated_files.push(file);

                i = 0;
                while i < files_vector.len() {
                    if let Ok(()) = crate::processor::compare(files_vector[i], duplicated_files[0])
                    {
                        duplicated_files.push(files_vector[i]);
                        files_vector.remove(i);
                    }

                    i += 1;
                }

                if duplicated_files.len() > 1 {
                    crate::processor::duplicate_msgs(duplicated_files);
                }
            }
        }
    }

    if !(std::path::Path::new(folderpath).exists() && std::path::Path::new(folderpath).is_dir()) {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            file: file!(),
            line: line!(),
            source: Some(folderpath.to_string()),
            destination: None,
        });
    }

    add_files(folderpath, &mut size_filepath)?;

    // Print empty files
    if let Some(files) = size_filepath.remove(&0) {
        let files_vector: Vec<&str> = files.split('|').collect();
        for file in files_vector.iter() {
            crate::processor::empty_msg(file);
        }
    }

    println!();

    for value in size_filepath.values() {
        file_count = value.matches('|').count();

        // 2 files with the same size only, compare without hashing
        if file_count == 1 {
            let files: Vec<&str> = value.split('|').collect();
            if let Ok(()) = crate::processor::compare(files[0], files[1]) {
                crate::processor::duplicate_msgs(files);
            }

            continue;
        }

        if file_count > 1 {
            let files: Vec<&str> = value.split('|').collect();
            for file in files {
                hash = adler32(file, crate::processor::get_hash_buffer_size())?;

                match adler32_filepath.get(&hash) {
                    Some(paths) => adler32_filepath.insert(hash, paths.to_owned() + "?" + file),
                    None => adler32_filepath.insert(hash, file.to_string()),
                };
            }
        }
    }

    size_filepath.clear();

    // Same size and same hash, very high probability to be the same file (compare every byte)
    for value in adler32_filepath.values() {
        compare_all_files_contents(value);
    }

    Ok(())
}
