//! English string messages module for sync operations
//!
//! This module contains all user-facing messages used throughout the application,
//! organized by category for better maintainability.

pub mod errors {
    pub const CONFIG_DUPLICATED: &str = "source and destination already in config file";
    pub const CONFIG_EXT_CODE: &str = "config file not ended in .config";
    pub const CONFIG_FOLDER_CODE: &str = "config must be a .config text file";
    pub const COPY_FILE_FOLDER: &str = "cannot copy file to destination folder";
    pub const DEST_FILE: &str = "destination file exists";
    pub const DEST_NOT_FILE: &str = "source is a file and destination is a folder";
    pub const DEST_NOT_FOLDER: &str = "source is a folder and destination is a file";
    pub const DIFF_FILE_FOLDER: &str = "files or folders are different";
    pub const FILE_SIZE: &str = "file size must be positive";
    pub const IO: &str = "Input or output error";
    pub const OSSTRING: &str = "Operating system string error";
    pub const PARSE_INT: &str = "cannot convert number to integer";
    pub const PARSE_LINE: &str = "cannot parse line from config file";
    pub const SAME_FILE_FOLDER: &str = "source and destination are the same";
    pub const SOURCE_FILE: &str = "source file not found";
    pub const SOURCE_FOLDER: &str = "source folder not found";
    pub const SPLIT_SIZE: &str = "file does not need to split";
    pub const SYSTEM_TIME: &str = "system time error";
    pub const THREAD_JOIN: &str = "cannot join thread";
    pub const TRY_FROM_INT: &str = "cannot convert number to usize";
}

pub mod status {
    pub const CREATE: &str = "Creating";
    pub const COPY: &str = "Copying";
    pub const DUPLICATE: &str = "DUPLICATED";
    pub const ELAPSED: &str = "elapsed:";
    pub const EMPTY: &str = "(EMPTY)";
    pub const ERROR: &str = "ERROR";
    pub const LOADING: &str = "Loading";
    pub const OK: &str = "Ok";
    pub const ONE_ITEM: &str = "(ONE ITEM)";
    pub const REMOVE: &str = "Removing";
    pub const SIMULATION: &str = "(SIMULATION)";
    pub const START: &str = "started";
    pub const SYNC: &str = "Sync";
    pub const UPDATE: &str = "Updating";
    pub const USAGE: &str = "Usage:";
}

/// Message displayed when user enters "help" or sync didn't find any possible operation
pub const HELP: &str = "
	sync [source] [destination]
	sync [source] [destination] [file.config]
	sync [file.config] (or just sync if .config files are in the same folder)
	sync check [source] [destination]
	sync empty [folder]
	sync duplicate [folder]
	sync force [source] [destination]
	sync hash [folder] [file.hashs]
	sync hash [file.hashs]
	sync join [folder]
	sync move [source] [destination]
	sync split [size in bytes] [file]
	sync simulate [source] [destination]
";
