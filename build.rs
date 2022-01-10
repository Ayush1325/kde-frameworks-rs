use semver::Version;
use std::path::PathBuf;
use std::process::Command;

const KF_CONFIG_COMMAND: &str = "kf5-config";

fn main() {
    let (kf_version, kf_include_path, kf_library_path) = probe_kde();

    println!(
        "cargo:rustc-link-search={}",
        kf_library_path.to_str().unwrap()
    );

    println!("cargo:rustc-env=KF_VERSION={}", kf_version);
    println!(
        "cargo:rustc-env=KF_LIBRARY_PATH={}",
        kf_library_path.to_str().unwrap()
    );
    println!(
        "cargo:rustc-env=KF_INCLUDE_PATH={}",
        kf_include_path.to_str().unwrap()
    );
    println!("cargo:rustc-env=KF_FOUND={}", 1);
}

fn probe_kde() -> (Version, PathBuf, PathBuf) {
    println!("cargo:rerun-if-env-changed=KF_VERSION");
    println!("cargo:rerun-if-env-changed=KF_INCLUDE_PATH");
    println!("cargo:rerun-if-env-changed=KF_LIBRARY_PATH");

    match (
        std::env::var("KF_VERSION").ok(),
        std::env::var("KF_INCLUDE_PATH").ok(),
        std::env::var("KF_LIBRARY_PATH").ok(),
    ) {
        (Some(version), Some(include_path), Some(library_path)) => (
            version
                .parse()
                .expect("Error Parsing Version from KF_VERSION"),
            PathBuf::from(include_path),
            PathBuf::from(library_path),
        ),
        (None, None, None) => (
            get_kf_version(),
            get_kf_include_path(),
            get_kf_library_path(),
        ),
        _ => {
            panic!("KF_INCLUDE_PATH, KF_LIBRARY_PATH and KF_VERSION env variables must either all be set of all empty")
        }
    }
}

fn get_kf_version() -> Version {
    const ERROR_MSG: &str = "Error in Getting KDE Frameworks Version";
    let stdout = Command::new(KF_CONFIG_COMMAND)
        .arg("--kde-version")
        .output()
        .expect(ERROR_MSG)
        .stdout;
    String::from_utf8(stdout)
        .expect(ERROR_MSG)
        .trim()
        .parse()
        .expect(ERROR_MSG)
}

fn get_kf_include_path() -> PathBuf {
    const ERROR_MSG: &str = "Error in Getting KDE Frameworks Include PATH";
    let stdout = Command::new(KF_CONFIG_COMMAND)
        .args(["--path", "include"])
        .output()
        .expect(ERROR_MSG)
        .stdout;
    PathBuf::from(String::from_utf8(stdout).expect(ERROR_MSG).trim())
}

fn get_kf_library_path() -> PathBuf {
    const ERROR_MSG: &str = "Error in Getting KDE Frameworks Include PATH";
    let stdout = Command::new(KF_CONFIG_COMMAND)
        .args(["--path", "lib"])
        .output()
        .expect(ERROR_MSG)
        .stdout;
    PathBuf::from(String::from_utf8(stdout).expect(ERROR_MSG).trim())
}
