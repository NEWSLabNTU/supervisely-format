use crate::{utils::load_json, Dataset, Error, ProjectMeta, Result};
use itertools::Itertools;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Represent a Supervisely project.
#[derive(Debug, Clone)]
pub struct Project {
    pub project_dir: PathBuf,
    pub meta: ProjectMeta,
    pub datasets: HashMap<String, Dataset>,
}

impl Project {
    pub fn open<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();
        let meta: ProjectMeta = load_json(dir.join("meta.json"))?;

        // Scan the dataset folders
        let datasets: HashMap<_, _> = dir
            .read_dir()
            .map_err(|error| Error::read_dir_error(dir, error))?
            .map(|entry| -> Result<_> {
                let entry = entry.map_err(|error| Error::read_dir_error(dir, error))?;
                let path = entry.path();

                let path = if path.is_file() {
                    return Ok(None);
                } else if path.is_dir() {
                    path
                } else {
                    let resolved_path = path
                        .canonicalize()
                        .map_err(|error| Error::resolve_path_error(&path, error))?;

                    if resolved_path.is_dir() {
                        return Ok(None);
                    }
                    path
                };

                let dataset_name = path
                    .file_name()
                    .expect("unable to get the directory name")
                    .to_str()
                    .ok_or_else(|| Error::expect_utf8_file_name(&path))?
                    .to_string();
                let dataset = Dataset::open(&path)?;

                Ok(Some((dataset_name, dataset)))
            })
            .flatten_ok()
            .try_collect()?;

        Ok(Self {
            project_dir: dir.to_path_buf(),
            meta,
            datasets,
        })
    }
}
