//! SyncError class implementations

use crate::processor::SyncError;

/// Returns a String given an error code
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn error_to_string(code: i32) -> Option<String> {
    let code_usize: Result<usize, std::num::TryFromIntError> = code.try_into();
    match code_usize {
        Ok(i) => {
            if i < crate::processor::error_msgs().len() {
                return Some(String::from(crate::processor::error_msgs()[i]));
            }
            None
        }
        Err(_) => None,
    }
}

/// Returns a String given an error code
#[cfg(not(feature = "i18n"))]
#[inline(always)]
pub fn error_to_string(_code: i32) -> Option<String> {
    None
}

/// Process input or outpur errors like "File not found"
impl From<std::io::Error> for SyncError {
    fn from(_error: std::io::Error) -> Self {
        #[cfg(debug_assertions)]
        println!("===> {:?} <===", _error);

        SyncError {
            code: crate::processor::error_io(),
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        }
    }
}

/// Process system time errors like negative differences between times
impl From<std::time::SystemTimeError> for SyncError {
    fn from(_error: std::time::SystemTimeError) -> Self {
        #[cfg(debug_assertions)]
        println!("===> {:?} <===", _error);

        SyncError {
            code: crate::processor::error_system_time(),
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        }
    }
}

/// The way a SyncError will be shown on user screen
impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = error_to_string(self.code) {
            write!(f, "{}", msg)?;
        }
        Ok(())
    }
}

/// The way a SyncError will be shown on user screen with debug mode "{:?}"
impl std::fmt::Debug for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "SyenError:\nCode: {}\nFile: {}\nLine: {}",
            self.code, self.file, self.line
        )?;

        if let Some(msg) = error_to_string(self.code) {
            writeln!(f, "Message: {}", msg)?;
        }

        if let Some(src) = &self.source {
            writeln!(f, "Source: {}", src)?;
        }

        if let Some(dest) = &self.destination {
            writeln!(f, "Destination: {}", dest)?;
        }

        Ok(())
    }
}

/// Process errors converting string to integers
impl From<std::num::ParseIntError> for SyncError {
    fn from(_error: std::num::ParseIntError) -> Self {
        #[cfg(debug_assertions)]
        println!("===> {:?} <===", _error);

        SyncError {
            code: crate::processor::error_parse_int(),
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        }
    }
}

/// Process errors converting integers to usize
impl From<std::num::TryFromIntError> for SyncError {
    fn from(_error: std::num::TryFromIntError) -> Self {
        #[cfg(debug_assertions)]
        println!("===> {:?} <===", _error);

        SyncError {
            code: crate::processor::error_try_from_int(),
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        }
    }
}

/// Process errors converting operating system strings
impl From<std::ffi::OsString> for SyncError {
    fn from(_error: std::ffi::OsString) -> Self {
        #[cfg(debug_assertions)]
        println!("===> {:?} <===", _error);

        SyncError {
            code: crate::processor::error_os_string(),
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        }
    }
}
