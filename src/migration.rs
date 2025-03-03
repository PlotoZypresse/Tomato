//! This file handles the migration of the `Settings` and `Sessions`
//! struct from earlier versions to new versions.

use crate::json_serializable::JsonSerializable;
use crate::settings::{Notifications, Settings};
use crate::storage::Storage;
use regex::Regex;

/// Checks if the version of the `settings.json` file is up to date
/// with the current settings version.
pub fn is_correct_version(file_contents: &str, settings_version: &str) -> bool {
    find_settings_version(file_contents) == settings_version
}
/// Migrates settings from the version of the settings the user is currently
/// using, to the newest version, by modifying the file in-place.
///
///
/// ## Arguments
/// * file_contents: The contents of the `settings.json` file.
pub fn migrate_settings(file_contents: &str) -> Settings {
    let version = find_settings_version(file_contents);
    match version {
        "0.1" => match migrate_0_1(file_contents) {
            Ok(settings) => settings,
            Err(()) => panic!("You have a settings version of 0.1, but it could not be migrated. Please report this incident at GitHub.") 
        },
        _ => panic!("Did not find a valid version! Found version {version}"),
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
    let version = Regex::new(r#""version":"([^"]+)""#).unwrap();

    let Some(caps) = version.captures(file_contents) else {
        return "ERR";
    };

    caps.get(1).unwrap().as_str()
}

fn migrate_0_1(settings: &str) -> Result<Settings, ()> {
    let work_re = Regex::new(r#""work_time":(\d+)"#).unwrap();
    let break_re = Regex::new(r#""break_time":(\d+)"#).unwrap();
    let Some(work_time) = work_re.captures(settings) else {
        return Err(());
    };
    let Some(break_time) = break_re.captures(settings) else {
        return Err(());
    };

    let storage: Storage = Storage::new(None, "settings.json".to_owned());

    let return_settings = Settings {
        version: "0.2".to_owned(),
        work_time: work_time.get(1).unwrap().as_str().parse::<u64>().unwrap(),
        break_time: break_time.get(1).unwrap().as_str().parse::<u64>().unwrap(),
        notification: Notifications::default(),
    };

    storage
        .write(return_settings.to_json())
        .expect("Could not write to settings file during migration.");

    Ok(return_settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_settings_version_should_find_correct_settings() {
        let found_settings = "{\"version\":\"0.1\",\"work_time\":25,\"break_time\":5}";

        assert_eq!(find_settings_version(found_settings), "0.1");
    }

    // TODO: Needs to be implemented once an update rolls out
    #[test]
    fn test_migrate_settings_migrates_correctly() {}

    #[test]
    fn test_is_correct_version_is_correct() {
        let found_settings = "{\"version\":\"0.1\",\"work_time\":25,\"break_time\":5}";
        assert!(is_correct_version(found_settings, "0.1"));
        assert!(!is_correct_version(found_settings, "0.2"));

        let found_settings = "{\"version\":\"0.5\",\"work_time\":25,\"break_time\":5}";
        assert!(is_correct_version(found_settings, "0.5"));
        assert!(!is_correct_version(found_settings, "0.123"));

        let found_settings = "{\"version\":\"2025.1.105\", osidhf aposudhfapoidf hasdfio nadaå vasoåiv j\"work_time\":25,\"break_time\":5}";
        assert!(is_correct_version(found_settings, "2025.1.105"));
        assert!(!is_correct_version(found_settings, "0.123"));
    }
}
