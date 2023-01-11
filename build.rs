fn main() -> std::io::Result<()> {
    #[cfg(windows)]
    {
        winres::WindowsResource::new()
            .set_icon("icon.ico")
            .compile()?;
    }
    Ok(())
}
