## Building from sources
Make sure [Rust](https://www.rust-lang.org), git and cargo are in your PATH:
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
