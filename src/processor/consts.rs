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
        println!("{}", crate::processor::i18n::messages::errors::CONFIG_DUPLICATED);
        println!("{}", crate::processor::i18n::messages::errors::CONFIG_EXT_CODE);
        println!("{}", crate::processor::i18n::messages::errors::CONFIG_FOLDER_CODE);
        println!("{}", crate::processor::i18n::messages::errors::COPY_FILE_FOLDER);
        println!("{}", crate::processor::i18n::messages::errors::DEST_FILE);
        println!("{}", crate::processor::i18n::messages::errors::DEST_NOT_FILE);
        println!("{}", crate::processor::i18n::messages::errors::DEST_NOT_FOLDER);
        println!("{}", crate::processor::i18n::messages::errors::DIFF_FILE_FOLDER);
        println!("{}", crate::processor::i18n::messages::errors::FILE_SIZE);
        println!("{}", crate::processor::i18n::messages::errors::IO);
        println!("{}", crate::processor::i18n::messages::errors::OSSTRING);
        println!("{}", crate::processor::i18n::messages::errors::PARSE_INT);
        println!("{}", crate::processor::i18n::messages::errors::PARSE_LINE);
        println!("{}", crate::processor::i18n::messages::errors::SAME_FILE_FOLDER);
        println!("{}", crate::processor::i18n::messages::errors::SOURCE_FILE);
        println!("{}", crate::processor::i18n::messages::errors::SOURCE_FOLDER);
        println!("{}", crate::processor::i18n::messages::errors::SPLIT_SIZE);
        println!("{}", crate::processor::i18n::messages::errors::SYSTEM_TIME);
        println!("{}", crate::processor::i18n::messages::errors::THREAD_JOIN);
        println!("{}", crate::processor::i18n::messages::errors::TRY_FROM_INT);


        println!("{}", crate::processor::i18n::messages::status::CREATE);
        println!("{}", crate::processor::i18n::messages::status::COPY);
        println!("{}", crate::processor::i18n::messages::status::DUPLICATE);
        println!("{}", crate::processor::i18n::messages::status::ELAPSED);
        println!("{}", crate::processor::i18n::messages::status::EMPTY);
        println!("{}", crate::processor::i18n::messages::status::ERROR);
        println!("{}", crate::processor::i18n::messages::status::LOADING);
        println!("{}", crate::processor::i18n::messages::status::OK);
        println!("{}", crate::processor::i18n::messages::status::ONE_ITEM);
        println!("{}", crate::processor::i18n::messages::status::REMOVE);
        println!("{}", crate::processor::i18n::messages::status::SIMULATION);
        println!("{}", crate::processor::i18n::messages::status::START);
        println!("{}", crate::processor::i18n::messages::status::SYNC);
        println!("{}", crate::processor::i18n::messages::status::UPDATE);
        println!("{}", crate::processor::i18n::messages::status::USAGE);

        println!("{}", crate::processor::i18n::messages::HELP);
    }
}
