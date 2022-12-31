#[cfg(feature = "tree")]
use std::collections::BTreeMap as Map;

#[cfg(not(feature = "tree"))]
use std::collections::HashMap as Map;

fn compare_all_files_contents(files: &str) {
    let mut files_vector: Vec<&str> = files.split('?').collect();
    while files_vector.len() > 0 {
        let mut duplicated_files: Vec<&str> = Default::default();

        if let Some(file) = files_vector.pop() {
            duplicated_files.push(file);

            let mut i: usize = 0;
            while i < files_vector.len() {
                if let Ok(()) = crate::processor::compare(files_vector[i], duplicated_files[0]) {
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

fn add_files(
    folder: &str,
    size_filepath: &mut Map<u64, String>,
) -> Result<(), crate::processor::SyncError> {
    // Check for empty folder
    if std::fs::read_dir(folder)?.next().is_none() {
        return Ok(());
    }

    for path in std::fs::read_dir(folder)? {
        let fullpath = path?.path().display().to_string();

        // Add file to map
        if !std::path::Path::new(&fullpath).is_dir() {
            let file_size = std::fs::metadata(&fullpath)?.len();

            match size_filepath.get(&file_size) {
                Some(files) => {
                    let data = files.to_owned() + "?" + &fullpath;
                    size_filepath.insert(file_size, data);
                }
                None => {
                    size_filepath.insert(file_size, fullpath);
                }
            }
            continue;
        }
        add_files(&fullpath, size_filepath)?;
    }
    Ok(())
}

pub fn duplicate(folder: &str) -> Result<(), crate::processor::SyncError> {
    let mut size_filepath: Map<u64, String> = Default::default();
    let mut adler32_filepath: Map<u32, String> = Default::default();

    if !(std::path::Path::new(folder).exists() && std::path::Path::new(folder).is_dir()) {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            file: file!(),
            line: line!(),
            source: Some(folder.to_string()),
            destination: None,
        });
    }

    add_files(folder, &mut size_filepath)?;

    // Print empty files
    if let Some(files) = size_filepath.remove(&0) {
        let files_vector: Vec<&str> = files.split('?').collect();
        for file in files_vector.iter() {
            crate::processor::empty_msg(file);
        }
    }

    println!("");

    for (_, value) in &size_filepath {
        let file_count = value.matches("?").count();

        // 2 files, compare without hashing
        if file_count == 1 {
            let files: Vec<&str> = value.split('?').collect();
            if let Ok(()) = crate::processor::compare(files[0], files[1]) {
                crate::processor::duplicate_msgs(files);
            }

            continue;
        }

        if file_count > 1 {
            let files: Vec<&str> = value.split('?').collect();
            for file in files {
                let hash = crate::processor::adler32(file)?;

                match adler32_filepath.get(&hash) {
                    Some(paths) => {
                        let data = paths.to_owned() + "?" + file;
                        adler32_filepath.insert(hash, data);
                    }
                    None => {
                        adler32_filepath.insert(hash, file.to_string());
                    }
                }
            }
        }
    }

    size_filepath.clear();

    // Same size and same hash, very high probability to be the same file
    // Compare every byte
    for (_, value) in &adler32_filepath {
        compare_all_files_contents(value);
    }

    Ok(())
}
