use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{domain::{other_ids::OtherIds, rs_ids::RsIds}, url::RsLink, Gender};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,
    pub name: String,
    pub socials: Option<Vec<RsLink>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portrait: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<i64>,
    pub modified: u64,
    pub added: u64,
    pub posterv: u32,
    #[serde(default)]
    pub generated: bool,

    pub imdb: Option<String>,
    pub slug: Option<String>,
    pub tmdb: Option<u64>,
    pub trakt: Option<u64>,

    pub death: Option<i64>,
    pub gender: Option<Gender>,
    pub country: Option<String>,
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otherids: Option<OtherIds>,
}

impl From<Person> for RsIds {
    fn from(value: Person) -> Self {
        let mut ids = RsIds {
            trakt: value.trakt,
            slug: value.slug,
            imdb: value.imdb,
            tmdb: value.tmdb,
            other_ids: value.otherids,
            ..Default::default()
        };
        if ids.try_add(value.id.clone()).is_err() {
            ids.redseat = Some(value.id);
        }
        ids
    }
}
