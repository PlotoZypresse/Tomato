//! # Storage
//! This file contains the necessary functions to create persistent storage for
//! Tomato.
//!
//! Tomato uses a JSON file to store the sessions a user has had.

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use home::home_dir;

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
