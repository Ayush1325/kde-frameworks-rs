//! Helper Functions.

use semver::Version;
use std::path::PathBuf;

/// Helper function. Prefixes `KF{version}` to Library Name.
/// Eg: `I18n` -> `KF5I18n`
pub fn get_lib_name(lib: &str) -> Result<String, semver::Error> {
    Ok(format!("KF{}{}", get_kf_version()?.major, lib))
}

/// Returns KDE Frameworks Root Library Path.
pub fn get_kf_library_path() -> PathBuf {
    env!("KF_LIBRARY_PATH").into()
}

/// Returns KDE Frameworks Root Include Path.
pub fn get_kf_include_path() -> PathBuf {
    env!("KF_INCLUDE_PATH").into()
}

/// Returns KDE Frameworks Version.
pub fn get_kf_version() -> Result<Version, semver::Error> {
    env!("KF_VERSION").parse()
}
