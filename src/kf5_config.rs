use std::{path::PathBuf, process::Command};

use semver::Version;

const KF_CONFIG_COMMAND: &str = "kf5-config";

pub fn get_version() -> Result<Version, &'static str> {
    const ERROR_MSG: &str = "Error in Getting KDE Frameworks Version";

    let stdout = Command::new(KF_CONFIG_COMMAND)
        .arg("--kde-version")
        .output()
        .map_err(|_| ERROR_MSG)?
        .stdout;
    String::from_utf8(stdout)
        .expect(ERROR_MSG)
        .trim()
        .parse()
        .map_err(|_| ERROR_MSG)
}

pub fn get_include_path() -> Result<PathBuf, &'static str> {
    const ERROR_MSG: &str = "Error in Getting KDE Frameworks Include PATH";

    let stdout = Command::new(KF_CONFIG_COMMAND)
        .args(["--path", "include"])
        .output()
        .map_err(|_| ERROR_MSG)?
        .stdout;
    Ok(PathBuf::from(
        String::from_utf8(stdout).map_err(|_| ERROR_MSG)?.trim(),
    ))
}

pub fn get_library_path() -> Result<PathBuf, &'static str> {
    const ERROR_MSG: &str = "Error in Getting KDE Frameworks Include PATH";

    let stdout = Command::new(KF_CONFIG_COMMAND)
        .args(["--path", "lib"])
        .output()
        .expect(ERROR_MSG)
        .stdout;
    Ok(PathBuf::from(
        String::from_utf8(stdout).map_err(|_| ERROR_MSG)?.trim(),
    ))
}
