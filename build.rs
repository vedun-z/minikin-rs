use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;
use path_slash::PathBufExt;
use std::str::FromStr;

fn cpp_stdlib_name() -> Option<String> {
    if let Ok(stdlib) = env::var("CXXSTDLIB") {
        if stdlib.is_empty() {
            None
        } else {
            Some(stdlib)
        }
    } else {
        let target = env::var("TARGET").unwrap();
        if target.contains("msvc") {
            None
        } else if target.contains("apple") {
            Some("c++".to_string())
        } else if target.contains("freebsd") {
            Some("c++".to_string())
        } else if target.contains("openbsd") {
            Some("c++".to_string())
        } else {
            Some("stdc++".to_string())
        }
    }
}

fn main() {
    // https://docs.microsoft.com/en-us/cpp/c-runtime-library/crt-library-features?view=vs-2019

    let target = std::env::var("TARGET").unwrap();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    // rustc always links against non-debug Windows runtime https://github.com/rust-lang/rust/issues/39016
    let build_debug = &target_env != "msvc" && bool::from_str(env::var("DEBUG").unwrap().as_str()).unwrap();

    // https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes
    let features = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or(String::new());

    let vcpkg_os_name =
        if target.contains("linux") { "x64-linux" }
        else if target.contains("windows") {"x64-windows" }
        else { "x64-osx" };

    let output = if target.contains("windows") {
        Command::new("./build-third-party.bat").output().unwrap()
    } else {
        Command::new("./build-third-party.sh").output().unwrap()
    };

    println!("CARGO_CFG_TARGET_FEATURE={}", features);
    let static_crt = features.contains("crt-static");

    {
        let mut minikin_cfg = cmake::Config::new("minikin");
        if build_debug {
            minikin_cfg.profile("Debug");
        } else {
            minikin_cfg.profile("Release");
        }
        if !static_crt {
            minikin_cfg.define("MSVC_DYNAMIC_RT", "ON");
        } else {
            minikin_cfg.define("MSVC_DYNAMIC_RT", "OFF");
        }
        let minikin_out = minikin_cfg.build();
        println!("cargo:rustc-link-search=native={}", minikin_out.display());
        println!("cargo:rustc-link-lib=static=minikin");

        println!("cargo:rustc-link-search=native={}/vcpkg/installed/{}/lib", src_dir.display(), vcpkg_os_name);

        println!("cargo:rustc-link-lib=static=harfbuzz");
        println!("cargo:rustc-link-lib=static=harfbuzz-icu");
        println!("cargo:rustc-link-lib=static=icuuc");
        println!("cargo:rustc-link-lib=static=icudata");
        println!("cargo:rustc-link-lib=static=freetype");
        println!("cargo:rustc-link-lib=static=bz2");
        println!("cargo:rustc-link-lib=static=png");
        println!("cargo:rustc-link-lib=static=z");
        println!("cargo:rustc-link-lib=static=png");
        println!("cargo:rustc-link-lib=static=brotlidec-static");
        println!("cargo:rustc-link-lib=static=brotlicommon-static");
    }

    if let Some(name) = cpp_stdlib_name() {
        println!("cargo:rustc-link-lib={}", name);
    }
}