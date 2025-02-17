//! This file handles the migration of the `Settings` and `Sessions`
//! struct from earlier versions to new versions.

use crate::settings::SETTINGS_VERSION;
use regex::Regex;

/// Checks if the version of the `settings.json` file is up to date
/// with the current settings version.
pub fn is_correct_version(file_contents: &str) -> bool {
    find_settings_version(file_contents) == SETTINGS_VERSION
}
/// Migrates settings from the version of the settings the user is currently
/// using, to the newest version.
///
/// ## Arguments
/// * file_contents: The contents of the `settings.json` file.
pub fn migrate_settings(file_contents: &str) {
    match find_settings_version(file_contents) {
        "0.1" => todo!(),
        _ => panic!("Did not find a valid version!"),
    }
}

/// Finds the version of the settings in the `file_contents` argument.
///
/// ```
/// let found_settings = "{\"version\":\"0.1\",\"work_time\":25,\"break_time\":5}";
/// assert_eq!(crate::migration::find_settings_version(found_settings), "0.1");
/// ```
///
/// ## Arguments
/// * file_contents: The contents of the `settings.json` file.
fn find_settings_version(file_contents: &str) -> &str {
    let version = Regex::new(r#""version":"(\d+\.\d+)""#).unwrap();

    let Some(caps) = version.captures(file_contents) else {
        return "ERR";
    };

    caps.get(1).unwrap().as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_settings_version_should_find_correct_settings() {
        let found_settings = "{\"version\":\"0.1\",\"work_time\":25,\"break_time\":5}";

        assert_eq!(find_settings_version(found_settings), "0.1");
    }
}
