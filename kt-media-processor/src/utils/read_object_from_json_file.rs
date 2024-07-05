use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Reads an object of type `T` from a JSON file at the given path.
///
/// # Type Parameters
///
/// * `T`: The type of the object to deserialize. Must implement `DeserializeOwned`.
///
/// # Arguments
///
/// * `file_path` - A reference to the path of the file to read from.
///
/// # Returns
///
/// Returns `Ok(T)` if the file was read and deserialized successfully, or an `Err` if an error occurred.
pub fn read_object_from_json_file<T: DeserializeOwned, P: AsRef<Path>>(
    file_path: P,
) -> Result<T, Box<dyn std::error::Error>> {
    // Open the file in read-only mode.
    let file = File::open(file_path)?;
    // Create a buffered reader for the file.
    let reader = BufReader::new(file);
    // Deserialize the JSON into an object of type `T`.
    let obj = serde_json::from_reader(reader)?;
    // Return the deserialized object.
    Ok(obj)
}
