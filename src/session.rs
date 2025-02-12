use chrono::prelude::*;
use chrono::serde::ts_seconds; // Allows for seralization with Chrono Timestamps
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::json_serializable::JsonSerializable;
use crate::storage::Storage;

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

    /// Finds the sessions from `session.json` and deserializes into the
    /// `SessionList` struct.
    ///
    /// ## Returns
    /// * A SessionList struct containing all previous sessions stored in
    ///   `sessions.json`.
    pub fn load_sessions(folder: String, file_name: String) -> SessionList {
        let storage = Storage::new(Some(folder), file_name.clone());
        let contents = storage.read().unwrap_or_else(|_| "ERR".to_string());

        if contents.is_empty() || contents == "ERR" {
            SessionList::new(None)
        } else {
            SessionList::from_json(&contents).expect("Could not parse the contents of file.")
        }
    }
}

#[cfg(test)]
mod tests {
    use home::home_dir;
    use std::fs::remove_dir_all;

    use crate::storage;

    use super::*;

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
    fn test_load_sessions() {
        // Write some sessions to a file
        let storage = Storage::new(
            Some(".tomato_test".to_string()),
            "sessions.json".to_string(),
        );
        let sessions = SessionList::new(Some(vec![
            Session::new(None, 25, 5),
            Session::new(None, 10, 5),
            Session::new(
                Some(Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).single().unwrap()),
                1,
                1,
            ),
        ]));

        let _ = storage.write(sessions.to_json());

        assert_eq!(
            SessionList::load_sessions(".tomato_test".to_string(), "sessions.json".to_string()),
            sessions
        );

        let _ = remove_dir_all(format!(
            "{}/{}",
            storage::get_home_path_with(home_dir),
            ".tomato_test"
        ));
    }

    #[test]
    fn test_load_sessions_no_file() {
        assert_eq!(
            SessionList::load_sessions(".tomato_test".to_string(), "sessions.json".to_string()),
            SessionList::new(None)
        );
    }
}
