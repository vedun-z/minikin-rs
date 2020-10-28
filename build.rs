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

    let is_linux = target.contains("linux");
    let is_windows = target.contains("windows");
    let vcpkg_os_name =
        if is_linux { "x64-linux" }
        else if is_windows {"x64-windows" }
        else { "x64-osx" };

    let mut build_third_party_script = src_dir.clone();
    build_third_party_script.push(
        if is_windows { "build-third-party.bat" } else {"build-third-party.sh"}
    );
    Command::new(build_third_party_script).status().unwrap();

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

        let libs = if is_windows {
            ["brotlicommon",
            "brotlidec",
            "brotlienc",
            "bz2",
            "freetype",
            "harfbuzz",
            "harfbuzz-icu",
            "icudt",
            "icuin",
            "icuio",
            "icutu",
            "icuuc",
            "libpng16",
            "zlib"].iter()
        } else {
            ["harfbuzz",
             "harfbuzz-icu",
             "icuuc",
             "icudata",
             "freetype",
             "png",
             "bz2",
             "z",
             "brotlidec-static",
             "brotlicommon-static"].iter()
        };

        for lib in libs {
            println!("cargo:rustc-link-lib=static={}", lib);
        }
    }

    if let Some(name) = cpp_stdlib_name() {
        println!("cargo:rustc-link-lib={}", name);
    }
}