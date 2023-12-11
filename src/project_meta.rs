use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyIdMap {
    pub tags: HashMap<String, usize>,
    pub objects: HashMap<String, usize>,
    pub figures: HashMap<String, usize>,
    pub videos: HashMap<String, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub classes: Vec<ClassMeta>,
    pub tags: Vec<TagMeta>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassMeta {
    pub title: String,
    pub shape: Shape,
    #[serde(with = "serde_color")]
    pub color: Option<palette::Srgb<u8>>,
    pub geometry_config: GeometryConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagMeta {
    pub name: String,
    #[serde(with = "serde_color")]
    pub color: Option<palette::Srgb<u8>>,
    pub value_type: ValueType,
    pub values: Option<HashSet<String>>,
}

impl TagMeta {
    pub fn new_any_string(name: String) -> Self {
        Self {
            name,
            color: None,
            value_type: ValueType::AnyString,
            values: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shape {
    #[serde(rename = "rectangle")]
    Rectangle,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "polygon")]
    Polygon,
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "bitmap")]
    Bitmap,
    #[serde(rename = "cuboid_3d")]
    Cuboid3D,
}

impl Default for Shape {
    fn default() -> Shape {
        Shape::Cuboid3D
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "any_number")]
    AnyNumber,
    #[serde(rename = "oneof_string")]
    OneOfString,
    #[serde(rename = "any_string")]
    AnyString,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeometryConfig {}

mod serde_color {
    use super::*;

    pub fn serialize<S>(color: &Option<palette::Srgb<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let text: Option<String> = if let Some(color) = color {
            let (r, g, b) = color.into_components();
            let text = hex_color::HexColor { r, g, b }.to_string();
            Some(text)
        } else {
            None
        };
        text.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<palette::Srgb<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let some_text: Option<String> = Option::<String>::deserialize(deserializer)?;
        let Some(text) = some_text else {
            return Ok(None);
        };
        let hex_color::HexColor { r, g, b } = text
            .parse()
            .map_err(|err| D::Error::custom(format!("invalid color code '{}': {:?}", text, err)))?;
        let color = palette::Srgb::from_components((r, g, b));
        Ok(Some(color))
    }
}
