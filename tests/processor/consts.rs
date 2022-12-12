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
        for s in crate::processor::error_msgs() {
            println!("{}", s);
        }

        for s in 0..13 {
            println!("{}", crate::processor::command_msgs(s));
        }

        println!("{}", crate::processor::msg_help());

        for n in [
            crate::processor::error_config_duplicated(),
            crate::processor::error_config_ext_code(),
            crate::processor::error_config_folder_code(),
            crate::processor::error_copy_file_folder(),
            crate::processor::error_dest_not_file(),
            crate::processor::error_dest_not_folder(),
            crate::processor::error_diff_file_folder(),
            crate::processor::error_io(),
            crate::processor::error_parse_line(),
            crate::processor::error_same_file_folder(),
            crate::processor::error_source_folder(),
            crate::processor::error_system_time(),
            crate::processor::no_error(),
            crate::processor::help(),
        ] {
            println!("{}", n);
        }
    }
}
