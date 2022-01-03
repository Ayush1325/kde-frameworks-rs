use semver::Version;
use std::path::PathBuf;

pub fn get_lib_name(lib: &str) -> Result<String, semver::Error> {
    Ok(format!("KF{}{}", get_kf_version()?.major, lib))
}

pub fn get_kf_library_path() -> PathBuf {
    env!("KF_LIBRARY_PATH").into()
}

pub fn get_kf_include_path() -> PathBuf {
    env!("KF_INCLUDE_PATH").into()
}

pub fn get_kf_version() -> Result<Version, semver::Error> {
    env!("KF_VERSION").parse()
}
