use semver::Version;
use std::path::PathBuf;
use std::process::Command;

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
    println!("cargo:rerun-if-env-changed=KF_INCLUDE_PATH");
    println!("cargo:rerun-if-env-changed=KF_LIBRARY_PATH");

    match (
        std::env::var("KF_INCLUDE_PATH").ok(),
        std::env::var("KF_LIBRARY_PATH").ok(),
    ) {
        (Some(include_path), Some(library_path)) => (
            get_kf_version(),
            PathBuf::from(include_path),
            PathBuf::from(library_path),
        ),
        (None, None) => {
            const DEFAULT_INCLUDE_PATH: &str = "/usr/include/KF";
            const DEFAULT_LIBRARY_PATH: &str = "/usr/lib";
            let version = get_kf_version();
            let major_version = version.major;
            (
                version,
                PathBuf::from(format!("{}{}", DEFAULT_INCLUDE_PATH, major_version)),
                PathBuf::from(DEFAULT_LIBRARY_PATH),
            )
        }
        (Some(_), None) | (None, Some(_)) => {
            panic!("KF_INCLUDE_PATH and KF_LIBRARY_PATH env variable must be either both empty or both set.")
        }
    }
}

fn get_kf_version() -> Version {
    const ERROR_MSG: &str = "Error in Getting KDE Frameworks Version";
    let stdout = Command::new("kf5-config")
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
