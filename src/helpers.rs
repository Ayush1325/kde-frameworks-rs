//! Helper Functions.

/// Helper function. Prefixes `KF{version}` to Library Name.
/// Eg: `I18n` -> `KF5I18n`
pub(crate) fn get_lib_name(lib: &str, major_version: u64) -> Result<String, semver::Error> {
    Ok(format!("KF{}{}", major_version, lib))
}
