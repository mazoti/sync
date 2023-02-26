//! English string messages crate

/// "Copying"
pub const COPY_MSG: &str = "Copying";

/// "Creating"
pub const CREATE_MSG: &str = "Creating";

/// "DUPLICATED"
pub const DUPLICATE_MSG: &str = "DUPLICATED";

/// "elapsed:"
pub const ELAPSE_MSG: &str = "elapsed:";

/// "(EMPTY)"
pub const EMPTY_MSG: &str = "(EMPTY)";

/// "ERROR"
pub const ERROR_MSG: &str = "ERROR";

/// Message displayed when user enters "help" or sync didn't find any possible operation
pub const HELP_MSG: &str = "
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

/// "Loading"
pub const LOADING_MSG: &str = "Loading";

/// "Ok"
pub const OK_MSG: &str = "Ok";

/// "(ONE ITEM)"
pub const ONE_ITEM_MSG: &str = "(ONE ITEM)";

/// "Removing"
pub const REMOVE_MSG: &str = "Removing";

/// "(SIMULATION)"
pub const SIMULATION_MSG: &str = "(SIMULATION)";

/// "started"
pub const START_MSG: &str = "started";

/// "Sync"
pub const SYNC_MSG: &str = "Sync";

/// "Updating"
pub const UPDATE_MSG: &str = "Updating";

/// "Usage:"
pub const USAGE_MSG: &str = "Usage:";

/// "source and destination already in config file"
pub const ERROR_CONFIG_DUPLICATED: &str = "source and destination already in config file";

/// "config file not ended in .config"
pub const ERROR_CONFIG_EXT_CODE: &str = "config file not ended in .config";

/// "config must be a .config text file"
pub const ERROR_CONFIG_FOLDER_CODE: &str = "config must be a .config text file";

/// "cannot copy file to destination folder"
pub const ERROR_COPY_FILE_FOLDER: &str = "cannot copy file to destination folder";

/// "destination file exists"
pub const ERROR_DEST_FILE: &str = "destination file exists";

/// "source is a file and destination is a folder"
pub const ERROR_DEST_NOT_FILE: &str = "source is a file and destination is a folder";

/// "source is a folder and destination is a file"
pub const ERROR_DEST_NOT_FOLDER: &str = "source is a folder and destination is a file";

/// "files or folders are different"
pub const ERROR_DIFF_FILE_FOLDER: &str = "files or folders are different";

/// "file size must be positive"
pub const ERROR_FILE_SIZE: &str = "file size must be positive";

/// "Input or output error"
pub const ERROR_IO: &str = "Input or output error";

/// "Operating system string error"
pub const ERROR_OSSTRING: &str = "Operating system string error";

/// "cannot convert number to integer"
pub const ERROR_PARSE_INT: &str = "cannot convert number to integer";

/// "cannot parse line from config file"
pub const ERROR_PARSE_LINE: &str = "cannot parse line from config file";

/// "source and destination are the same"
pub const ERROR_SAME_FILE_FOLDER: &str = "source and destination are the same";

/// "source file not found"
pub const ERROR_SOURCE_FILE: &str = "source file not found";

/// "source folder not found"
pub const ERROR_SOURCE_FOLDER: &str = "source folder not found";

/// "file does not need to split"
pub const ERROR_SPLIT_SIZE: &str = "file does not need to split";

/// "system time error"
pub const ERROR_SYSTEM_TIME: &str = "system time error";

/// "cannot join thread"
pub const ERROR_THREAD_JOIN: &str = "cannot join thread";

/// "cannot convert number to usize"
pub const ERROR_TRY_FROM_INT: &str = "cannot convert number to usize";
