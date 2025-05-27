use std::process::Command;
use std::io::{self, Write};

//rustc --print target-list
pub const TARGETS: &[&str] = &[
	"aarch64-unknown-linux-gnu",          
	"aarch64-unknown-linux-musl",         
	"aarch64-unknown-uefi",            
	"arm-linux-androideabi",           
	"arm-unknown-linux-musleabi",     
	"arm-unknown-linux-musleabihf", 
	"armv5te-unknown-linux-gnueabi", 
	"armv5te-unknown-linux-musleabi",  
	"armv7-linux-androideabi",        
	"armv7-unknown-linux-musleabihf", 
	"i586-pc-windows-msvc",
	"i686-pc-windows-msvc",
	"i686-unknown-uefi",
 	"powerpc64le-unknown-linux-gnu",
 	"powerpc64-unknown-linux-gnu",
	"powerpc-unknown-linux-gnu",
	"riscv64gc-unknown-linux-gnu",
	"s390x-unknown-linux-gnu",
 	"sparc64-unknown-linux-gnu",	
	"sparcv9-sun-solaris",
 	"thumbv7neon-linux-androideabi",
	"x86_64-fortanix-unknown-sgx",
	"x86_64-pc-windows-msvc",
	"x86_64-unknown-illumos",
	"x86_64-unknown-linux-gnu",
	"x86_64-unknown-linux-musl",
	"x86_64-unknown-uefi",
];

fn main() -> io::Result<()> {
	for target in TARGETS {
		let output_rustup = Command::new("rustup")
			.arg("target")
			.arg("add")
			.arg(target)
			.output()?;
		io::stdout().write_all(&output_rustup.stdout)?;
		io::stderr().write_all(&output_rustup.stderr)?;

		let output = Command::new("cross")
			.arg("build")
			.arg("--target")
			.arg(target)
			.arg("--features")
			.arg("en colored")
			.arg("--release").output()?;
		io::stdout().write_all(&output.stdout)?;
		io::stderr().write_all(&output.stderr)?;
	}

    Ok(())
}
