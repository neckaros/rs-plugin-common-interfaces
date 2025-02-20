// Your plugin must implement:
// exists(path: RsProviderPath) -> bool;
// remove(path: RsProviderPath) -> bool;
// infos(path: RsProviderPath) -> MediaForUpdate;
// get(path: RsProviderPath) -> RsRequest;
// add(path: RsProviderAddRequest) -> RsRequest;

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::RsRequest;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsProviderPath {
    pub root: Option<String>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsProviderAddRequest {
    pub root: String,
    pub name: String,
    pub overwrite: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsProviderAddResponse {
    pub request: RsRequest,
    pub multipart: Option<String>,
    pub source: Option<String>,
    pub packets: Option<u64>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[strum(serialize_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub enum RsProviderEntryType {
    Directory,
    File,
    #[default]
    Other
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsProviderEntry {
    pub source: String,
    pub kind: RsProviderEntryType,
    pub size: Option<u64>,
    pub mimetype: Option<String>,

    pub hash: Option<String>,

    pub added: Option<i64>,
    pub modified: Option<i64>,
    pub created: Option<i64>,
}

impl RsProviderEntry {
    pub fn mime_or_default(&self) -> String {
        self.mimetype.clone().unwrap_or("application/octet-stream".to_string())
    }
}
