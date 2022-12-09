//! SyncError class implementations

use crate::processor::SyncError;

/// Returns a String given an error code
#[cfg(feature = "cli")]
#[inline]
pub fn error_to_string(code: i32) -> Option<String> {
    if (code < 1) || (code as usize) >= (crate::processor::error_msgs().len() - 1) {
        return None;
    }

    Some(String::from(
        crate::processor::error_msgs()[(code as usize) - 1],
    ))
}

/// Returns a String given an error code
#[cfg(not(feature = "cli"))]
#[inline(always)]
pub fn error_to_string(_code: i32) -> Option<String> {
    None
}

impl From<std::io::Error> for SyncError {
    fn from(error: std::io::Error) -> Self {
        SyncError {
            code: crate::processor::error_io(),
            message: Some(error.to_string()),
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        }
    }
}

impl From<std::time::SystemTimeError> for SyncError {
    fn from(error: std::time::SystemTimeError) -> Self {
        SyncError {
            code: crate::processor::error_system_time(),
            message: Some(error.to_string()),
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        }
    }
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = &self.message {
            write!(f, "{}", msg)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "SyenError:\nCode: {}\nFile: {}\nLine: {}",
            self.code, self.file, self.line
        )?;

        if let Some(msg) = &self.message {
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

impl From<std::num::ParseIntError> for SyncError {
    fn from(error: std::num::ParseIntError) -> Self {
        SyncError {
            code: crate::processor::parse_int_error(),
            message: Some(error.to_string()),
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        }
    }
}
