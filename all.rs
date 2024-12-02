use std::process::Command;
use std::io::{self, Write};

pub const COMMANDS: &[&str] = &[
	"cargo clean",
	"cargo audit",
	"cargo fmt",
	"cargo build --features en --release",
	"cargo doc --features en",
	"cargo tarpaulin -v",
	"cargo tarpaulin -v --features en"
];


fn main() -> io::Result<()> {
	for command in COMMANDS {
		let args: Vec<&str> = command.split_whitespace().collect();
		let output = Command::new(args[0]).args(&args[1..]).output()?;

		io::stdout().write_all(&output.stdout)?;
		io::stderr().write_all(&output.stderr)?;
	}

    Ok(())
}
