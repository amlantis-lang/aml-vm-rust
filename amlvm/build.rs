use std::process::Command;
use std::string::String;

fn main() {
    println!("cargo:rustc-link-search=native={}", icu_libs_path());
    println!("cargo:rustc-link-lib=dylib=icui18n");
    println!("cargo:rustc-link-lib=dylib=icuuc");
    println!("cargo:rustc-link-lib=dylib=icudata");
    println!("cargo:rustc-link-lib=dylib=icuio");
}

fn icu_libs_path() -> String {
    let icu_libs_output = Command::new("pkg-config")
        .args(&["--libs-only-L", "icu-uc", "icu-io"])
        .output()
        .unwrap_or_else(|e| {
            panic!("Failed to execute: {}", e)
        });
    String::from_utf8_lossy(&icu_libs_output.stdout)
        [2 .. icu_libs_output.stdout.len()]
        .to_string()
}
