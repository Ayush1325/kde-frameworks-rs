pub mod helpers;

use std::path::PathBuf;

/// Function to link a KDE Library.
/// Eg: For linking `KI18n`:
/// ```no_run
/// use kde_frameworks::link_lib;
///
/// link_lib("I18n");
/// ```
pub fn link_lib(lib: &str) -> Result<(), semver::Error> {
    let lib_name = helpers::get_lib_name(lib);
    println!("cargo:rustc-link-lib={}", lib_name?);
    Ok(())
}

/// Function to Return the Include Path for the KDE Library.
/// This is the path where the header files for the Library are located.
/// ```no_run
/// use kde_frameworks::get_lib_include_path;
///
/// get_lib_include_path("I18n");
/// ```
pub fn get_lib_include_path(lib: &str) -> PathBuf {
    helpers::get_kf_include_path().join(format!("K{}", lib))
}
