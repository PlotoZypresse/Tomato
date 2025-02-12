use chrono::prelude::*;
use chrono::serde::ts_seconds; // Allows for seralization with Chrono Timestamps
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::json_serializable::JsonSerializable;

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
