use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BackupFile {
    pub backup: String,
    pub library: Option<String>,
    pub file: String,
    pub id: String,
    pub path: String,
    pub hash: String,
    pub sourcehash: String,
    pub size: u64,
    pub modified: i64,
    pub added: i64,
    pub iv: Option<String>,
    pub thumb_size: Option<u64>,
    pub info_size: Option<u64>,
    pub error: Option<String>,
}
