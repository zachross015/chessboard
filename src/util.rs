use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use ron::error::SpannedResult;
use serde::de::DeserializeOwned;

/// Loads an object from a ron file. 
pub fn load_ron<T: DeserializeOwned>(filepath: &str) -> SpannedResult<T> {
    // Open the file in read-only mode with buffer.
    let path = Path::new(filepath);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = ron::de::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}
