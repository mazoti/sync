## Building from sources
There are a couple of reasons to build from source code:
- Fix a bug
- Add a feature
- Learn how it works
- Optimize for your system
- Make a release for your architecture or operating system

Make sure **[Rust](https://www.rust-lang.org)**, **git** (optional) and **cargo** (installed with Rust) are in your PATH:
```sh
git --version
cargo --version
rustc --version
```
Clone or download the repository and build:
```bash
git clone https://github.com/mazoti/sync
cd sync
cargo build --features en --release (or br for portuguese)
```
The sync binary will be on *target/release* folder and ready to use. If you don't need any output you can build without command line interface (CLI) module:
```sh
cargo build --release
```

## Buffer size
When comparing files, *sync* will use 1MB of RAM. To change this, open the file *consts.rs*
and set the variable BUFFER_SIZE to the amount of bytes you want to use. The best value depends on your *hardware* and your operating system.

## Features

### Copy
Sync uses the operating system copy as default but has it's own:
```sh
cargo build --features "en copy" --release
```

### Tree
Sync uses hashmaps to find duplicated files, but instead it could use trees:
```sh
cargo build --features "en tree" --release
```
