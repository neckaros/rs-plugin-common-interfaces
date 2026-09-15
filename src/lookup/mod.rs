use std::collections::HashMap;

use crate::domain::book::Book;
use crate::domain::episode::Episode;
use crate::domain::media::Media;
use crate::domain::movie::Movie;
use crate::domain::person::{Person, PersonType};
use crate::domain::rs_ids::{ApplyRsIds, RsIds};
use crate::domain::serie::Serie;
use crate::domain::Relations;
use crate::request::RsGroupDownload;
use crate::request::RsRequest;
use crate::{CustomParamTypes, PluginCredential};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum RsLookupSourceResult {
    Requests(Vec<RsRequest>),
    GroupRequest(Vec<RsGroupDownload>),
    NotFound,
    #[default]
    NotApplicable,
}

/// A person constraint for title metadata searches.
///
/// Omitting `role` performs a broad search for the person regardless of their
/// relationship to the title. Supplying it restricts the match to that exact
/// canonical or custom role.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupPersonFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<RsIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<PersonType>,
}

/// A series constraint for title metadata searches.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupSerieFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<RsIds>,
}

/// A tag constraint for title metadata searches.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupTagFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<RsIds>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupPerson {
    pub name: Option<String>,
    pub ids: Option<RsIds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupSerie {
    pub name: Option<String>,
    pub ids: Option<RsIds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<RsLookupPersonFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<RsLookupSerieFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RsLookupTagFilter>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupSerieSeason {
    pub name: Option<String>,
    pub ids: Option<RsIds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupEpisode {
    pub name: Option<String>,
    pub ids: Option<RsIds>,

    pub season: u32,
    pub number: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupBook {
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    pub ids: Option<RsIds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<RsLookupPersonFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<RsLookupSerieFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RsLookupTagFilter>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_key: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupSong {
    pub title: Option<String>,
    pub author: Option<String>,
    pub album: Option<String>,
    pub ids: Option<RsIds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupMedia {
    pub search: Option<String>,
    pub ids: Option<RsIds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupMovie {
    pub name: Option<String>,
    pub ids: Option<RsIds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<RsLookupPersonFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<RsLookupSerieFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RsLookupTagFilter>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString)]
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
    Song(RsLookupSong),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum RsLookupMatchType {
    ExactId,
    ExactText,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum RsLookupMetadataResult {
    Book(Book),
    Media(Media),
    Episode(Episode),
    Movie(Movie),
    Person(Person),
    Serie(Serie),
    #[default]
    None,
}

impl RsLookupMetadataResult {
    /// Extract all IDs from the inner entity as an `RsIds` set.
    pub fn extract_ids(&self) -> Option<RsIds> {
        match self {
            Self::Movie(m) => Some(RsIds::from(m.clone())),
            Self::Serie(s) => Some(RsIds::from(s.clone())),
            Self::Book(b) => Some(RsIds::from(b.clone())),
            Self::Person(p) => Some(RsIds::from(p.clone())),
            Self::Episode(e) => Some(RsIds::from(e.clone())),
            Self::Media(_) | Self::None => None,
        }
    }

    /// Apply merged IDs back onto the inner entity.
    pub fn apply_ids(&mut self, ids: &RsIds) {
        match self {
            Self::Movie(m) => m.apply_rs_ids(ids),
            Self::Serie(s) => s.apply_rs_ids(ids),
            Self::Book(b) => b.apply_rs_ids(ids),
            Self::Person(p) => p.apply_rs_ids(ids),
            Self::Episode(e) => e.apply_rs_ids(ids),
            Self::Media(_) | Self::None => {}
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupMetadataResults {
    pub results: Vec<RsLookupMetadataResultWrapper>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupMetadataResultWrapper {
    pub metadata: RsLookupMetadataResult,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Relations>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<RsLookupMatchType>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RsLookupWrapper {
    pub query: RsLookupQuery,
    pub credential: Option<PluginCredential>,
    pub params: Option<HashMap<String, CustomParamTypes>>,
}

#[cfg(test)]
mod filter_tests {
    use super::*;

    #[test]
    fn title_filters_serialize_with_optional_person_role() {
        let query = RsLookupQuery::Movie(RsLookupMovie {
            name: Some("Ocean's Eleven".into()),
            people: Some(vec![
                RsLookupPersonFilter {
                    name: Some("George Clooney".into()),
                    ..Default::default()
                },
                RsLookupPersonFilter {
                    name: Some("Steven Soderbergh".into()),
                    role: Some(PersonType::Director),
                    ..Default::default()
                },
            ]),
            series: Some(vec![RsLookupSerieFilter {
                name: Some("Ocean's".into()),
                ..Default::default()
            }]),
            tags: Some(vec![
                RsLookupTagFilter {
                    name: Some("Heist".into()),
                    ..Default::default()
                },
                RsLookupTagFilter {
                    ids: Some("wikidata:Q123".to_string().try_into().unwrap()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        });

        let wire = serde_json::to_value(&query).unwrap();
        assert_eq!(wire["movie"]["people"][0]["name"], "George Clooney");
        assert!(wire["movie"]["people"][0].get("role").is_none());
        assert_eq!(wire["movie"]["people"][1]["role"], "Director");
        assert_eq!(wire["movie"]["series"][0]["name"], "Ocean's");
        assert_eq!(wire["movie"]["tags"][0]["name"], "Heist");
        assert_eq!(wire["movie"]["tags"][1]["ids"]["wikidata"], "Q123");
        assert_eq!(
            serde_json::from_value::<RsLookupQuery>(wire).unwrap(),
            query
        );
    }

    #[test]
    fn unused_title_filters_are_omitted() {
        for query in [
            RsLookupQuery::Book(RsLookupBook::default()),
            RsLookupQuery::Movie(RsLookupMovie::default()),
            RsLookupQuery::Serie(RsLookupSerie::default()),
        ] {
            let wire = serde_json::to_value(query).unwrap();
            let fields = wire.as_object().unwrap().values().next().unwrap();
            for field in ["people", "series", "tags"] {
                assert!(fields.get(field).is_none());
            }
        }
    }
}
