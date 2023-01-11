cargo clean

cargo test
cargo test --features copy
cargo test --features tree
cargo test --features "copy tree"
cargo test --features en
cargo test --features "en copy"
cargo test --features "en tree"
cargo test --features "en copy tree"
cargo test --features br
cargo test --features "br copy"
cargo test --features "br tree"
cargo test --features "br copy tree"

cargo bench
cargo bench --features copy
cargo bench --features tree
cargo bench --features "copy tree"
cargo bench --features en
cargo bench --features "en copy"
cargo bench --features "en tree"
cargo bench --features "en copy tree"
cargo bench --features br
cargo bench --features "br copy"
cargo bench --features "br tree"
cargo bench --features "br copy tree"

cargo clippy
cargo clippy --features copy
cargo clippy --features tree
cargo clippy --features "copy tree"
cargo clippy --features en
cargo clippy --features "en copy"
cargo clippy --features "en tree"
cargo clippy --features "en copy tree"
cargo clippy --features br
cargo clippy --features "br copy"
cargo clippy --features "br tree"
cargo clippy --features "br copy tree"

cargo audit

cargo fmt
cargo build --features en --release
cargo doc --features en

cargo tarpaulin -v
cargo tarpaulin -v --features en