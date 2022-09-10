# **sync**
#### _A backup and synchronization tool_
![FreeBSD](https://img.shields.io/badge/-FreeBSD-%23870000?style=for-the-badge&logo=freebsd&logoColor=white)![NetBSD](https://img.shields.io/badge/NetBSD-FF6600.svg?style=for-the-badge&logo=NetBSD&logoColor=white)![OpenBSD](https://img.shields.io/badge/-OpenBSD-%23FCC771?style=for-the-badge&logo=openbsd&logoColor=black)![Alpine Linux](https://img.shields.io/badge/Alpine_Linux-%230D597F.svg?style=for-the-badge&logo=alpine-linux&logoColor=white)![Debian](https://img.shields.io/badge/Debian-D70A53?style=for-the-badge&logo=debian&logoColor=white)![Manjaro](https://img.shields.io/badge/Manjaro-35BF5C.svg?style=for-the-badge&logo=Manjaro&logoColor=white)![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)![macOS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=macos&logoColor=F0F0F0)

After years using [rsync](https://rsync.samba.org), I decided to make a simpler and portable tool to make my own backups, so why use sync?
- Open source
- Multi-language
- Low CPU/RAM usage
- Very fast (I/O bounded)
- No compression/cryptography
- Configuration files are optional
- Portable, small and easy to use
- Sustainable: releases are optimized to save power
- Does the minimum modifications (good for SSDs health and network traffic)
- Batch friendly: all operations on stdout, errors on stderr and return code to OS

## Instalation

All you need is the sync binary file of your architecture and operating system (just decompress and use).

Some releases can be downloaded [here](https://github.com/mazoti/sync/tree/main/download).
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
It will synchronize all files and folders in all .config files found in sync binary folder. If you need to check every byte of the whole process:
```bash
sync --check "source" "destination"
```

To keep synchronizing and checking until both operations succeed (will retry on any error), use the "--force" flag:
```bash
sync --force "source" "destination"
```

## Build from source
Instructions to build from source code can be found [here](https://github.com/mazoti/sync/blob/main/BUILD.md). It's easier and faster than you think!

## Translations
All strings are in the folder i18n (each file is a language) and can be translated. Now sync has two languages:

- English
- Portuguese

## Donations
You can become a sponsor of this project or donate directly:

BTC: 3JpkXivH11xQU37Lwk5TFBqLUo8gytLH84

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

**Thanks for your time and have fun!**
