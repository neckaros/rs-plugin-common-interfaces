use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{domain::{other_ids::OtherIds, rs_ids::{ApplyRsIds, RsIds}}, url::RsLink, Gender};

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

impl ApplyRsIds for Person {
    fn apply_rs_ids(&mut self, ids: &RsIds) {
        if let Some(trakt) = ids.trakt() { self.trakt = Some(trakt); }
        if let Some(slug) = ids.slug() { self.slug = Some(slug.to_string()); }
        if let Some(imdb) = ids.imdb() { self.imdb = Some(imdb.to_string()); }
        if let Some(tmdb) = ids.tmdb() { self.tmdb = Some(tmdb); }
        let known: &[&str] = &["redseat", "trakt", "slug", "imdb", "tmdb"];
        let mut other = self.otherids.take().unwrap_or_default();
        for (k, v) in ids.iter() {
            if !known.contains(&k.as_str()) { other.add(k, v); }
        }
        if !other.as_slice().is_empty() { self.otherids = Some(other); }
    }
}

impl From<Person> for RsIds {
    fn from(value: Person) -> Self {
        let mut ids = RsIds::default();
        if let Some(v) = value.trakt { ids.set("trakt", v); }
        if let Some(v) = value.slug { ids.set("slug", v); }
        if let Some(v) = value.imdb { ids.set("imdb", v); }
        if let Some(v) = value.tmdb { ids.set("tmdb", v); }
        if let Some(other) = value.otherids {
            for entry in other.into_vec() {
                let _ = ids.try_add(entry);
            }
        }
        if ids.try_add(value.id.clone()).is_err() {
            ids.set("redseat", value.id);
        }
        ids
    }
}
