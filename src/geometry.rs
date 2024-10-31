use crate::{tags::Tag, Error, Result};
use base64::prelude::*;
use flate2::read::ZlibDecoder;
use noisy_float::types::R64;
use serde::{Deserialize, Serialize};
use std::io::{prelude::*, Cursor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Shape {
    Rectangle,
    Line,
    Polygon,
    Point,
    Bitmap,
    #[serde(rename = "cuboid_3d")]
    Cuboid3D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GeometryType {
    Point,
    Rectangle,
    Polygon,
    Polyline,
    Bitmap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Geometry {
    Point(PointGeometry),
    Rectangle(RectangleGeometry),
    Polygon(PolygonGeometry),
    Polyline(PolylineGeometry),
    Bitmap(BitmapGeometry),
    Cuboid3D(Cuboid3DGeometry),
}

impl From<Cuboid3DGeometry> for Geometry {
    fn from(v: Cuboid3DGeometry) -> Self {
        Self::Cuboid3D(v)
    }
}

impl From<BitmapGeometry> for Geometry {
    fn from(v: BitmapGeometry) -> Self {
        Self::Bitmap(v)
    }
}

impl From<PolylineGeometry> for Geometry {
    fn from(v: PolylineGeometry) -> Self {
        Self::Polyline(v)
    }
}

impl From<PolygonGeometry> for Geometry {
    fn from(v: PolygonGeometry) -> Self {
        Self::Polygon(v)
    }
}

impl From<RectangleGeometry> for Geometry {
    fn from(v: RectangleGeometry) -> Self {
        Self::Rectangle(v)
    }
}

impl From<PointGeometry> for Geometry {
    fn from(v: PointGeometry) -> Self {
        Self::Point(v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SerializedGeometry {
    pub geometry_type: GeometryType,
    pub tags: Option<Vec<Tag>>,
    pub points: Option<Points>,
    pub shape: Option<Shape>,
    pub bitmap: Option<Bitmap>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointGeometry {
    pub tags: Option<Vec<Tag>>,
    pub points: Points,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RectangleGeometry {
    pub tags: Option<Vec<Tag>>,
    pub points: Points,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolygonGeometry {
    pub tags: Option<Vec<Tag>>,
    pub points: Points,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolylineGeometry {
    pub tags: Option<Vec<Tag>>,
    pub points: Points,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitmapGeometry {
    pub tags: Option<Vec<Tag>>,
    pub bitmap: Bitmap,
    pub shape: Option<Shape>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cuboid3DGeometry {
    pub tags: Option<Vec<Tag>>,
    pub position: Xyz,
    pub rotation: Xyz,
    pub dimensions: Xyz,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Points {
    pub exterior: Vec<(R64, R64)>,
    pub interior: Vec<(R64, R64)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bitmap {
    #[serde(rename = "data")]
    pub data_encoded: String,
    pub origin: [u64; 2],
}

impl Bitmap {
    pub fn decode_data(&self) -> Result<Vec<u8>> {
        let mut decompressed = vec![];
        let zlib_bytes = BASE64_STANDARD
            .decode(&self.data_encoded)
            .map_err(|_| Error::DecodeDataError)?;
        ZlibDecoder::new(Cursor::new(zlib_bytes))
            .read_to_end(&mut decompressed)
            .map_err(|_| Error::DecodeDataError)?;
        Ok(decompressed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Xyz {
    pub x: R64,
    pub y: R64,
    pub z: R64,
}
