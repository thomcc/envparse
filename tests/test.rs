use compiletest_rs::{Config, common::Mode, run_tests};

fn run_mode(mode: &'static str) {
    let root = std::env::var_os("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| env!("CARGO_MANIFEST_DIR").into());
    let bless = std::env::var_os("ENVPARSE_BLESS").is_some();
    let src_base = root.join(format!("uitests/{mode}"));
    let mode = mode.parse::<Mode>().expect("invalid mode");
    let target_rustcflags = if std::env::var_os("ENVPARSE_MACRO_BACKTRACE").is_some() {
        Some("-Dwarnings -Zmacro-backtrace".into())
    } else {
        Some("-Dwarnings".into())
    };
    let mut config = Config {
        mode,
        edition: Some("2021".into()),
        bless,
        src_base,
        target_rustcflags,
        // rustc_path: std::path::PathBuf::from("rustc"),
        ..Config::default()
    };
    link_deps(&mut config);
    config.clean_rmeta();
    run_tests(&config);
}

fn link_deps(config: &mut Config) {
    config.link_deps();
    // config.link_deps doesn't handle DYLD_FALLBACK_LIBRARY_PATH
    if cfg!(target_os = "macos") {
        let varname = "DYLD_FALLBACK_LIBRARY_PATH";
        let lib_paths = std::env::var_os(varname).unwrap_or_default();
        let mut flags = config.target_rustcflags.take().unwrap_or_default();
        // `flags += lib_paths_flags(&lib_paths).as_str();`
        if !lib_paths.is_empty() {
            for p in std::env::split_paths(&lib_paths) {
                let p = p.to_str().unwrap();
                assert!(!p.contains(' '), "spaces in paths not supported: {}", p);
                flags += " -L ";
                flags += p;
            }
        }
        config.target_rustcflags = Some(flags);
    }
}

#[test]
fn run_pass() {
    // TODO: compile-fail tests
    run_mode("run-pass");
}

#[test]
fn compile_fail() {
    run_mode("compile-fail");
}
