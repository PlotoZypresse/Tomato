//! # Storage
//! This file contains the necessary functions to create persistent storage for
//! Tomato.
//!
//! Tomato uses a JSON file to store the sessions a user has had.

use std::fs;
use std::fs::File;
use std::path::Path;

use home::home_dir;

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

/// Holds the variable containing the path to the storage file.
pub struct Storage {
    storage_file: String,
    folder: String,
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
    pub fn write(&self, text: String) -> std::io::Result<()> {
        if !folder_exists(self.folder.clone()) {
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
    use super::*;

    /// Removes `storage_file`. This should only be called in tests.
    ///
    /// ## Returns
    /// A Result value. Ok(()) if successfully deleted file, err otherwise.
    fn remove_file(storage: Storage) -> std::io::Result<()> {
        fs::remove_file(storage.storage_file)?;
        Ok(())
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

        match storage.write(write_value.clone()) {
            Ok(_) => (),
            Err(v) => panic!("{}. Check path {}", v, expected_path),
        }

        match storage.read() {
            Ok(v) => assert_eq!(v, write_value),
            Err(v) => panic!("{}", v),
        }

        match remove_file(storage) {
            Ok(_) => (),
            Err(v) => panic!("{}", v),
        }
    }
}
