use crate::{utils::load_json, Error, Result, VideoAnnotation};
use indexmap::IndexSet;
use itertools::Itertools;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// The classical Supervisely dataset.
#[derive(Debug, Clone)]
pub struct VideoDataset {
    pub dataset_dir: PathBuf,
    pub video_names: IndexSet<String>,
}

/// The reference to a media data.
#[derive(Debug, Clone)]
pub struct VideoData<'a> {
    video_name: &'a str,
    dataset: &'a VideoDataset,
}

impl VideoDataset {
    /// Open a classical Supervisely dataset in a directory.
    pub fn open<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();
        let video_dir = dir.join("video");

        let entries =
            fs::read_dir(&video_dir).map_err(|error| Error::read_dir_error(&video_dir, error))?;

        let mut video_names: IndexSet<_> = entries
            .map(|entry| -> Result<_> {
                let entry = entry.map_err(|error| Error::read_dir_error(&video_dir, error))?;

                let file_name = entry.file_name();
                let file_name = file_name
                    .to_str()
                    .ok_or_else(|| Error::expect_utf8_file_name(entry.path()))?
                    .to_string();
                Ok(file_name)
            })
            .try_collect()?;
        video_names.sort_unstable();

        Ok(Self {
            video_names,
            dataset_dir: dir.to_owned(),
        })
    }

    /// Query the video data by its name.
    pub fn get_video(&self, video_name: &str) -> Option<VideoData<'_>> {
        let video_name = self.video_names.get(video_name)?;
        Some(VideoData {
            video_name,
            dataset: self,
        })
    }
}

impl<'a> VideoData<'a> {
    /// Get the annotation data.
    pub fn ann(&self) -> Result<VideoAnnotation> {
        let Self {
            video_name: media_name,
            dataset,
        } = *self;

        let path = dataset
            .dataset_dir
            .join("ann")
            .join(format!("{media_name}.json"));
        let ann: VideoAnnotation = load_json(path)?;

        Ok(ann)
    }
}
