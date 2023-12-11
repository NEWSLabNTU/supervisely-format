use crate::{
    objects::{Object, Points},
    project_meta::Shape,
    tags::Tag,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageAnnotation {
    pub name: String,
    pub description: Option<String>,
    pub size: Size,
    pub tags: Option<Vec<Tag>>,
    pub objects: Option<Vec<Object>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoAnnotation {
    pub size: Size,
    pub description: String,
    pub tags: Vec<Tag>,
    pub key: String,
    pub objects: Vec<VideoObject>,
    pub frames: Vec<Frame>,
    pub frames_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointCloudAnnotation {
    pub description: String,
    pub key: Option<String>,
    pub tags: Vec<Tag>,
    pub objects: Vec<PointCloudObject>,
    pub figures: Vec<PointCloudFigure>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointCloudObject {
    pub key: String,
    #[serde(rename = "classTitle")]
    pub class_title: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointCloudFigure {
    pub key: String,
    #[serde(rename = "objectKey")]
    pub object_key: String,
    #[serde(rename = "geometryType")]
    pub geometry_type: Shape,
    pub geometry: PointCloudGeometry,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointCloudGeometry {
    pub position: Vector3D,
    pub rotation: Vector3D,
    pub dimensions: Vector3D,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frame {
    pub index: usize,
    pub figures: Vec<Figure>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoObject {
    pub key: String,
    #[serde(rename = "classTitle")]
    pub class_title: Option<usize>,
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "labelerLogin")]
    pub labeler_login: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Figure {
    pub key: String,
    #[serde(rename = "objectKey")]
    pub object_key: String,
    pub geometry_type: FigureGeometryType,
    pub geometry: FigureGeometry,
    #[serde(rename = "classTitle")]
    pub class_title: Option<usize>,
    #[serde(rename = "labelerLogin")]
    pub labeler_login: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FigureGeometryType {
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "rectangle")]
    Rectangle,
    #[serde(rename = "polygon")]
    Polygon,
    #[serde(rename = "polyline")]
    Polyline,
    #[serde(rename = "bitmap")]
    Bitmap,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FigureGeometry {
    pub points: Points,
}
