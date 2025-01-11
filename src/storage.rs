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

/// Using the `home` crate, finds the home folder for the current user.
///
/// ## Returns
/// The home directory. Panics if this cannot be found.
fn get_home_path() -> String {
    let mut path: String;

    match home_dir() {
        Some(home_path) => {
            path = home_path.into_os_string().into_string().unwrap();
        }
        None => panic!("Impossible to get home dir."),
    }

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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Session {
    #[serde(with = "ts_seconds")] // Converts to a format Serde can (de)serailize
    pub timestamp: DateTime<Utc>, // Has to be UTC, can be converted later
    pub work_time: u32,
    pub break_time: u32,
}

/// Holds the variable containing the path to the storage file.
struct Storage {
    storage_file: String,
    folder: String,
}

impl Session {
    /// Creates a new instance of the Session struct.
    ///
    /// ## Arguments
    /// * timestamp: An optional argument of a DateTime<Utc> timestamp. If left
    /// to none, it becomes Jan 1, 1970.
    /// * work_time: The amount of time the user has worked this session.
    /// * break_time: The amount of time the user has had a break this session.
    ///
    /// ## Returns
    /// An instance of Session from specified arguments.
    pub fn new(timestamp: Option<DateTime<Utc>>, work_time: u32, break_time: u32) -> Session {
        match timestamp {
            None => {
                return Session {
                    timestamp: Utc
                        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                        .single()
                        .expect("Failed to parse fixed date."),
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

    /// Converts the Session instance to a JSON string.
    ///
    /// ## Returns
    /// A string containing a JSON formatted string containing the informations
    /// contained within the instance.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Takes a JSON string and converts it to an instance of Session
    ///
    /// ## Returns
    /// * Some(Session) if successfull
    /// * None if unsuccessfull
    pub fn from_json(string: &str) -> Option<Self> {
        serde_json::from_str(string).ok()
    }
}

impl Storage {
    /// Creates a new Storage struct.
    ///
    /// ## Arguments
    /// * path
    /// The name of the file, without any prefix. E.g. "file.txt"
    ///
    /// ## Returns
    /// A storage struct containing the variable `storage_file` with the value
    /// of argument appended to the default path prefix ("~/.tomato/")
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

        file.write(text.as_bytes())?;

        Ok(())
    }

    // TODO: If this should only be called in tests, should it maybe only be
    // defined there?
    /// Removes `storage_file`. This should only be called in tests.
    ///
    /// ## Returns
    /// A Result value. Ok(()) if successfully deleted file, err otherwise.
    fn remove_file(&self) -> std::io::Result<()> {
        let mut file = fs::remove_file(self.storage_file.clone())?;
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
}
