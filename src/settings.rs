use serde::{Deserialize, Serialize};

pub const SETTINGS_VERSION: &str = "0.1";

/// The `Settings` struct holds all the settings which will be saved and loaded
/// from a file.
///
/// These settings give persistence between sessions, such as the amount of
/// time the user should work, as well as the amount of time the user should
/// have a break.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Settings {
    // Once new features are added, the version will increment. Thus, breaking
    // changes can be mitigated, as to not cause a disaster.
    pub version: String,
    pub work_time: u64,
    pub break_time: u64,
    pub notification: Notifications,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Notifications {
    pub enable: bool,
    pub work_msg: String,
    pub break_msg: String,
}

impl Default for Notifications {
    fn default() -> Self {
        Notifications {
            enable: true,
            work_msg: "Good job your work is done. Take a break".to_string(),
            break_msg: "Break is done. Get back to work".to_string(),
        }
    }
}

impl Settings {
    /// Creates a new instance of the `Settings` struct.
    ///
    /// ## Arguments
    /// - work_time: The amount of time the work session lasts.
    /// - break_time: The amount of time the break session lasts.
    ///
    /// ## Returns
    /// A new `Settings` instance where the version of the settings, is the one
    /// which is set in the `SETTINGS_VERSION` const. As well as the break
    /// and work time specified in the arguments.
    pub fn new(work_time: u64, break_time: u64, notification: Notifications) -> Self {
        Self {
            version: SETTINGS_VERSION.to_string(),
            work_time,
            break_time,
            notification,
        }
    }

    /// Using serde, converts a the `Session` instance to a string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Using serde, returns a `Session` instance from a string, if it can be
    /// deserialized. Otherwise, it returns `None`.
    pub fn from_json(string: &str) -> Option<Self> {
        serde_json::from_str(string).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_settings_to_json_and_back() {
        let notification = Notifications::default();
        let settings = Settings::new(25, 5, notification);

        let json_str = settings.to_json();

        let deserialized_settings = Settings::from_json(&json_str).expect("Invalid JSON");

        assert_eq!(settings, deserialized_settings);
    }
}
