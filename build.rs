#[cfg(target_os = "windows")]
extern crate pkg_config;

#[cfg(target_os = "windows")]
fn main() {
    use std::env;
    let libs_to_find = ["glib-2.0", "gobject-2.0", "pango-1.0", "gio-2.0", "atk-1.0",
                        "gdk-pixbuf-2.0", "cairo", "gdk-3", "gtk-3"];
    if let Ok(lib_dir) = env::var("GTK_LIB_DIR") {
        for lib_ in libs_to_find.iter() {
            println!("cargo:rustc-link-lib=dylib={}", lib_);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir);
    } else {
        for lib in libs_to_find.iter() {
            pkg_config::probe_library(lib).unwrap();
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
