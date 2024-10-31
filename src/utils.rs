use crate::{Error, Result};
use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};

pub fn load_json<T, P>(path: P) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let file = File::open(path).map_err(|error| Error::open_file_error(path, error))?;
    let reader = BufReader::new(file);
    let value = serde_json::from_reader(reader)
        .map_err(|error| Error::parse_json_file_error(path, error))?;
    Ok(value)
}
