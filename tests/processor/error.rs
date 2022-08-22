//! SyncError class implementations

/// Returns a String given an error code
#[cfg(feature = "cli")]
#[inline]
pub fn error_to_string(code: i32) -> Option<String> {
    if (code < 1) || (code as usize) >= (crate::processor::consts::ERROR_MSGS.len() - 1) {
        return None;
    }

    Some(String::from(
        crate::processor::consts::ERROR_MSGS[(code as usize) - 1],
    ))
}

/// Returns a String given an error code
#[cfg(not(feature = "cli"))]
#[inline]
pub fn error_to_string(_code: i32) -> Option<String> {
    None
}

/// Error class with the message and code defined in consts.rs.
/// "code" is the number returned to operating system, "message" is the error displayed in stderr,
/// "file" is the source code file and "line" is the line number of the error
pub struct SyncError {
    pub code: i32,
    pub message: Option<String>,
    pub file: &'static str,
    pub line: u32,
    pub source: Option<String>,
    pub destination: Option<String>,
}

impl From<std::io::Error> for SyncError {
    fn from(error: std::io::Error) -> Self {
        SyncError {
            code: crate::processor::consts::ERROR_IO,
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
            code: crate::processor::consts::ERROR_SYSTEM_TIME,
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
