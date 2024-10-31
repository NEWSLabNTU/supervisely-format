use crate::{geometry::Geometry, objects::Object, tags::Tag, Shape};
use serde::{Deserialize, Serialize};

/// The image annotation data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageAnnotation {
    pub name: Option<String>,
    pub description: Option<String>,
    pub size: Size,
    pub tags: Option<Vec<Tag>>,
    pub objects: Vec<Object>,
}

/// The video annotation data.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoAnnotation {
    pub size: Size,
    pub description: String,
    pub tags: Vec<Tag>,
    pub key: String,
    pub objects: Vec<VideoObject>,
    pub frames: Vec<Frame>,
    pub frames_count: u64,
}

/// The point cloud annotation data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointCloudAnnotation {
    pub description: String,
    pub key: Option<String>,
    pub tags: Vec<Tag>,
    pub objects: Vec<PointCloudObject>,
    pub figures: Vec<PointCloudFigure>,
}

/// The point cloud annotation data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointCloudEpisodeAnnotation {
    pub description: String,
    pub key: Option<String>,
    pub tags: Vec<Tag>,
    pub objects: Vec<PointCloudObject>,
    pub frames: Vec<Frame>,
}

/// Represent a point cloud object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointCloudObject {
    pub key: String,
    pub class_title: String,
    pub tags: Vec<Tag>,
}

/// The point cloud figure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointCloudFigure {
    pub key: String,
    pub object_key: String,
    pub geometry_type: Shape,
    pub geometry: PointCloudGeometry,
}

/// The shape parameters of a point cloud object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointCloudGeometry {
    pub position: Vector3D,
    pub rotation: Vector3D,
    pub dimensions: Vector3D,
}

/// A 3-dimensional vector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// The size of an object.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size {
    pub width: u64,
    pub height: u64,
}

/// A collection of figures associated with a timestamp.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frame {
    pub index: u64,
    pub figures: Vec<Figure>,
}

/// Represent a video object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoObject {
    pub key: String,
    pub class_title: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub labeler_login: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Figure {
    pub key: String,
    pub object_key: String,
    #[serde(flatten, with = "serde_figure_geometry")]
    pub geometry: Geometry,
    pub class_title: Option<String>,
    pub labeler_login: Option<String>,
}

mod serde_figure_geometry {
    use crate::{
        BitmapGeometry, Cuboid3DGeometry, Geometry, PointGeometry, PolygonGeometry,
        PolylineGeometry, RectangleGeometry,
    };
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(geometry: &Geometry, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized: SerializedGeometryRef = match geometry {
            Geometry::Point(point) => point.into(),
            Geometry::Rectangle(rect) => rect.into(),
            Geometry::Polygon(polygon) => polygon.into(),
            Geometry::Polyline(polyline) => polyline.into(),
            Geometry::Bitmap(bitmap) => bitmap.into(),
            Geometry::Cuboid3D(cuboid3d) => cuboid3d.into(),
        };
        serialized.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Geometry, D::Error>
    where
        D: Deserializer<'de>,
    {
        let geometry = match SerializedGeometry::deserialize(deserializer)? {
            SerializedGeometry::Point(point) => point.into(),
            SerializedGeometry::Rectangle(rect) => rect.into(),
            SerializedGeometry::Polygon(polygon) => polygon.into(),
            SerializedGeometry::Polyline(polyline) => polyline.into(),
            SerializedGeometry::Bitmap(bitmap) => bitmap.into(),
            SerializedGeometry::Cuboid3D(cuboid) => cuboid.into(),
        };
        Ok(geometry)
    }

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
    #[serde(tag = "geometryType", content = "geometry", rename_all = "camelCase")]
    pub enum SerializedGeometry {
        Point(PointGeometry),
        Rectangle(RectangleGeometry),
        Polygon(PolygonGeometry),
        Polyline(PolylineGeometry),
        Bitmap(BitmapGeometry),
        #[serde(rename = "cuboid_3d")]
        Cuboid3D(Cuboid3DGeometry),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    #[serde(tag = "geometryType", content = "geometry", rename_all = "camelCase")]
    pub enum SerializedGeometryRef<'a> {
        Point(&'a PointGeometry),
        Rectangle(&'a RectangleGeometry),
        Polygon(&'a PolygonGeometry),
        Polyline(&'a PolylineGeometry),
        Bitmap(&'a BitmapGeometry),
        #[serde(rename = "cuboid_3d")]
        Cuboid3D(&'a Cuboid3DGeometry),
    }

    impl<'a> From<&'a Cuboid3DGeometry> for SerializedGeometryRef<'a> {
        fn from(v: &'a Cuboid3DGeometry) -> Self {
            Self::Cuboid3D(v)
        }
    }

    impl<'a> From<&'a BitmapGeometry> for SerializedGeometryRef<'a> {
        fn from(v: &'a BitmapGeometry) -> Self {
            Self::Bitmap(v)
        }
    }

    impl<'a> From<&'a PolylineGeometry> for SerializedGeometryRef<'a> {
        fn from(v: &'a PolylineGeometry) -> Self {
            Self::Polyline(v)
        }
    }

    impl<'a> From<&'a PolygonGeometry> for SerializedGeometryRef<'a> {
        fn from(v: &'a PolygonGeometry) -> Self {
            Self::Polygon(v)
        }
    }

    impl<'a> From<&'a RectangleGeometry> for SerializedGeometryRef<'a> {
        fn from(v: &'a RectangleGeometry) -> Self {
            Self::Rectangle(v)
        }
    }

    impl<'a> From<&'a PointGeometry> for SerializedGeometryRef<'a> {
        fn from(v: &'a PointGeometry) -> Self {
            Self::Point(v)
        }
    }
}
