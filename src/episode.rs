use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FramePointcloudMap {
    #[serde(flatten)]
    pub map: HashMap<u64, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeAnnotation {
    pub description: String,
    pub tags: Vec<Value>,
    pub objects: Vec<Value>,
    pub frames_count: u64,
    pub frames: Vec<Value>,
}
