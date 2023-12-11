use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedImages {
    pub name: String,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub device_id: String,
    pub timestamp: String,
    pub sensors_data: SensorsData,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SensorsData {
    pub extrinsic_matrix: Vec<f64>,
    pub intrinsic_matrix: Vec<f64>,
}
