use crate::{utils::load_json, Error, PointCloudAnnotation, Result};
use indexmap::IndexSet;
use itertools::Itertools;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// The classical Supervisely dataset.
#[derive(Debug, Clone)]
pub struct PointCloudDataset {
    pub dataset_dir: PathBuf,
    pub point_cloud_names: IndexSet<String>,
}

/// The reference to point cloud data.
#[derive(Debug, Clone)]
pub struct PointCloudData<'a> {
    point_cloud_name: &'a str,
    dataset: &'a PointCloudDataset,
}

impl PointCloudDataset {
    /// Open a classical Supervisely dataset in a directory.
    pub fn open<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();
        let point_cloud_dir = dir.join("pointcloud");

        let entries = fs::read_dir(&point_cloud_dir)
            .map_err(|error| Error::read_dir_error(&point_cloud_dir, error))?;

        let mut point_cloud_names: IndexSet<_> = entries
            .map(|entry| -> Result<_> {
                let entry =
                    entry.map_err(|error| Error::read_dir_error(&point_cloud_dir, error))?;

                let file_name = entry.file_name();
                let file_name = file_name
                    .to_str()
                    .ok_or_else(|| Error::expect_utf8_file_name(entry.path()))?
                    .to_string();
                Ok(file_name)
            })
            .try_collect()?;
        point_cloud_names.sort_unstable();

        Ok(Self {
            point_cloud_names,
            dataset_dir: dir.to_owned(),
        })
    }

    /// Query the media data by its name.
    pub fn get_point_cloud(&self, media_name: &str) -> Option<PointCloudData<'_>> {
        let media_name = self.point_cloud_names.get(media_name)?;
        Some(PointCloudData {
            point_cloud_name: media_name,
            dataset: self,
        })
    }
}

impl<'a> PointCloudData<'a> {
    /// Get the annotation data.
    pub fn ann(&self) -> Result<PointCloudAnnotation> {
        let Self {
            point_cloud_name: media_name,
            dataset,
        } = *self;

        let path = dataset
            .dataset_dir
            .join("ann")
            .join(format!("{media_name}.json"));
        let ann: PointCloudAnnotation = load_json(path)?;
        Ok(ann)
    }
}
