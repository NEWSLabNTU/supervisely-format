use super::DatasetKind;
use crate::{
    utils::load_json, Error, ImageAnnotation, MediaAnnotation, MediaKind, PointCloudAnnotation,
    Result, VideoAnnotation,
};
use indexmap::IndexSet;
use itertools::Itertools;
use std::{
    fs,
    path::{Path, PathBuf},
};
use strum::IntoEnumIterator;

/// The classical Supervisely dataset.
#[derive(Debug, Clone)]
pub struct ClassicalDataset {
    pub dataset_dir: PathBuf,
    pub media_kind: MediaKind,
    pub media_names: IndexSet<String>,
}

/// The reference to a media data.
#[derive(Debug, Clone)]
pub struct ClassicalMedia<'a> {
    media_name: &'a str,
    dataset: &'a ClassicalDataset,
}

impl From<ClassicalDataset> for DatasetKind {
    fn from(v: ClassicalDataset) -> Self {
        Self::Classical(v)
    }
}

impl ClassicalDataset {
    /// Open a classical Supervisely dataset in a directory.
    pub fn open<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();

        let media_kind = {
            let kinds: Vec<MediaKind> = MediaKind::iter()
                .filter(|kind| dir.join(kind.dir_name()).exists())
                .collect();

            match kinds.as_slice() {
                [] => return Err(crate::Error::expect_single_media_folder(dir)),
                &[kind] => kind,
                [..] => {
                    return Err(crate::Error::expect_single_media_folder(dir));
                }
            }
        };

        let media_dir = dir.join(media_kind.dir_name());

        let entries =
            fs::read_dir(&media_dir).map_err(|error| Error::read_dir_error(&media_dir, error))?;

        let mut media_names: IndexSet<_> = entries
            .map(|entry| -> Result<_> {
                let entry = entry.map_err(|error| Error::read_dir_error(&media_dir, error))?;

                let file_name = entry.file_name();
                let file_name = file_name
                    .to_str()
                    .ok_or_else(|| Error::expect_utf8_file_name(entry.path()))?
                    .to_string();
                Ok(file_name)
            })
            .try_collect()?;
        media_names.sort_unstable();

        Ok(Self {
            media_names,
            dataset_dir: dir.to_owned(),
            media_kind,
        })
    }

    /// Query the media data by its name.
    pub fn get_media(&self, media_name: &str) -> Option<ClassicalMedia<'_>> {
        let media_name = self.media_names.get(media_name)?;
        Some(ClassicalMedia {
            media_name,
            dataset: self,
        })
    }
}

impl<'a> ClassicalMedia<'a> {
    /// Get the media folder path.
    pub fn media_path(&self) -> PathBuf {
        let Self {
            media_name,
            dataset,
        } = *self;

        dataset
            .dataset_dir
            .join(dataset.media_kind.dir_name())
            .join(media_name)
    }

    /// Get the annotation data.
    pub fn ann(&self) -> Result<MediaAnnotation> {
        let Self {
            media_name,
            dataset,
        } = *self;

        let path = dataset
            .dataset_dir
            .join("ann")
            .join(format!("{media_name}.json"));

        let ann = match dataset.media_kind {
            MediaKind::Image => load_json::<ImageAnnotation, _>(path)?.into(),
            MediaKind::Video => load_json::<VideoAnnotation, _>(path)?.into(),
            MediaKind::PointCloud => load_json::<PointCloudAnnotation, _>(path)?.into(),
        };

        Ok(ann)
    }
}
