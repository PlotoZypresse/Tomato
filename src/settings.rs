use serde::{Deserialize, Serialize};
use serde_json;

pub const SETTINGS_VERSION: &str = "0.1";

#[derive(Serialize, Deserialize)]
pub struct Settings {
    // Once new features are added, the version will increment. Thus, breaking
    // changes can be mitigated, as to not cause a disaster.
    pub version: String,
    pub work_time: u64,
    pub break_time: u64,
}

impl Settings {
    pub fn new(work_time: u64, break_time: u64) -> Self {
        let settings = Self {
            version: SETTINGS_VERSION.to_string(),
            work_time,
            break_time,
        };
        settings
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(string: &str) -> Option<Self> {
        serde_json::from_str(string).ok()
    }
}
