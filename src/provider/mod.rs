// Your plugin must implement:
// exists(path: RsProviderPath) -> bool;
// remove(path: RsProviderPath) -> bool;
// infos(path: RsProviderPath) -> MediaForUpdate;
// get(path: RsProviderPath) -> RsRequest;
// add(path: RsProviderAddRequest) -> RsRequest;

use serde::{Deserialize, Serialize};

use crate::RsFileType;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsProviderPath {
    pub root: Option<String>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsProviderAddRequest {
    pub path: RsProviderPath,
    pub overwrite: bool,
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
