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
cargo build --features cli --release
```
The sync binary will be on *target/release* folder and ready to use. If you don't need any output you can build without command line interface (CLI) module:
```sh
cargo build --release
```

## Translation
To create a translation, use the file *src/processor/i18n/messages.rs* as a template and
change the **include!("i18n/messages.rs")** to your file in *consts.rs*.

## Buffer size
When comparing files, *sync* will use 1MB of RAM. To change this, open the file *consts.rs*
and set the variable BUFFER_SIZE to the amount of bytes you want to use. The best value depends on your *hardware*.

## Optimization flags
Default releases are optimized to save power, but you can change it in the file *Cargo.toml*:
```sh
...
opt-level = 3
...
```
This will optimize for speed. Remember that *sync* is **I/O bounded**, any optimization here will be very small.

## Optional
To run tests, lint, generate documentation and formatter:
```sh
cargo test --features cli -- --test-threads=1
cargo clippy --features cli
cargo doc --features cli
cargo fmt
```
