use indexmap::IndexMap;

use crate::{utils::load_json, Frame, PointCloudEpisodeAnnotation, Result};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

/// The point cloud episode Supervisely dataset.
#[derive(Debug, Clone)]
pub struct PointCloudEpisodeDataset {
    pub dataset_dir: PathBuf,
    pub frame_point_map: IndexMap<u64, String>,
    pub annotation: PointCloudEpisodeAnnotation,
}

impl PointCloudEpisodeDataset {
    /// Open a point cloud episode Supervisely dataset in a directory.
    pub fn open<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();

        let frame_point_map_file = dir.join("frame_pointcloud_map.json");
        let frame_point_map: IndexMap<u64, String> = load_json(&frame_point_map_file)?;

        let annotation_file = dir.join("annotation.json");
        let annotation: PointCloudEpisodeAnnotation = load_json(annotation_file)?;

        Ok(Self {
            dataset_dir: dir.to_path_buf(),
            frame_point_map,
            annotation,
        })
    }

    pub fn frame_id_iter(
        &self,
    ) -> impl ExactSizeIterator<Item = u64> + Debug + Clone + Sync + Send + '_ {
        self.frame_point_map.keys().copied()
    }

    pub fn get_frame(&self, id: u64) -> Option<FrameData<'_>> {
        let file_name = self.frame_point_map.get(&id)?;
        let annotation = self.annotation.frames.get(id as usize).unwrap();
        assert_eq!(annotation.index, id);

        Some(FrameData {
            id,
            file_name,
            annotation,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FrameData<'a> {
    pub id: u64,
    pub file_name: &'a str,
    pub annotation: &'a Frame,
}
