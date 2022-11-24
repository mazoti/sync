//! Contains all strings and error codes

pub const BUFFER_SIZE: usize = 1024 * 512; // 512KB, it will use 2 buffers, so 1MB

#[cfg(feature = "cli")]
include!("i18n/messages.rs");
//include!("i18n/messages_BR.rs");

//====================================== Exit codes to Operating System ======================================
#[cfg(feature = "cli")]
pub const HELP: i32 = -1;

pub const NO_ERROR: i32 = 0;

pub const ERROR_CONFIG_DUPLICATED: i32 = 1;
pub const ERROR_CONFIG_EXT_CODE: i32 = 2;
pub const ERROR_CONFIG_FOLDER_CODE: i32 = 3;
pub const ERROR_COPY_FILE_FOLDER: i32 = 4;
pub const ERROR_DEST_NOT_FILE: i32 = 5;
pub const ERROR_DEST_NOT_FOLDER: i32 = 6;
pub const ERROR_DIFF_FILE_FOLDER: i32 = 7;
pub const ERROR_FILE_SIZE: i32 = 8;
pub const ERROR_IO: i32 = 9;
pub const ERROR_PARSE_LINE: i32 = 10;
pub const ERROR_SAME_FILE_FOLDER: i32 = 11;
pub const ERROR_SOURCE_FILE: i32 = 12;
pub const ERROR_SOURCE_FOLDER: i32 = 13;
pub const ERROR_SPLIT_SIZE: i32 = 14;
pub const ERROR_SYSTEM_TIME: i32 = 15;
pub const ERROR_THREAD_JOIN: i32 = 16;

pub const PARSE_INT_ERROR: i32 = 17;

//====================================== Unit Tests ======================================

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(feature = "cli")]
    fn consts_tests() {
        for s in crate::processor::consts::ERROR_MSGS {
            println!("{}", s);
        }

        for s in crate::processor::consts::COMMAND_MSGS {
            println!("{}", s);
        }

        println!("{}", crate::processor::consts::MSG_HELP);

        for n in [
            crate::processor::consts::ERROR_CONFIG_DUPLICATED,
            crate::processor::consts::ERROR_CONFIG_EXT_CODE,
            crate::processor::consts::ERROR_CONFIG_FOLDER_CODE,
            crate::processor::consts::ERROR_COPY_FILE_FOLDER,
            crate::processor::consts::ERROR_DEST_NOT_FILE,
            crate::processor::consts::ERROR_DEST_NOT_FOLDER,
            crate::processor::consts::ERROR_DIFF_FILE_FOLDER,
            crate::processor::consts::ERROR_IO,
            crate::processor::consts::ERROR_PARSE_LINE,
            crate::processor::consts::ERROR_SAME_FILE_FOLDER,
            crate::processor::consts::ERROR_SOURCE_FOLDER,
            crate::processor::consts::ERROR_SYSTEM_TIME,
            crate::processor::consts::NO_ERROR,
            crate::processor::consts::HELP,
        ] {
            println!("{}", n);
        }
    }
}
