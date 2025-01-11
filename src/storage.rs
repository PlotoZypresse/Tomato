//! # Storage
//! This file contains the necessary functions to create persistent storage for
//! Tomato.
//!
//! Tomato uses a JSON file to store the sessions a user has had.

use chrono::prelude::*;
use chrono::serde::ts_seconds; // Allows for seralization with Chrono Timestamps
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Session {
    #[serde(with = "ts_seconds")] // Converts to a format Serde can (de)serailize
    pub timestamp: DateTime<Utc>, // Has to be UTC, can be converted later
    pub work_time: u32,
    pub break_time: u32,
}

struct Storage {
    storage_file: String,
}

impl Session {
    pub fn new(timestamp: Option<DateTime<Utc>>, work_time: u32, break_time: u32) -> Session {
        match timestamp {
            None => {
                return Session {
                    timestamp: Utc::now(),
                    work_time,
                    break_time,
                }
            }
            Some(timestamp) => {
                return Session {
                    timestamp,
                    work_time,
                    break_time,
                }
            }
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(string: &str) -> Option<Self> {
        serde_json::from_str(string).ok()
    }
}

impl Storage {
    pub fn new(path: String) -> Storage {
        Storage { storage_file: path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serialize_session_to_json_and_back() {
        let date: DateTime<Utc> = Utc
            .with_ymd_and_hms(2012, 1, 19, 0, 0, 0)
            .single()
            .expect("Invalid date or time");

        let session = Session::new(Some(date), 25, 5);
        let json_str = session.to_json();

        let deserialized_session = Session::from_json(&json_str).expect("Invalid JSON");

        assert_eq!(session, deserialized_session);
    }

    #[test]
    fn serialize_session_to_json_and_back_with_none() {
        let session = Session::new(None, 25, 5);
        let json_str = session.to_json();

        let deserialized_session = Session::from_json(&json_str).expect("Invalid JSON");

        assert_eq!(session, deserialized_session);
    }
}
