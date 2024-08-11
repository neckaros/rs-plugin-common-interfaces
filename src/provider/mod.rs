// Your plugin must implement:
// exists(path: RsProviderPath) -> bool;
// remove(path: RsProviderPath) -> bool;
// infos(path: RsProviderPath) -> MediaForUpdate;
// get(path: RsProviderPath) -> RsRequest;
// add(path: RsProviderAddRequest) -> RsRequest;

use serde::{Deserialize, Serialize};

use crate::{RsFileType, RsRequest};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsProviderPath {
    pub root: Option<String>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsProviderAddRequest {
    pub name: RsProviderPath,
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


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsProviderEntry {
    pub source: String,
    pub kind: RsFileType,
    pub size: Option<u64>,

    pub hash: Option<String>,

    pub added: Option<u64>,
    pub modified: Option<u64>,
    pub created: Option<i64>,
}
