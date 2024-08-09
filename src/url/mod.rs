use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;

#[cfg(feature = "rusqlite")]
pub mod rusqlite;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")] 
pub struct RsLink {
	pub platform: String,
    #[serde(rename = "type")]
    pub kind: Option<RsLinkType>,
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum RsLinkType {
	Post,
    Profile,
    Video,
    Photo,
    File,
    #[default]
    Other,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsPluginRequest<T> {
    pub request: T,
    pub plugin_settings: Value,
}

