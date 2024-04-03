use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;

#[cfg(feature = "rusqlite")]
pub mod rusqlite;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct PluginInformation {
    pub name: String,
    pub kind: PluginType,
    pub version: usize,
    pub publisher: String,
    pub description: String,
    pub credential_kind: Option<CredentialType>,
    pub oauth_url: Option<String>
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum PluginType {
	ImageClassification,
    UrlParser,
    Request,
    Lookup,
    #[default]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum CredentialType {
	Url,
	Password,
    Oauth,
    #[default]
    Token,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct PluginCredential {
    pub kind: CredentialType,
    pub login: Option<String>,
    pub password: Option<String>,
    pub settings: Value,
    pub user_ref: Option<String>,
    pub refresh_token: Option<String>,
    pub expires: Option<u64>,
}
