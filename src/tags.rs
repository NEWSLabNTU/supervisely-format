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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TagValue {
    Number(isize),
    Text(String),
    OneOf(String),
}
