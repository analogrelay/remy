#![feature(path_ext)]

use std::{path,env,fs};
use std::fs::PathExt;

fn main() {
    let project_root = path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_triple = env::var("TARGET").unwrap().to_string();

    // TODO(anurse): Find a cleaner way to handle this
    // Starts in: target/debug/build/rudy-.../out
    let mut out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    out_dir.pop(); // target/debug/build/rudy-...
    out_dir.pop(); // target/debug/build
    out_dir.pop(); // target/debug

    let bin_out = out_dir.join("SDL2.dll");

    // Find externals dir
    let host_externals = project_root.parent().unwrap().join("ext").join(target_triple);

    // Check for SDL2
    let sdl2_root = host_externals.join("sdl2");
    if !sdl2_root.exists() {
        panic!("could not find SDL2 external root in: {:?}", sdl2_root);
    }

    // Find the files for copying
    if !bin_out.exists() {
        let bin_path = sdl2_root.join("bin").join("SDL2.dll");
        if !bin_path.exists() {
            panic!("could not find SDL2.dll in: {:?}", bin_path);
        }

        // Copy to the output directory
        fs::copy(bin_path, bin_out).unwrap();
    }

    // Add the lib directory to the search path
    let lib_path = sdl2_root.join("lib");
    if !lib_path.exists() {
        panic!("could not find lib path in: {:?}", lib_path);
    }
    println!("cargo:rustc-link-search=native={}", lib_path.to_str().unwrap());
}
