fn run_mode(mode: &'static str) {
    use compiletest_rs::{common::Mode, Config};
    let root = std::env::var_os("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| env!("CARGO_MANIFEST_DIR").into());
    let bless = std::env::var_os("ENVPARSE_BLESS").is_some();
    let src_base = root.join(format!("uitests/{mode}"));
    let mode = mode.parse::<Mode>().expect("invalid mode");
    let target_rustcflags = Some("-Dwarnings -L target/debug".into());
    let mut config = Config {
        mode,
        edition: Some("2021".into()),
        bless,
        src_base,
        target_rustcflags,
        // rustc_path: std::path::PathBuf::from("rustc"),
        ..Config::default()
    };

    config.link_deps(); // Populate config.target_rustcflags with dependencies on the path
    config.clean_rmeta(); // If your tests import the parent crate, this helps with E0464
    compiletest_rs::run_tests(&config);
}

#[test]
fn compile_test() {
    // run_mode("compile-fail");
    run_mode("run-pass");
}
