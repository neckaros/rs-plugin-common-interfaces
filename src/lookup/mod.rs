use std::collections::HashMap;

use crate::{domain::rs_ids::RsIds, request::RsRequest};
use crate::{ElementType, PluginCredential};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum RsLookupSourceResult {
    Requests(Vec<RsRequest>),
    NotFound,
    #[default]
    NotApplicable
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupEpisode {
    pub season: u32,
    pub number: Option<u32>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookup<T> {
    pub kind: ElementType,
    pub name: String,
    pub ids: RsIds,

    pub infos: Option<T>
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupWrapper<T> {
    pub query: RsLookup<T>,
    pub credential: Option<PluginCredential>,
    pub params: Option<HashMap<String, String>>

}


