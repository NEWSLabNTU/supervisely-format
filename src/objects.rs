use crate::tags::Tag;
use noisy_float::types::R64;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Object {
    pub id: Option<usize>,
    #[serde(rename = "classId")]
    pub class_id: Option<usize>,
    #[serde(rename = "labelerLogin")]
    pub labeler_login: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub geometry: ObjectGeometry,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "geometryType")]
pub enum ObjectGeometry {
    #[serde(rename = "point")]
    Point(PointGeometry),
    #[serde(rename = "rectangle")]
    Rectangle(RectangleGeometry),
    #[serde(rename = "polygon")]
    Polygon(PolygonGeometry),
    #[serde(rename = "polyline")]
    Polyline(PolylineGeometry),
    #[serde(rename = "bitmap")]
    Bitmap(BitmapGeometry),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointGeometry {
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "classTitle")]
    pub class_title: String,
    pub points: Points,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RectangleGeometry {
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "classTitle")]
    pub class_title: String,
    pub points: Points,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolygonGeometry {
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "classTitle")]
    pub class_title: String,
    pub points: Points,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolylineGeometry {
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "classTitle")]
    pub class_title: String,
    pub points: Points,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitmapGeometry {
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "classTitle")]
    pub class_title: String,
    pub bitmap: Bitmap,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Points {
    pub exterior: Vec<(R64, R64)>,
    pub interior: Vec<(R64, R64)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bitmap {
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    pub origin: (R64, R64),
}
