use crate::domain::tools::rating_serializer;
use crate::{
    domain::{other_ids::OtherIds, rs_ids::RsIds},
    url::RsLink,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};

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
    #[strum(default)]
    Other(String),
    #[default]
    Unknown,
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
    pub otherids: Option<OtherIds>,

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
    pub cardv: u64,
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
            other_ids: value.otherids,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Movie;
    use crate::domain::other_ids::OtherIds;
    use serde_json::json;

    #[test]
    fn movie_otherids_serializes_as_array_and_rejects_string() {
        let movie = Movie {
            id: "movie-1".to_string(),
            name: "Movie 1".to_string(),
            otherids: Some(OtherIds(vec!["imdb:tt1234567".to_string()])),
            ..Default::default()
        };
        let value = serde_json::to_value(&movie).unwrap();
        assert_eq!(value.get("otherids"), Some(&json!(["imdb:tt1234567"])));

        let parsed: Movie = serde_json::from_value(json!({
            "id": "movie-1",
            "name": "Movie 1",
            "otherids": ["tmdb:42"]
        }))
        .unwrap();
        assert_eq!(parsed.otherids, Some(OtherIds(vec!["tmdb:42".to_string()])));

        let invalid = serde_json::from_value::<Movie>(json!({
            "id": "movie-1",
            "name": "Movie 1",
            "otherids": "tmdb:42"
        }));
        assert!(invalid.is_err());
    }
}
