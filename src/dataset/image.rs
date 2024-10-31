use crate::{utils::load_json, Error, ImageAnnotation, Result};
use indexmap::IndexSet;
use itertools::Itertools;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// The classical Supervisely dataset.
#[derive(Debug, Clone)]
pub struct ImageDataset {
    pub dataset_dir: PathBuf,
    pub image_names: IndexSet<String>,
}

/// The reference to a media data.
#[derive(Debug, Clone)]
pub struct ImageData<'a> {
    image_name: &'a str,
    dataset: &'a ImageDataset,
}

impl ImageDataset {
    /// Open a classical Supervisely dataset in a directory.
    pub fn open<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();
        let data_dir = dir.join("img");

        let entries =
            fs::read_dir(&data_dir).map_err(|error| Error::read_dir_error(&data_dir, error))?;

        let mut image_names: IndexSet<_> = entries
            .map(|entry| -> Result<_> {
                let entry = entry.map_err(|error| Error::read_dir_error(&data_dir, error))?;
                let file_name = entry.file_name();
                let file_name = file_name
                    .to_str()
                    .ok_or_else(|| Error::expect_utf8_file_name(entry.path()))?
                    .to_string();
                Ok(file_name)
            })
            .try_collect()?;
        image_names.sort_unstable();

        Ok(Self {
            image_names,
            dataset_dir: dir.to_owned(),
        })
    }

    /// Query the image data by its name.
    pub fn get_image(&self, image_name: &str) -> Option<ImageData<'_>> {
        let image_name = self.image_names.get(image_name)?;
        Some(ImageData {
            image_name,
            dataset: self,
        })
    }
}

impl<'a> ImageData<'a> {
    /// Get the annotation data.
    pub fn ann(&self) -> Result<ImageAnnotation> {
        let Self {
            image_name,
            dataset,
        } = *self;

        let path = dataset
            .dataset_dir
            .join("ann")
            .join(format!("{image_name}.json"));
        let ann: ImageAnnotation = load_json(path)?;
        Ok(ann)
    }
}
