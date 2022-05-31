# **sync**
#### _A backup and synchronization tool_
![FreeBSD](https://img.shields.io/badge/-FreeBSD-%23870000?style=for-the-badge&logo=freebsd&logoColor=white)![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

After years using [rsync](https://rsync.samba.org), I decided to make a simpler and portable tool to make my own backups, because I didn' t find anything that fits exactly on my needs. So why use it?
- Portable, small and easy to use
- Low CPU/RAM usage
- No compression/cryptography
- Very fast (I/O bounded)
- Does the minimum modifications (good for SSDs health)
- Batch friendly: all operations on stdout, errors on stderr and return code to OS

## Instalation
Some releases can be downloaded [here](https://github.com/mazoti/sync/tree/main/download).
All you need is the sync binary file of your architecture and operating system.
## Usage
For a simple backup/restore:
```bash
sync "source" "destination" (backup)
sync "destination" "source" (restore, just invert the order!)
```
If the destination exists, sync will remove files and folders not found in source and add or update existing files and folders. To backup multiple files and folders, create a config file (any filename ending in .config):
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

If you want sync to keep synchronizing and checking until both operations succeed (will retry on any error), use the "--force" flag:
```bash
sync --force "source" "destination"
```
You can change the buffer size in the file const.rs for a better performance

## Building from sources
Make sure the last [Rust](https://www.rust-lang.org) stable compiler is in your PATH:
```sh
rustup self update
rustup update
```
Clone or download the repository and build:
```bash
git clone https://github.com/mazoti/sync
cd sync
cargo build --features cli --release
```
The sync binary will be on target/release folder and ready to use. If you don't need any output you can build without command line interface (CLI) module:
```sh
cargo build --release
```
## Optional
To run tests, lint, formatter and generate documentation:
```sh
cargo test --features cli -- --test-threads=1
cargo clippy --features cli
cargo fmt
cargo doc --features cli
```
## Translations
All strings are in the folder i18n (each file is a language) and can be translated.

## Donations
BTC: 3JpkXivH11xQU37Lwk5TFBqLUo8gytLH84


**Thanks for your time and have fun!**