//! SyncError class implementations

use crate::processor::SyncError;

/// Process input or outpur errors like "File not found"
impl From<std::io::Error> for SyncError {
    fn from(_error: std::io::Error) -> Self {
        #[cfg(debug_assertions)]
        println!("===> {_error:?} <===");

        SyncError {
            code: crate::processor::ErrorCode::ErrorIO,
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
        println!("===> {_error:?} <===");

        SyncError {
            code: crate::processor::ErrorCode::ErrorSystemTime,
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        }
    }
}

/// The way a SyncError will be shown on user screen
#[cfg(feature = "i18n")]
impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(f, "{}", self.code)?)
    }
}

/// The way a SyncError will be shown on user screen with debug mode "{:?}"
#[cfg(feature = "i18n")]
impl std::fmt::Debug for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "SyenError:\nCode: {}\nFile: {}\nLine: {}",
            self.code, self.file, self.line
        )?;

        if let Some(src) = &self.source {
            writeln!(f, "Source: {src}")?;
        }

        if let Some(dest) = &self.destination {
            writeln!(f, "Destination: {dest}")?;
        }

        Ok(())
    }
}

/// Process errors converting string to integers
impl From<std::num::ParseIntError> for SyncError {
    fn from(_error: std::num::ParseIntError) -> Self {
        #[cfg(debug_assertions)]
        println!("===> {_error:?} <===");

        SyncError {
            code: crate::processor::ErrorCode::ErrorParseInt,
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
        println!("===> {_error:?} <===");

        SyncError {
            code: crate::processor::ErrorCode::ErrorTryFromInt,
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
        println!("===> {_error:?} <===");

        SyncError {
            code: crate::processor::ErrorCode::ErrorOSString,
            file: file!(),
            line: line!(),
            source: None,
            destination: None,
        }
    }
}
