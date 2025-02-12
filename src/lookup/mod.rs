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
    pub serie: String,
    pub imdb: Option<String>,
    pub slug: Option<String>,
    pub tmdb: Option<u64>,
    pub trakt: Option<u64>,
    pub tvdb: Option<u64>,
    pub otherids: Option<String>,

    pub season: u32,
    pub number: Option<u32>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupMovie {
    pub name: String,
    pub imdb: Option<String>,
    pub slug: Option<String>,
    pub tmdb: Option<u64>,
    pub trakt: Option<u64>,
    pub otherids: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookup {
    pub kind: ElementType,
    pub name: String,
    pub ids: RsIds
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupWrapper {
    pub query: RsLookup,
    pub credential: Option<PluginCredential>,
    pub params: Option<HashMap<String, String>>

}


