use std::collections::HashMap;

use crate::{domain::rs_ids::RsIds, request::RsRequest};
use crate::PluginCredential;
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
pub struct RsLookupPerson {
    pub name: String,
    pub ids: Option<RsIds>
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupSerie {
    pub name: String,
    pub ids: Option<RsIds>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupSerieSeason {
    pub name: String,
    pub ids: Option<RsIds>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupEpisode {
    pub serie: String,
    pub ids: Option<RsIds>,

    pub season: u32,
    pub number: Option<u32>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupBook {
    pub title: String,
    pub author: String,
    pub ids: Option<RsIds>
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupSong {
    pub title: String,
    pub author: Option<String>,
    pub album: Option<String>,
    pub ids: Option<RsIds>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupMedia {
    pub search: String,
    pub ids: Option<RsIds>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupMovie {
    pub name: String,
    pub ids: Option<RsIds>
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum RsLookupQuery {
    Book(RsLookupBook),
    Media(RsLookupMedia),
    Episode(RsLookupEpisode),
    Movie(RsLookupMovie),
    Person(RsLookupPerson),
    Serie(RsLookupSerie),
    SerieSeason(RsLookupSerieSeason),
    Song(RsLookupSong)
}



#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupWrapper {
    pub query: RsLookupQuery,
    pub credential: Option<PluginCredential>,
    pub params: Option<HashMap<String, String>>

}


