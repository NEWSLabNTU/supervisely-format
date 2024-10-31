use crate::geometry::Geometry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Object {
    pub id: usize,
    #[serde(rename = "classId")]
    pub class_id: Option<usize>,
    #[serde(rename = "labelerLogin")]
    pub labeler_login: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(flatten, with = "serde_object_geometry")]
    pub geometry: Geometry,
}

mod serde_object_geometry {
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
            SerializedGeometry::Cuboid3D(cuboid3d) => cuboid3d.into(),
        };
        Ok(geometry)
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(tag = "geometryType", rename_all = "camelCase")]
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
