// Your plugin must implement:
// exists(path: RsProviderPath) -> bool;
// remove(path: RsProviderPath) -> bool;
// infos(path: RsProviderPath) -> MediaForUpdate;
// get_file(path: RsProviderPath) -> RsRequest;
// add_file(path: RsProviderPath) -> RsRequest;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsProviderPath {
    pub root: String,
    pub source: String,
}

