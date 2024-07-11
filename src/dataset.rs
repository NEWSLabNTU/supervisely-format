use crate::{utils::load_json, ImageAnnotation, PointCloudAnnotation, VideoAnnotation};
use anyhow::{anyhow, bail, Context, Result};
use indexmap::IndexSet;
use itertools::Itertools;
use std::{
    fs,
    path::{Path, PathBuf},
};
use strum::{EnumIter, IntoEnumIterator};
use tracing::warn;

#[derive(Debug, Clone)]
pub struct Dataset {
    pub name: String,
    pub kind: DatasetKind,
}

#[derive(Debug, Clone)]
pub enum DatasetKind {
    Classical(ClassicalDataset),
    PointCloudEpisode(PointCloudEpisodeDataset),
}

#[derive(Debug, Clone)]
pub struct ClassicalDataset {
    pub dataset_dir: PathBuf,
    pub media_kind: MediaKind,
    pub media_names: IndexSet<String>,
}

#[derive(Debug, Clone)]
pub struct PointCloudEpisodeDataset {
    pub dataset_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ClassicalMedia<'a> {
    media_name: &'a str,
    dataset: &'a ClassicalDataset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum MediaKind {
    Image,
    Video,
    PointCloud,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MediaAnnotation {
    Image(ImageAnnotation),
    Video(VideoAnnotation),
    PointCloud(PointCloudAnnotation),
}

impl MediaAnnotation {
    pub fn try_into_image(self) -> Result<ImageAnnotation, Self> {
        if let Self::Image(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_video(self) -> Result<VideoAnnotation, Self> {
        if let Self::Video(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_point_cloud(self) -> Result<PointCloudAnnotation, Self> {
        if let Self::PointCloud(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn as_image(&self) -> Option<&ImageAnnotation> {
        if let Self::Image(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_video(&self) -> Option<&VideoAnnotation> {
        if let Self::Video(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_point_cloud(&self) -> Option<&PointCloudAnnotation> {
        if let Self::PointCloud(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<PointCloudEpisodeDataset> for DatasetKind {
    fn from(v: PointCloudEpisodeDataset) -> Self {
        Self::PointCloudEpisode(v)
    }
}

impl From<ClassicalDataset> for DatasetKind {
    fn from(v: ClassicalDataset) -> Self {
        Self::Classical(v)
    }
}

impl From<PointCloudAnnotation> for MediaAnnotation {
    fn from(v: PointCloudAnnotation) -> Self {
        Self::PointCloud(v)
    }
}

impl From<VideoAnnotation> for MediaAnnotation {
    fn from(v: VideoAnnotation) -> Self {
        Self::Video(v)
    }
}

impl From<ImageAnnotation> for MediaAnnotation {
    fn from(v: ImageAnnotation) -> Self {
        Self::Image(v)
    }
}

impl MediaKind {
    pub fn dir_name(&self) -> &str {
        match self {
            MediaKind::Image => "img",
            MediaKind::Video => "video",
            MediaKind::PointCloud => "pointcloud",
        }
    }
}

impl Dataset {
    pub fn open<P>(dir: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();
        let name = get_dir_name(dir)
            .unwrap_or_else(|| {
                warn!(
                    "unable to determine the directory name of dataset {}",
                    dir.display()
                );
                ""
            })
            .to_string();
        let kind = if dir.join("frame_pointcloud_map.json").exists() {
            PointCloudEpisodeDataset::open(dir)?.into()
        } else {
            ClassicalDataset::open(dir)?.into()
        };

        Ok(Self { name, kind })
    }
}

impl ClassicalDataset {
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
                &[] => {
                    bail!(
                        "no media folder found in dataset directory {}",
                        dir.display()
                    );
                }
                &[kind] => kind,
                &[..] => {
                    let media_dirs = kinds.into_iter().map(|kind| kind.dir_name()).join(", ");
                    bail!("multiple media folders found: {media_dirs}");
                }
            }
        };

        let media_dir = dir.join(media_kind.dir_name());

        let entries = fs::read_dir(&media_dir).with_context(|| {
            format!(
                "unable to list entries in directory {}",
                media_dir.display()
            )
        })?;

        let mut media_names: IndexSet<_> = entries
            .map(|entry| -> Result<_> {
                let entry = entry.with_context(|| {
                    format!(
                        "unable to list entries in directory {}",
                        media_dir.display()
                    )
                })?;

                let file_name = entry.file_name();
                let file_name = file_name
                    .to_str()
                    .ok_or_else(|| anyhow!("{:?} is not a valid media name", file_name))?
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

    pub fn get_media(&self, media_name: &str) -> Option<ClassicalMedia<'_>> {
        let media_name = self.media_names.get(media_name)?;
        Some(ClassicalMedia {
            media_name,
            dataset: self,
        })
    }
}

impl PointCloudEpisodeDataset {
    pub fn open<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();
        todo!();
    }
}

impl<'a> ClassicalMedia<'a> {
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

fn get_dir_name(dir: &Path) -> Option<&str> {
    dir.file_name()?.to_str()
}
