use std::io::Read;

pub fn adler32(file: &str, buffer_size: u64) -> Result<u32, crate::processor::SyncError> {
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

    let metadata_file = std::fs::metadata(file)?;
    let mut hash_file = std::fs::File::open(file)?;

    if metadata_file.len() <= buffer_size {
        let bytes_read = hash_file.read(&mut buffer)?;
        let mut i: usize = 0;
        while i < bytes_read {
            a = (a + (buffer[i] as u32)) % 65521;
            b = (b + a) % 65521;

            i += 1;
        }

        return Ok((b << 16) | a);
    }

    let mut bytes_read: usize;
    loop {
        bytes_read = hash_file.read(&mut buffer)?;
        let mut i: usize = 0;
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

fn sha256_hash(filepath: &str) -> Result<String, crate::processor::SyncError> {
    if !std::path::Path::new(filepath).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: None,
            destination: Some(filepath.to_string()),
        });
    }

    if !std::path::Path::new(filepath).is_file() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: Some(filepath.to_string()),
            destination: None,
        });
    }

    let input = std::path::Path::new(filepath);
    Ok(sha256::try_digest(input).unwrap())
}

pub fn hash_file(file: &str) -> Result<(), crate::processor::SyncError> {
    if !std::path::Path::new(file).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: None,
            destination: Some(file.to_string()),
        });
    }

    if !std::path::Path::new(file).is_file() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_file(),
            file: file!(),
            line: line!(),
            source: Some(file.to_string()),
            destination: None,
        });
    }

    let input = std::path::Path::new(file);
    let val = sha256::try_digest(input).unwrap();

    println!("{} => {}", val, file);

    Ok(())
}

pub fn hash_folder(source: &str, destination: &str) -> Result<(), crate::processor::SyncError> {
    fn hash(
        source_folder: &str,
        destination_file: &str,
    ) -> Result<(), crate::processor::SyncError> {
        for path in std::fs::read_dir(source_folder)? {
            let fullpath = path?.path().display().to_string();

            if !std::fs::metadata(&fullpath)?.is_dir() {
                let hash_str = sha256_hash(&fullpath)?;
                crate::processor::create(&fullpath, &hash_str, &destination_file)?;
                continue;
            }

            // Create destination folder and copy directories recursively
            hash(&fullpath, destination_file)?;
        }
        Ok(())
    }

    if !std::path::Path::new(source).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if !std::path::Path::new(source).is_dir() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_source_folder(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    if std::path::Path::new(destination).exists() {
        return Err(crate::processor::SyncError {
            code: crate::processor::error_dest_file(),
            file: file!(),
            line: line!(),
            source: Some(source.to_string()),
            destination: Some(destination.to_string()),
        });
    }

    hash(source, destination)
}
