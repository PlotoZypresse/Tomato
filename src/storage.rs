//! # Storage
//! This file contains the necessary functions to create persistent storage for
//! Tomato.
//!
//! Tomato uses a JSON file to store the sessions a user has had.

use chrono::prelude::*;
use chrono::serde::ts_seconds; // Allows for seralization with Chrono Timestamps
use chrono::{DateTime, Utc};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::json_serializable::JsonSerializable;

/// Using the `home` crate, finds the home folder for the current user.
///
/// ## Returns
/// The home directory. Panics if this cannot be found.
fn get_home_path() -> String {
    let path: String = match home_dir() {
        Some(home_path) => home_path.into_os_string().into_string().unwrap(),
        None => panic!("Impossible to get home dir."),
    };

    path
}

/// Checks whether the folder exists.
///
/// ## Returns
/// True if the folder exists. False otherwise.
fn folder_exists(folder: String) -> bool {
    let path = format!("{}/{}", get_home_path(), folder);
    Path::new(&path).exists()
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Session {
    #[serde(with = "ts_seconds")] // Converts to a format Serde can (de)serailize
    pub timestamp: DateTime<Utc>, // Has to be UTC, can be converted later
    pub work_time: u32,
    pub break_time: u32,
}

/// Holds a list of Session instances.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SessionList {
    sessions: Vec<Session>,
}

/// Holds the variable containing the path to the storage file.
pub struct Storage {
    storage_file: String,
    folder: String,
}

impl JsonSerializable for Session {}

impl Session {
    /// Creates a new instance of the Session struct.
    ///
    /// ## Arguments
    /// * timestamp: An optional argument of a `DateTime<Utc>` timestamp. If left to none, it becomes Jan 1, 1970.
    /// * work_time: The amount of time the user has worked this session.
    /// * break_time: The amount of time the user has had a break this session.
    ///
    /// ## Returns
    /// An instance of Session from specified arguments.
    pub fn new(timestamp: Option<DateTime<Utc>>, work_time: u32, break_time: u32) -> Session {
        match timestamp {
            None => Session {
                timestamp: Utc
                    .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                    .single()
                    .expect("Failed to parse fixed date."),
                work_time,
                break_time,
            },
            Some(timestamp) => Session {
                timestamp,
                work_time,
                break_time,
            },
        }
    }
}

impl JsonSerializable for SessionList {}

impl SessionList {
    /// Initializes a new empty SessionList struct.
    ///
    /// ## Arguments
    /// An Option data type containing a vector of Session instances.
    ///
    /// ## Returns
    /// A SessionList instance containing:
    /// * If `None`: An empty vector of Session instances initialized to `sessions`.
    /// * If `Some(v)`: Given vector of Session instances initialized to `sessions`.
    pub fn new(sessions: Option<Vec<Session>>) -> Self {
        match sessions {
            None => SessionList {
                sessions: Vec::new(),
            },
            Some(v) => SessionList { sessions: v },
        }
    }

    /// Pushes a value to the `sessions` vector.
    ///
    /// ## Arguments
    /// * session: A Session instance to be added to the `sessions` field.
    pub fn append(&mut self, session: Session) {
        self.sessions.push(session);
    }

    /// Gets the total amount of minutes worked from all Session instances
    /// in `sessions`.
    ///
    /// ## Returns
    /// The total amount of minutes worked from all Session instances in
    /// `sessions` field.
    pub fn total_work_minutes(&self) -> u64 {
        let mut total: u64 = 0;
        for session in &self.sessions {
            total += session.work_time as u64;
        }

        total
    }
}

impl Storage {
    /// Creates a new Storage struct.
    ///
    /// ## Arguments
    /// * path: The name of the file, without any prefix. E.g. "file.txt"
    ///
    /// ## Returns
    /// A storage struct containing the variable `storage_file` with the value of argument appended to the default path prefix ("~/.tomato/")
    pub fn new(folder: Option<String>, path: String) -> Storage {
        Storage {
            storage_file: format!(
                "{}/{}/{}",
                get_home_path(),
                folder.clone().unwrap_or(".tomato".to_string()),
                path
            ),
            folder: folder.unwrap_or(".tomato".to_string()),
        }
    }

    /// Writes to `storage_file`.
    ///
    /// ## Returns
    /// A Result value. Ok(()) if no problems occured, otherwise Err.
    pub fn write(&self, folder: Option<String>, text: String) -> std::io::Result<()> {
        if !folder_exists(folder.unwrap_or(".tomato".to_string())) {
            let path = format!("{}/{}/", get_home_path(), self.folder);
            match fs::create_dir(path) {
                Ok(_) => (),
                Err(v) => panic!("{}", v),
            }
        }

        // File::create creates a file if it does not exist.
        // If it does exist, it truncates the file.
        let mut file = File::create(self.storage_file.clone())?;

        let byte_amount = file.write(text.as_bytes())?;

        if byte_amount != text.len() {
            panic!("Something went wrong while writing to file. Written amount not equal to amount which should have been written.");
        }

        file.flush().unwrap();

        Ok(())
    }

    /// Removes `storage_file`. This should only be called in tests.
    ///
    /// ## Returns
    /// A Result value. Ok(()) if successfully deleted file, err otherwise.
    #[allow(dead_code)]
    fn remove_file(&self) -> std::io::Result<()> {
        fs::remove_file(self.storage_file.clone())?;
        Ok(())
    }

    /// Reads from `storage_file`.
    ///
    /// ## Returns
    /// A Result value. Ok(String) containing the contents, if succesful.
    /// Err otherwise.
    pub fn read(&self) -> std::io::Result<String> {
        let mut file = File::open(self.storage_file.clone())?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // TODO: Do we need Clone here?
        Ok(contents.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_session_to_json_and_back() {
        let date: DateTime<Utc> = Utc
            .with_ymd_and_hms(2012, 1, 19, 0, 0, 0)
            .single()
            .expect("Failed to parse fixed date.");

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

    #[test]
    fn create_and_write_to_file() {
        // Remove the directory first, to test if it creates the directory
        // correctly.
        let path = format!("{}/.tomato_test/", get_home_path());

        match fs::remove_dir_all(path) {
            Ok(_) => (),
            Err(v) =>
                println!("Folder did not exist at the start. If this is not the first time running this test, something is wrong. Error: {}", v),
        }

        let name = "test.txt".to_string();

        let expected_path = format!("{}/.tomato_test/{}", get_home_path(), name).to_string();
        let storage = Storage::new(Some(".tomato_test".to_string()), name.clone());

        // Make sure that it has the correct prefix
        assert_eq!(storage.storage_file, expected_path);

        let write_value = String::from("Æether Åland Øndre");

        match storage.write(Some(".tomato_test".to_string()), write_value.clone()) {
            Ok(_) => (),
            Err(v) => panic!("{}. Check path {}", v, expected_path),
        }

        match storage.read() {
            Ok(v) => assert_eq!(v, write_value),
            Err(v) => panic!("{}", v),
        }

        match storage.remove_file() {
            Ok(_) => (),
            Err(v) => panic!("{}", v),
        }
    }

    #[test]
    fn serialize_deserialize_sessionlist() {
        let sessions = vec![
            Session::new(None, 25, 5),
            Session::new(
                Some(
                    Utc.with_ymd_and_hms(2020, 12, 1, 0, 0, 0)
                        .single()
                        .expect("Failed to parse fixed date."),
                ),
                10, // work minutes
                5,
            ), // break minutes
            Session::new(
                Some(
                    Utc.with_ymd_and_hms(2025, 1, 11, 20, 43, 50)
                        .single()
                        .expect("Failed to parse fixed date."),
                ),
                1, // work minutes
                1, // break minutes
            ),
        ];

        let list = SessionList::new(Some(sessions));

        let json = list.to_json();

        let deserialize = SessionList::from_json(&json).unwrap();

        assert_eq!(deserialize, list);
    }
}
