use std::collections::HashMap;

use serde::{Deserialize, Serialize};
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

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
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

impl From<(String, String)> for RsLink {
    fn from((platform, id): (String, String)) -> Self {
        RsLink {
            platform,
            kind: None,
            id,
            file: None,
            user: None,
            plugin: None,
        }
    }
}

pub trait ToRsLinks {
    fn to_rs_links(self) -> Vec<RsLink>;
}

// Implement for HashMap
impl ToRsLinks for HashMap<String, Option<String>> {
    fn to_rs_links(self) -> Vec<RsLink> {
        self.into_iter()
            .filter_map(|(platform, opt_id)| opt_id.map(|id| RsLink::from((platform, id))))
            .collect()
    }
}

// Optional implementation for Option<HashMap>
impl ToRsLinks for Option<HashMap<String, Option<String>>> {
    fn to_rs_links(self) -> Vec<RsLink> {
        match self {
            Some(map) => map.to_rs_links(),
            None => Vec::new(),
        }
    }
}
