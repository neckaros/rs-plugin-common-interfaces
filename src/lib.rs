use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

pub struct PluginInformation {
    pub name: String,
    pub kind: PluginType,
    pub version: usize,
    pub publisher: String,
    pub description: String,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum PluginType {
	ImageClassification,
    UrlParser,
}