use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};

pub fn load_json<T, P>(path: P) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let reader = BufReader::new(File::open(path)?);
    let value = serde_json::from_reader(reader)?;
    Ok(value)
}
