fn main() {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", now);

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "unknown".to_string());
    eprintln!("[build.rs] CARGO_CFG_TARGET_OS = {}", target_os);

    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    match res.compile() {
        Ok(_) => eprintln!("[build.rs] winres compile OK"),
        Err(e) => eprintln!("[build.rs] winres error: {}", e),
    }
}
  
