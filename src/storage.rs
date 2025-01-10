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

#[derive(Serialize, Deserialize, Debug)]
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
}

impl Storage {
    pub fn new(path: String) -> Storage {
        Storage { storage_file: path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_session_to_json() {
        let date: DateTime<Utc> = Utc
            .with_ymd_and_hms(2012, 1, 19, 0, 0, 0)
            .single()
            .expect("Invalid date or time");

        let json = Session::new(Some(date), 25, 5).to_json();

        // January 19, 2012 00:00:00 is 1326931200 in Unix time
        assert_eq!(
            json,
            "{\"timestamp\":1326931200,\"work_time\":25,\"break_time\":5}"
        );
    }
}
