use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::other_ids::OtherIds;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub parent: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub alt: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    pub modified: u64,
    pub added: u64,
    pub generated: bool,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otherids: Option<OtherIds>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TagForUpdate {
    pub name: Option<String>,
    pub parent: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,

    pub alt: Option<Vec<String>>,
    pub add_alts: Option<Vec<String>>,
    pub remove_alts: Option<Vec<String>>,


    pub add_otherids: Option<Vec<String>>,
    pub remove_otherids: Option<Vec<String>>,

    pub thumb: Option<String>,
    pub params: Option<Value>,
    pub generated: Option<bool>,

    pub migrate_to: Option<String>,
    pub otherids: Option<OtherIds>,
}

impl Tag {
    pub fn full_path(&self) -> String {
        format!("{}{}", self.path, self.name)
    }
    pub fn childs_path(&self) -> String {
        format!("{}{}/", self.path, self.name)
    }
}
