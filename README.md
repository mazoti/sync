# **sync**
#### _A backup and synchronization tool_
![FreeBSD](https://img.shields.io/badge/-FreeBSD-%23870000?style=for-the-badge&logo=freebsd&logoColor=white)![Debian](https://img.shields.io/badge/Debian-D70A53?style=for-the-badge&logo=debian&logoColor=white)![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)![macOS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=macos&logoColor=F0F0F0)

After years using [rsync](https://rsync.samba.org), I decided to make a simpler and portable tool to make my own backups, so why use sync?
- Open source
- Multi-language
- Low CPU/RAM usage
- Very fast (I/O bounded)
- Configuration files are optional
- Portable, small and easy to use
- Does the minimum modifications (good for SSDs health and network traffic)
- Batch friendly: all operations on stdout, errors on stderr and return code to OS

## Installation

All you need is the sync binary file of your architecture and operating system (just decompress and use).

Releases can be downloaded [here](https://github.com/mazoti/sync/tree/main/download).
## Usage
For a simple backup/restore:
```bash
sync "source" "destination" (backup)
sync "destination" "source" (restore, just invert the order!)
```
If the destination exists, sync will remove files and folders not found in source and add or update existing files and folders.
To see what sync would do without any modification, add the "--simulate" parameter first:
```bash
sync --simulate "source" "destination"
```
To backup multiple files and folders, create a config file (any filename ending in .config):
```bash
sync "source" "destination" "My backup.config" (creates "My backup.config" file)
sync "another file or folder" "another destination" "My backup.config" (adds to "My backup.config")
...
```
and pass the .config file as an argument:
```bash
sync "My backup.config" (synchronize "source folder", "another file or folder", ...)
```
You can also use multiple config files, just put them in the sync binary folder. Ex: "user1.config", "user2.config" and run sync without arguments:
```bash
sync
```
It will synchronize all files and folders in all .config files found in sync binary folder with one thread each config file.

If you need to check every byte of the whole process:
```bash
sync --check "source" "destination"
```

To keep synchronizing and checking until both operations succeed (will retry on any error), use the "--force" flag:
```bash
sync --force "source" "destination"
```

To search for duplicated files in a folder, enter the "--duplicate" flag:
```bash
sync --duplicate "folder"
```

Empty files or folders with only one file or folder could be found with "--empty":
```bash
sync --empty "folder"
```

For security, you can generate a file with all SHA-256 hashes of all files in a folder with "--hash":
```bash
sync --hash "folder" "file.hash"
```

With the same flag, you can check if any file was modified (hash will be different):
```bash
sync --hash "file.hash"
```

To split a file, use the "--split" and the size of each file in bytes:
```bash
sync --split 1024 "data.file" (sync will create data.file.0, data.file.1... all with 1024 bytes) 
```

To join files, use just "--join" (run in the data.file.0, data.file.1... folder path):
```bash
sync --join 
```

The move feature is different from the operating system: it will copy the source to destination,
check each byte and remove the source if no errors were found:
```bash
sync --move "source file" "destination file" 
sync --move "source folder" "destination folder"
```


## Aliases

### Check:
```
--CHECK, --check, -C, -CHECK, -c, -check, /C, /CHECK, /c, /check, CHECK, check
```
### Duplicate:
```
--DUPLICATE, --duplicate, -D, -DUPLICATE, -d, -duplicate, /D, /DUPLICATE, /d, /duplicate, DUPLICATE, duplicate
```
### Empty:
```
--EMPTY, --empty, -E, -EMPTY, -e, -empty, /E, /EMPTY, /e, /empty, EMPTY, empty
```
### Force:
```
--FORCE, --force, -F, -FORCE, -f, -force, /F, /FORCE, /f, /force, FORCE, force
```
### Hash:
```
--HASH, --hash, -HASH, -hash, /HASH, /hash, HASH, hash
```
### Help:
```
--HELP, --help, -?, -H, -h, -help, /?, /H, /HELP, /h, /help, HELP, help
```
### Join
```
--JOIN, --join, -J, -JOIN, -j, -join, /J, /JOIN, /j, /join, JOIN, join
```
### Move
```
--MOVE, --move, -M, -MOVE, -m, -move, /M, /MOVE, /m, /move, MOVE, move
```
### Simulate
```
--SIMULATE, --simulate, -S, -SIMULATE, -s, -simulate, /S, /SIMULATE, /s, /simulate, SIMULATE, simulate
```
### Split
```
--SPLIT, --split, -S, -SPLIT, -s, -split, /S, /SPLIT, /s, /split, SPLIT, split
```
### Version
```
--VERSION, --version, -V, -VERSION, -v, -version, /V, /VERSION, /v, /version, VERSION, version
```

## Build from source
Instructions to build from source code can be found [here](https://github.com/mazoti/sync/blob/main/BUILDING.md). It's easier and faster than you think!

## Translations
All strings are in the folder i18n (each file is a language) and can be translated. Now sync has two languages:

- English
- Portuguese

## Donations
You can become a [sponsor](https://github.com/sponsors/mazoti) or donate directly:

BTC: 3JpkXivH11xQU37Lwk5TFBqLUo8gytLH84

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

**Thanks for your time and have fun!**
