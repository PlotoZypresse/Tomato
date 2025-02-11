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
use std::path::{Path, PathBuf};

use crate::json_serializable::JsonSerializable;

/// Using the `home` crate, finds the home folder for the current user.
///
/// ## Returns
/// The home directory. Panics if this cannot be found.
fn get_home_path_with<F>(home_dir_fn: F) -> String
where
    F: Fn() -> Option<PathBuf>,
{
    let path: String = match home_dir_fn() {
        Some(home_path) => home_path.into_os_string().into_string().unwrap(),
        None => panic!("Impossible to get home dir."),
    };

    path
}

/// Checks whether the folder in user home dir exists.
///
/// ## Arguments
/// * folder: A folder in the home dir (~/).
///
/// ## Returns
/// True if the folder exists. False otherwise.
fn folder_exists(folder: String) -> bool {
    let path = format!("{}/{}", get_home_path_with(home_dir), folder);
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
                get_home_path_with(home_dir),
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
    pub fn write(&self, text: String) -> std::io::Result<()> {
        if !folder_exists(self.folder.clone()) {
            let path = format!("{}/{}/", get_home_path_with(home_dir), self.folder);
            fs::create_dir(path)?;
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

    /// Reads from `storage_file`.
    ///
    /// ## Returns
    /// A Result value. Ok(String) containing the contents, if succesful.
    /// Err otherwise.
    pub fn read(&self) -> std::io::Result<String> {
        let mut file = File::open(self.storage_file.clone())?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{create_dir, remove_dir_all};

    use super::*;

    #[test]
    #[should_panic]
    fn test_get_home_path_with_none_should_panic() {
        // Later tests not specific to get_home_path tests with non-None values.
        get_home_path_with(|| None);
    }

    #[test]
    fn test_folder_exists_should_return_true_when_folder_exists() {
        let folder_path = format!("{}/{}", get_home_path_with(home_dir), "test_folder");

        match create_dir(&folder_path) {
            Ok(_) => (),
            Err(e) => panic!("The folder could not be created: {e}"),
        }

        assert!(folder_exists("test_folder".to_string()));

        match remove_dir_all(folder_path) {
            Ok(_) => (),
            Err(e) => panic!("The folder could not be removed: {e}"),
        }
    }

    #[test]
    fn test_folder_exists_should_return_false_when_folder_doesnt_exist() {
        assert!(!folder_exists("test_folder".to_string()));
    }

    #[test]
    fn test_session_new_creates_new_session() {
        let session1: Session = Session::new(
            Some(Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).single().unwrap()),
            25,
            5,
        );
        let session2: Session = Session {
            timestamp: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).single().unwrap(),
            work_time: 25,
            break_time: 5,
        };

        assert_eq!(session1, session2);
    }

    #[test]
    fn test_session_new_create_new_session_with_none() {
        let session1: Session = Session::new(None, 25, 5);
        let session2: Session = Session {
            timestamp: Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).single().unwrap(),
            work_time: 25,
            break_time: 5,
        };

        assert_eq!(session1, session2);
    }

    #[test]
    fn test_session_new_not_equal() {
        let session1: Session = Session::new(None, 25, 5);
        let session2: Session = Session {
            timestamp: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).single().unwrap(),
            work_time: 25,
            break_time: 5,
        };

        assert_ne!(session1, session2);
    }

    #[test]
    fn test_session_clone_is_equal() {
        let session1 = Session::new(None, 5, 5);
        assert_eq!(session1, session1.clone());
    }

    #[test]
    fn test_sessionlist_new_creates_new_session() {
        let session1 = Session::new(None, 25, 5);
        let session2 = Session::new(None, 40, 5);
        let session3 = Session::new(None, 10, 10);
        let sessions_new: SessionList = SessionList::new(Some(vec![
            session1.clone(),
            session2.clone(),
            session3.clone(),
        ]));
        let sessions_manual = SessionList {
            sessions: vec![session1, session2, session3],
        };

        assert_eq!(sessions_new, sessions_manual);
    }

    #[test]
    fn test_sessionlist_new_creates_new_session_with_none() {
        let sessions = SessionList::new(None);
        let sessions_none = SessionList {
            sessions: Vec::new(),
        };
        assert_eq!(sessions, sessions_none);
    }

    #[test]
    fn test_sessionlist_not_equal() {
        // TODO: get some better naming
        let session1 = Session::new(None, 25, 5);
        let sessions1 = SessionList::new(None);
        let sessions2 = SessionList::new(Some(vec![session1]));

        assert_ne!(sessions1, sessions2);
    }

    #[test]
    fn test_sessionlist_append_does_append() {
        let session = Session::new(None, 25, 5);
        let appended_session = Session::new(None, 5, 5);

        let mut sessions_append = SessionList::new(Some(vec![session.clone()]));
        sessions_append.append(appended_session.clone());

        let sessions_not_appended = SessionList::new(Some(vec![session, appended_session]));

        assert_eq!(sessions_append, sessions_not_appended);
    }

    #[test]
    fn test_sessionlist_get_total_work_minutes() {
        let session_list = SessionList::new(Some(vec![
            Session::new(None, 25, 5),
            Session::new(None, 35, 5),
            Session::new(None, 100, 0),
        ]));

        assert_eq!(session_list.total_work_minutes(), 160);
    }

    #[test]
    fn test_storage_new_with_custom_folder() {
        let folder = Some("custom_folder".to_string());
        let path = "file.txt".to_string();
        let storage = Storage::new(folder.clone(), path.clone());

        let home = get_home_path_with(home_dir);
        let expected_storage_file = format!("{}/{}/{}", home, "custom_folder", "file.txt");

        assert_eq!(storage.storage_file, expected_storage_file);
        assert_eq!(storage.folder, "custom_folder".to_string());
    }

    #[test]
    fn test_storage_new_with_default_folder() {
        let folder = None;
        let path = "file.txt".to_string();
        let storage = Storage::new(folder, path.clone());

        let home = get_home_path_with(home_dir);
        let expected_storage_file = format!("{}/{}/{}", home, ".tomato", "file.txt");

        assert_eq!(storage.storage_file, expected_storage_file);
        assert_eq!(storage.folder, ".tomato".to_string());
    }

    #[test]
    fn test_storage_new_with_empty_path() {
        let folder = Some("custom_folder".to_string());
        let path = "".to_string();
        let storage = Storage::new(folder.clone(), path.clone());

        let home = get_home_path_with(home_dir);
        let expected_storage_file = format!("{}/{}/{}", home, "custom_folder", "");

        assert_eq!(storage.storage_file, expected_storage_file);
    }

    #[test]
    fn test_storage_read_and_write() {
        let folder = Some("custom_folder".to_string());
        let path = "file.txt".to_string();
        let storage = Storage::new(folder.clone(), path.clone());

        let lorem = String::from("Lorem ipsum dolor sit amet.");

        match storage.write(lorem.clone()) {
            Ok(_) => (),
            Err(e) => panic!("Error! {e}"),
        }

        let read = storage.read().unwrap();

        assert_eq!(lorem, read);

        let _ = remove_dir_all(format!(
            "{}/{}",
            get_home_path_with(home_dir),
            "custom_folder"
        ));
    }

    #[test]
    fn test_storage_write_folder_doesnt_exist() {
        let folder = Some("non_existant_folder".to_string());
        let path = "file.txt".to_string();
        let storage = Storage::new(folder.clone(), path.clone());

        let lorem = String::from("Lorem ipsum dolor sit amet.");

        match storage.write(lorem.clone()) {
            Ok(_) => (),
            Err(e) => panic!("Error! {e}"),
        }

        assert!(folder_exists("non_existant_folder".to_string()));

        let _ = remove_dir_all(format!(
            "{}/{}",
            get_home_path_with(home_dir),
            "non_existant_folder"
        ));
    }
}
