use serde::{Deserialize, Serialize};
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
    #[default]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum CredentialType {
	Password,
    Oauth,
    Token,
}