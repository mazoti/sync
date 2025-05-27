use std::process::Command;
use std::io::{self, Write};

pub const FEATURES: &[&str] = &[
	// Base features
	"copy",
	"tree",
	"check-mt",

	// Combined base features
	"copy tree",
	"copy check-mt",
	"copy tree check-mt",
	"tree check-mt",

	// English variants
	"en",
	"en colored",
	"en copy",
	"en colored copy",
	"en tree",
	"en colored tree",
	"en check-mt",
	"en colored check-mt",
	"en copy tree",
	"en colored copy tree",
	"en copy check-mt",
	"en colored copy check-mt",
	"en copy tree check-mt",
	"en colored copy tree check-mt",
	"en tree check-mt",
	"en colored tree check-mt",

	// Brazilian Portuguese variants
	"br",
	"br colored",
	"br copy",
	"br colored copy",
	"br tree",
	"br colored tree",
	"br check-mt",
	"br colored check-mt",
	"br copy tree",
	"br colored copy tree",
	"br copy check-mt",
	"br colored copy check-mt",
	"br copy tree check-mt",
	"br colored copy tree check-mt",
	"br tree check-mt",
	"br colored tree check-mt"
];

fn main() -> io::Result<()> {
	// First run clippy without any features
	let output = Command::new("cargo").arg("clippy").output()?;
	io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

	// Then run clippy for each feature combination
	for feature in FEATURES {
		let output_features = Command::new("cargo").arg("clippy")
			.arg("--features").arg(feature).output()?;
		io::stdout().write_all(&output_features.stdout)?;
		io::stderr().write_all(&output_features.stderr)?;
	}

    Ok(())
}
