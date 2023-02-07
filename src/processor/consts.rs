//! Contains all strings and error codes: users may change BUFFER_SIZE variables for better performance

/// Size of the buffer used by check methods
pub const CHECK_BUFFER_SIZE: u64 = 1024 * 512; // 512KB, but it will use 2 buffers

/// Size of the buffer used by copy method (if enabled by feature "copy")
pub const COPY_BUFFER_SIZE: u64 = 1024 * 512;

/// Size of the buffer used by hash method
pub const HASH_BUFFER_SIZE: u64 = 1024 * 512;

/// Size of the buffer used by join method
pub const JOIN_BUFFER_SIZE: u64 = 1024 * 512;

/// Size of the buffer used by split method
pub const SPLIT_BUFFER_SIZE: u64 = 1024 * 512;

//====================================== Exit codes to Operating System ======================================

/// Displays all commands and how to use them
#[cfg(feature = "i18n")]
pub const HELP: i32 = -1;

/// No messages to display
pub const NO_ERROR: i32 = 0;

/// Source and destination already in config file
pub const ERROR_CONFIG_DUPLICATED: i32 = 1;

/// Config file not ended in .config
pub const ERROR_CONFIG_EXT_CODE: i32 = 2;

/// Config must be a .config text file
pub const ERROR_CONFIG_FOLDER_CODE: i32 = 3;

/// Cannot copy file to destination folder
pub const ERROR_COPY_FILE_FOLDER: i32 = 4;

/// Destination file exists
pub const ERROR_DEST_FILE: i32 = 5;

/// Source is a file and destination is a folder
pub const ERROR_DEST_NOT_FILE: i32 = 6;

/// Source is a folder and destination is a file
pub const ERROR_DEST_NOT_FOLDER: i32 = 7;

/// Files or folders are different
pub const ERROR_DIFF_FILE_FOLDER: i32 = 8;

/// File size must be positive
pub const ERROR_FILE_SIZE: i32 = 9;

/// Input or output error
pub const ERROR_IO: i32 = 10;

/// Operating system string error
pub const ERROR_OSSTRING: i32 = 11;

/// Cannot convert number to integer
pub const ERROR_PARSE_INT: i32 = 12;

/// Cannot parse line from config file
pub const ERROR_PARSE_LINE: i32 = 13;

/// Source and destination are the same
pub const ERROR_SAME_FILE_FOLDER: i32 = 14;

/// Source file not found
pub const ERROR_SOURCE_FILE: i32 = 15;

/// Source folder not found
pub const ERROR_SOURCE_FOLDER: i32 = 16;

/// File does not need to split
pub const ERROR_SPLIT_SIZE: i32 = 17;

/// System time error (like negative time difference)
pub const ERROR_SYSTEM_TIME: i32 = 18;

/// Cannot join thread
pub const ERROR_THREAD_JOIN: i32 = 19;

/// Cannot convert number to usize
pub const ERROR_TRY_FROM_INT: i32 = 20;

//====================================== Unit Tests ======================================

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(feature = "i18n")]
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
