//! Contains all strings and error codes: users may change BUFFER_SIZE variables for better performance

/// Size of the buffer used by check methods
pub const CHECK_BUFFER_SIZE: u64 = 1024 * 512; // 512KB, but it will use 2 buffers

/// Size of the buffer used by copy method
pub const COPY_BUFFER_SIZE: u64 = 1024 * 512;

/// Size of the buffer used by hash method
pub const HASH_BUFFER_SIZE: u64 = 1024 * 512;

/// Size of the buffer used by join method
pub const JOIN_BUFFER_SIZE: u64 = 1024 * 512;

/// Size of the buffer used by split method
pub const SPLIT_BUFFER_SIZE: u64 = 1024 * 512;

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
