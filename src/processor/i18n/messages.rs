pub const MSG_HELP: &str = "
	sync [source] [destination]
	sync /home/user/data /home/user/backup
	sync \"C:\\My project\\file.dat\" \"D:\\Backup\\file.dat\"

If the destination does not exists, it will be created.
To see what sync would do without any modification,
add the \"--simulate\" parameter first:

    sync --simulate \"source\" \"destination\"

To synchronize more files or folders, create a config file:

	sync [source] [destination] [config_file.config]
	sync /home/user/folder /home/user/backup/folder backups.config
	sync /home/user/file.ext /home/user/backup/file.ext backups.config
	...

and run with the .config file as argument:

	sync backups.config

Multiple .config files are also supported:

	sync user1/folder   user1/backup/folder   sync_bin_folder/user1.config
	sync user1/file.ext user1/backup/file.ext sync_bin_folder/user1.config
	...
	sync user2/folder   user2/backup/folder   sync_bin_folder/user2.config
	sync user2/file.ext user2/backup/file.ext sync_bin_folder/user2.config
	...

the .config files must be in same folder as the sync binary; than run without arguments:

	sync

To check every folder, file and byte of the whole process:

	sync --check \"source\" \"destination\"

To keep synchronizing and checking until both operations succeed, use the \"--force\" flag:

	sync --force \"source\" \"destination\"
";

pub const ERROR_MSGS: &[&str] = &[
    "",                                              // NO_ERROR
    "source and destination already in config file", // ERROR_CONFIG_DUPLICATED
    "config file not ended in .config",              // ERROR_CONFIG_EXT_CODE
    "config must be a .config text file",            // ERROR_CONFIG_FOLDER_CODE
    "cannot copy file to destination folder",        // ERROR_COPY_FILE_FOLDER
    "destination file exists",                       // ERROR_DEST_FILE
    "source is a file and destination is a folder",  // ERROR_DEST_NOT_FILE
    "source is a folder and destination is a file",  // ERROR_DEST_NOT_FOLDER
    "files or folders are different",                // ERROR_DIFF_FILE_FOLDER
    "file size must be positive",                    // ERROR_FILE_SIZE
    "Input or output error",                         // ERROR_IO
    "Operating system string error",                 // ERROR_OSSTRING
    "cannot convert number to integer",              // ERROR_PARSE_INT
    "cannot parse line from config file",            // ERROR_PARSE_LINE
    "source and destination are the same",           // ERROR_SAME_FILE_FOLDER
    "source file not found",                         // ERROR_SOURCE_FILE
    "source folder not found",                       // ERROR_SOURCE_FOLDER
    "file does not need to split",                   // ERROR_SPLIT_SIZE
    "system time error",                             // ERROR_SYSTEM_TIME
    "cannot join thread",                            // ERROR_THREAD_JOIN
    "cannot convert number to usize",                // ERROR_TRY_FROM_INT
];

pub const COMMAND_MSGS: &[&str] = &[
    "Creating",
    " Copying",
    "Elapsed",
    "ERROR",
    " Loading",
    "      Ok",
    "Removing",
    "started",
    "    Sync",
    "Updating",
    "Usage:",
    " WARNING",
    "(SIMULATION)",
    "(EMPTY)",
    "(ONE ITEM)",
    "DUPLICATED",
];
