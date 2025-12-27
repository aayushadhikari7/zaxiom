fn main() {
    // Only run on Windows
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();

        // Set the icon
        res.set_icon("assets/icon.ico");

        // Set version info
        res.set("ProductName", "Zaxiom");
        res.set("FileDescription", "A Linux-style terminal for Windows");
        res.set("LegalCopyright", "Copyright 2025");

        // Compile the resources
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {}", e);
            // Don't fail the build if icon is missing
        }
    }
}
