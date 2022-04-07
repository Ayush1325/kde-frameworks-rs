//! # Introduction
//! This crate serves as the base for all the KDE Frameworks Crate written by me. It sets important
//! environment variables and provides helpful methods required in most KDE Frameworks.
//!
//! Currently, mostly supposed to be used as a build dependency.
//!
//! # Install kf5-config
//! ## Ubuntu
//! ```sh
//! sudo apt install libkf5kdelibs4support5-bin
//! ```
//! ## Fedroa
//! ```sh
//! sudo dnf install kf5-kdelibs4support
//! ```
//!
//! # Environment Variables Read By this Crate
//! - It is optional to provide these variables. If these variables are not present, then `kf5-config`
//!   must be present in the path.
//! 1. `KF_LIBRARY_PATH` : Path for KDE Frameworks Library Location.
//! 2. `KF_INCLUDE_PATH` : Path for KDE Frameworks Header Files Location.
//! 3. `KF_VERSION` : KDE Frameworks Version. Format `<major>.<minor>.<patch>`.
//!
//! # Environment Variables Set By this Crate
//! 1. `KF_LIBRARY_PATH` : Path for KDE Frameworks Library Location.
//! 2. `KF_INCLUDE_PATH` : Path for KDE Frameworks Header Files Location.
//! 3. `KF_VERSION` : KDE Frameworks Version Detected.
//! 4. `KF_FOUND` : Flag to specify if KDE Frameworks is found or not.

pub mod helpers;

use semver::Version;
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
pub fn get_lib_include_path(lib: &str) -> Result<PathBuf, semver::Error> {
    let major_version = helpers::get_kf_version()?.major;
    Ok(helpers::get_kf_include_path()
        .join(format!("KF{}", major_version))
        .join(format!("K{}", lib)))
}

/// Function to have Conditional Compilation based on KDE Frameworks version.
pub fn set_version_cfg(version: Version) {
    let mut minor = 90;
    while version >= Version::new(5, minor, 0) {
        println!("cargo:rustc-cfg=kf_{}_{}", 5, minor);
        minor += 1;
    }
}
