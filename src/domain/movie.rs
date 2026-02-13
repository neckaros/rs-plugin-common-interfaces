use crate::{domain::rs_ids::RsIds, url::RsLink};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};
use crate::domain::tools::rating_serializer;


#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum MovieStatus {
    Returning,
    Released,
    InProduction,
    PostProduction,
    Planned,
    Rumored,
    Canceled,
    #[strum(default)] Other(String),
    #[default] Unknown,

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {

    #[serde(default)]
    pub id: String,
    
    pub name: String,
    #[serde(rename = "type")]
    pub kind: Option<Value>,
    pub year: Option<u16>,
    pub airdate: Option<i64>,
    pub digitalairdate: Option<i64>,

    pub duration: Option<u32>,
    pub overview: Option<String>,
    pub country: Option<String>,
    pub status: Option<MovieStatus>,

    pub imdb: Option<String>,
    pub slug: Option<String>,
    pub tmdb: Option<u64>,
    pub trakt: Option<u64>,
    pub otherids: Option<String>,

    pub lang: Option<String>,
    pub original: Option<String>,

    #[serde(serialize_with = "rating_serializer")]
    pub imdb_rating: Option<f32>,
    pub imdb_votes: Option<u64>,
    #[serde(serialize_with = "rating_serializer")]
    pub trakt_rating: Option<f32>,
    pub trakt_votes: Option<u32>,

    pub trailer: Option<RsLink>,
    
    #[serde(default)]
    pub modified: u64,
    #[serde(default)]
    pub added: u64,


    #[serde(skip_serializing_if = "Option::is_none")]
    pub watched: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u64>,

    #[serde(default)]
    pub posterv: u64,
    #[serde(default)]
    pub backgroundv: u64,
    #[serde(default)]
    pub cardv: u64

}


impl From<Movie> for RsIds {
    fn from(value: Movie) -> Self {
        RsIds {
            redseat: Some(value.id),
            trakt: value.trakt,
            slug: value.slug,
            tvdb: None,
            imdb: value.imdb,
            tmdb: value.tmdb,
            tvrage: None,
            other_ids: None,
            ..Default::default()
        }
    }
}