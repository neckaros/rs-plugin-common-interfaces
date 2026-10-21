// Your plugin must implement:
// exists(path: RsProviderPath) -> bool;
// remove(path: RsProviderPath) -> bool;
// infos(path: RsProviderPath) -> MediaForUpdate;
// get(path: RsProviderPath) -> RsRequest;
// add(path: RsProviderAddRequest) -> RsRequest;
// Optional:
// storage_info(path: RsProviderPath) -> RsProviderStorage; (`root` of the library, empty `source`)

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
    pub packets: Option<u64>,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[strum(serialize_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub enum RsProviderEntryType {
    Directory,
    File,
    #[default]
    Other,
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
        self.mimetype
            .clone()
            .unwrap_or("application/octet-stream".to_string())
    }
}

/// Space of the storage behind a provider, in bytes, returned by the optional
/// `storage_info` export. Every field is optional: give what the provider knows.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsProviderStorage {
    /// Bytes used on the storage, by everything on it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<u64>,
    /// Bytes that can still be written. Omitted when there is no quota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free: Option<u64>,
    /// Size of the storage. Omitted when there is no quota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    /// Identifies the storage (an account, a disk): libraries with the same value
    /// share it, so its space is counted once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn provider_storage_is_camel_case_and_omits_what_is_unknown() {
        let storage = RsProviderStorage {
            used: Some(512),
            free: Some(488),
            total: Some(1000),
            volume: Some("account-1".to_string()),
        };
        let value = serde_json::to_value(&storage).unwrap();
        assert_eq!(
            value,
            json!({ "used": 512, "free": 488, "total": 1000, "volume": "account-1" })
        );
        assert_eq!(
            serde_json::from_value::<RsProviderStorage>(value).unwrap(),
            storage
        );

        let unlimited = RsProviderStorage {
            used: Some(512),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&unlimited).unwrap(),
            json!({ "used": 512 })
        );
        assert_eq!(
            serde_json::from_value::<RsProviderStorage>(json!({ "used": 512 })).unwrap(),
            unlimited
        );
    }
}
