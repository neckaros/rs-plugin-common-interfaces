use std::collections::HashMap;

use crate::domain::book::Book;
use crate::domain::episode::Episode;
use crate::domain::media::Media;
use crate::domain::movie::Movie;
use crate::domain::person::Person;
use crate::domain::serie::Serie;
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
    pub name: Option<String>,
    pub ids: Option<RsIds>
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupSerie {
    pub name: Option<String>,
    pub ids: Option<RsIds>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupSerieSeason {
    pub name: Option<String>,
    pub ids: Option<RsIds>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupEpisode {
    pub name: Option<String>,
    pub ids: Option<RsIds>,

    pub season: u32,
    pub number: Option<u32>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupBook {
    pub name: Option<String>,
    pub ids: Option<RsIds>
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupSong {
    pub title: Option<String>,
    pub author: Option<String>,
    pub album: Option<String>,
    pub ids: Option<RsIds>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupMedia {
    pub search: Option<String>,
    pub ids: Option<RsIds>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupMovie {
    pub name: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum RsLookupMetadataResult {
    Book(Book),
    Media(Media),
    Episode(Episode),
    Movie(Movie),
    Person(Person),
    Serie(Serie),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupMetadataResultWithImages{
    pub metadata: RsLookupMetadataResult,
    pub images: Vec<crate::domain::external_images::ExternalImage>,
    pub lookup_tags: Option<Vec<String>>,
    pub lookup_people: Option<Vec<String>>
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")] 
pub struct RsLookupWrapper {
    pub query: RsLookupQuery,
    pub credential: Option<PluginCredential>,
    pub params: Option<HashMap<String, String>>

}


