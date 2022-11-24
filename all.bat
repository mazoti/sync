cargo clean
cargo test
cargo test --features cli
cargo bench
cargo bench --features cli
cargo clippy
cargo clippy --features cli
cargo audit
cargo fmt
cargo build --features cli --release
cargo doc --features cli
cargo tarpaulin -v
cargo tarpaulin -v --features cli