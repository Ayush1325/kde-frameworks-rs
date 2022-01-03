pub mod helpers;

use std::path::PathBuf;

pub fn link_lib(lib: &str) -> Result<(), semver::Error> {
    let lib_name = helpers::get_lib_name(lib);
    println!("cargo:rustc-link-lib={}", lib_name?);
    Ok(())
}

pub fn get_lib_include_path(lib: &str) -> PathBuf {
    helpers::get_kf_include_path().join(format!("K{}", lib))
}
