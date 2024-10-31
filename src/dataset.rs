mod image;
mod point_cloud;
mod point_cloud_episode;
mod video;

pub use image::ImageDataset;
pub use point_cloud::PointCloudDataset;
pub use point_cloud_episode::PointCloudEpisodeDataset;
pub use video::VideoDataset;

use crate::Result;
use std::path::Path;
use tracing::warn;

/// Represent a dataset contained in a Supervisely project.
#[derive(Debug, Clone)]
pub struct Dataset {
    /// The name of the dataset.
    pub name: String,

    /// The dataset instance classified by its type.
    pub kind: DatasetKind,
}

/// The Supervisely dataset classified by its type.
#[derive(Debug, Clone)]
pub enum DatasetKind {
    Image(ImageDataset),
    Video(VideoDataset),
    PointCloud(PointCloudDataset),
    PointCloudEpisode(PointCloudEpisodeDataset),
}

impl From<PointCloudEpisodeDataset> for DatasetKind {
    fn from(v: PointCloudEpisodeDataset) -> Self {
        Self::PointCloudEpisode(v)
    }
}

impl From<PointCloudDataset> for DatasetKind {
    fn from(v: PointCloudDataset) -> Self {
        Self::PointCloud(v)
    }
}

impl From<VideoDataset> for DatasetKind {
    fn from(v: VideoDataset) -> Self {
        Self::Video(v)
    }
}

impl From<ImageDataset> for DatasetKind {
    fn from(v: ImageDataset) -> Self {
        Self::Image(v)
    }
}

impl Dataset {
    /// Open a Supervisely dataset in a directory.
    pub fn open<P>(dir: P) -> Result<Self>
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
        } else if dir.join("img").exists() {
            ImageDataset::open(dir)?.into()
        } else if dir.join("video").exists() {
            VideoDataset::open(dir)?.into()
        } else if dir.join("pointcloud").exists() {
            PointCloudDataset::open(dir)?.into()
        } else {
            todo!();
        };
        Ok(Self { name, kind })
    }
}

fn get_dir_name(dir: &Path) -> Option<&str> {
    dir.file_name()?.to_str()
}
