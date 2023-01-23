cargo clean
cargo audit
cargo fmt
cargo build --features en --release
cargo doc --features en
cargo tarpaulin -v
cargo tarpaulin -v --features en