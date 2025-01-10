//! # Storage
//! This file contains the necessary functions to create persistent storage for
//! Tomato.
//!
//! Tomato uses a JSON file to store the sessions a user has had.

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Session {
    pub timestamp: String, // Maybe use chrono?
    pub work_time: u32,
    pub break_time: u32,
}

struct Storage {
    storage_file: String,
}

impl Session {
    pub fn new() -> Session {}

    pub fn to_json() -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Storage {
    pub fn new(path: String) -> Storage {
        Storage { storage_file: path }
    }
}
