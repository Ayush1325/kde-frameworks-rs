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
pub mod kf5_config;

use semver::Version;
use std::path::{Path, PathBuf};

/// Function to link a KDE Library.
/// Eg: For linking `KI18n`:
/// ```no_run
/// use kde_frameworks::link_lib;
///
/// link_lib("I18n", 5);
/// ```
pub fn link_lib(lib: &str, major_version: u64) -> Result<(), semver::Error> {
    let lib_name = helpers::get_lib_name(lib, major_version);
    println!("cargo:rustc-link-lib={}", lib_name?);
    Ok(())
}

/// Function to Return the Include Path for the KDE Library.
/// This is the path where the header files for the Library are located.
/// ```no_run
/// use kde_frameworks::get_lib_include_path;
/// use std::path::PathBuf;
///
/// get_lib_include_path("I18n", 5, &PathBuf::from("/usr/include"));
/// ```
pub fn get_lib_include_path(
    lib: &str,
    major_version: u64,
    include_path: &Path,
) -> Result<PathBuf, semver::Error> {
    Ok(include_path
        .join(format!("KF{}", major_version))
        .join(format!("K{}", lib)))
}

/// Function to have Conditional Compilation based on KDE Frameworks version.
pub fn set_version_cfg(version: Version, least_minor_version: u64) {
    let mut minor = least_minor_version;
    while version >= Version::new(5, minor, 0) {
        println!("cargo:rustc-cfg=kf_{}_{}", 5, minor);
        minor += 1;
    }
}

/// Function to check if Version, Include Path and Library Path Environment variables have been set
pub fn check_env_variables(library_name: &str) -> Option<(Version, PathBuf, PathBuf)> {
    println!("cargo:rerun-if-env-changed={}_VERSION", library_name);
    println!("cargo:rerun-if-env-changed={}_INCLUDE_PATH", library_name);
    println!("cargo:rerun-if-env-changed={}_LIBRARY_PATH", library_name);

    match (
        std::env::var(format!("{}_VERSION", library_name)).ok(),
        std::env::var(format!("{}_INCLUDE_PATH", library_name)).ok(),
        std::env::var(format!("{}_LIBRARY_PATH", library_name)).ok(),
    ) {
        (Some(version), Some(include_path), Some(library_path)) => {
            let v = version.parse().ok()?;
            Some((v, PathBuf::from(include_path), PathBuf::from(library_path)))
        }
        _ => None,
    }
}

/// Function to probe KDE using kf5-config tool.
pub fn check_kf5_config() -> Result<(Version, PathBuf, PathBuf), &'static str> {
    Ok((
        kf5_config::get_version()?,
        kf5_config::get_include_path()?,
        kf5_config::get_library_path()?,
    ))
}
