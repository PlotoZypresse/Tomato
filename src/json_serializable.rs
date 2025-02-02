use serde::{Deserialize, Serialize};

pub trait JsonSerializable: Serialize + for<'de> Deserialize<'de> {
    /// Converts the struct to a string JSON object.
    ///
    /// ## Returns
    /// The struct as a string in a JSON format.
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Converts from JSON String into a struct.
    ///
    /// ## Arguments
    /// * input: The string input containing serialized json of the struct.
    ///
    /// ## Returns
    /// The json string converted to a struct.
    fn from_json(string: &str) -> Option<Self>
    where
        Self: Sized,
    {
        serde_json::from_str(string).ok()
    }
}
