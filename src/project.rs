use anyhow::Result;
use crate::{utils::load_json, Dataset, ProjectMeta};
use std::{collections::HashMap, path::{Path, PathBuf}};

#[derive(Debug, Clone)]
pub struct Project {
    pub project_dir: PathBuf,
    pub datasets: HashMap<String, Dataset>,
}

impl Project {
    pub fn open<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();
        let project_meta: ProjectMeta = load_json(dir.join("meta.json"))?;

        todo!();
        // Ok(Self {})
    }
}
