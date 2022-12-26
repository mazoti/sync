cargo clean

cargo test
cargo test --features copy
cargo test --features en
cargo test --features "en copy"
cargo test --features br
cargo test --features "br copy"

cargo bench
cargo bench --features copy
cargo bench --features en
cargo bench --features "en copy"
cargo bench --features br
cargo bench --features "br copy"

cargo clippy
cargo clippy --features copy
cargo clippy --features en
cargo clippy --features "en copy"
cargo clippy --features br
cargo clippy --features "br copy"

cargo audit

cargo fmt
cargo build --features en --release
cargo doc --features en

cargo tarpaulin -v
cargo tarpaulin -v --features en