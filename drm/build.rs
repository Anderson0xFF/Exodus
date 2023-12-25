use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=drm");

    let bindings = bindgen::Builder::default()
        .clang_arg("-I/usr/include/libdrm")
        .header("/usr/include/xf86drm.h")
        .header("/usr/include/xf86drmMode.h")
        .header("/usr/include/drm/drm_mode.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .constified_enum_module(".*")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("drm.rs"))
        .expect("Couldn't write bindings!");
}
