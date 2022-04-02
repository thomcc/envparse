fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // 1.48 is MSRV
    let version = rustc_version().unwrap_or(48);
    if version < 57 {
        println!("cargo:rustc-cfg=envparse_no_const_panic");
    }
}

fn rustc_version() -> Option<u32> {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| std::ffi::OsString::from("rustc"));
    let output = std::process::Command::new(rustc)
        .arg("--version")
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!(
            "cargo:warning=`rustc --version` gave error status {:?} (stderr: {:?}, stdout: {:?})",
            output.status, &stderr, &stdout,
        );
    }
    let mut components = stdout.trim().split('.');
    let rustc_1 = components.next();
    let minor = components
        .next()
        .and_then(|minor| minor.parse::<u32>().ok());
    match (rustc_1, minor) {
        (Some(s), Some(v)) if s.ends_with("rustc 1") => {
            return Some(v);
        }
        (_, ver) => {
            println!(
                "cargo:warning=`rustc --version` output is weird: {:?}",
                &stdout,
            );
            // Optimistically using it anyway.
            ver
        }
    }
}
