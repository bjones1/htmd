//! Records the htmd and pulldown-cmark versions this build uses, which the
//! page header shows (spec, Page layout)

use std::fs;

fn locked_version(lock: &str, package: &str) -> String {
    let marker = format!("name = \"{package}\"\nversion = \"");
    let start = lock
        .find(&marker)
        .unwrap_or_else(|| panic!("{package} is not in Cargo.lock"))
        + marker.len();
    lock[start..].split('"').next().unwrap().to_string()
}

fn main() {
    let lock_path = "../Cargo.lock";
    println!("cargo::rerun-if-changed={lock_path}");
    // Cargo.lock uses the platform's line endings when git converts them.
    let lock = fs::read_to_string(lock_path)
        .expect("read Cargo.lock")
        .replace("\r\n", "\n");
    for (package, variable) in [
        ("htmd", "HTMD_VERSION"),
        ("pulldown-cmark", "PULLDOWN_CMARK_VERSION"),
    ] {
        println!(
            "cargo::rustc-env={variable}={}",
            locked_version(&lock, package)
        );
    }
}
