fn main() -> std::io::Result<()> {
	#[cfg(windows)]
	let mut res = winres::WindowsResource::new();
	res.set("FileVersion", "1.0.0.0")
		.set("ProductVersion", "1.4.6.0")
		.set("FileDescription", "Sync data tool")
		.set("CompanyName", "Marcos Mazoti")
		.set("LegalCopyright", "Copyright © 2025 Marcos Mazoti")
		.set_icon("icon.ico")
		.set_language(0x0409)
		.compile()?;
	Ok(())
}
