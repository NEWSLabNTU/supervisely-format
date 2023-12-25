use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub value: Option<TagValue>,
    pub id: Option<usize>,
    #[serde(rename = "tagId")]
    pub tag_id: Option<usize>,
    #[serde(rename = "labelerLogin")]
    pub labeler_login: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

impl Tag {
    pub fn new<V>(name: String, value: V) -> Self
    where
        V: Into<TagValue>,
    {
        Self {
            name,
            value: Some(value.into()),
            id: None,
            tag_id: None,
            labeler_login: None,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TagValue {
    Number(isize),
    Text(String),
    OneOf(String),
}

impl From<String> for TagValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<isize> for TagValue {
    fn from(value: isize) -> Self {
        Self::Number(value)
    }
}

impl From<u128> for TagValue {
    fn from(value: u128) -> Self {
        Self::Number(value as isize)
    }
}
