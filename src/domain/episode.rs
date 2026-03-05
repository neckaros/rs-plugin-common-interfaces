use crate::domain::{
    other_ids::OtherIds,
    rs_ids::{ApplyRsIds, RsIds},
    tools::rating_serializer,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub serie: String,
    pub season: u32,
    pub number: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub abs: Option<u32>,

    pub name: Option<String>,
    pub overview: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub airdate: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imdb: Option<String>,
    pub slug: Option<String>,
    pub tmdb: Option<u64>,
    pub trakt: Option<u64>,
    pub tvdb: Option<u64>,
    pub otherids: Option<OtherIds>,

    #[serde(serialize_with = "rating_serializer")]
    pub imdb_rating: Option<f32>,
    pub imdb_votes: Option<u64>,
    #[serde(serialize_with = "rating_serializer")]
    pub trakt_rating: Option<f32>,
    pub trakt_votes: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watched: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u64>,

    #[serde(default)]
    pub modified: u64,
    #[serde(default)]
    pub added: u64,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serie_name: Option<String>,
}

impl Episode {
    pub fn id(&self) -> String {
        format!("{}x{}x{}", self.serie, self.season, self.number)
    }
}

impl From<Episode> for RsIds {
    fn from(value: Episode) -> Self {
        let id = value.id();
        let mut ids = RsIds::default();
        if let Some(v) = value.trakt { ids.set("trakt", v); }
        if let Some(v) = value.slug { ids.set("slug", v); }
        if let Some(v) = value.tvdb { ids.set("tvdb", v); }
        if let Some(v) = value.imdb { ids.set("imdb", v); }
        if let Some(v) = value.tmdb { ids.set("tmdb", v); }
        if let Some(other) = value.otherids {
            for entry in other.into_vec() {
                let _ = ids.try_add(entry);
            }
        }
        if ids.try_add(id.clone()).is_err() {
            ids.set("redseat", id);
        }
        ids
    }
}

impl ApplyRsIds for Episode {
    fn apply_rs_ids(&mut self, ids: &RsIds) {
        if let Some(trakt) = ids.trakt() {
            self.trakt = Some(trakt);
        }
        if let Some(slug) = ids.slug() {
            self.slug = Some(slug.to_string());
        }
        if let Some(tvdb) = ids.tvdb() {
            self.tvdb = Some(tvdb);
        }
        if let Some(imdb) = ids.imdb() {
            self.imdb = Some(imdb.to_string());
        }
        if let Some(tmdb) = ids.tmdb() {
            self.tmdb = Some(tmdb);
        }
        let known: &[&str] = &["trakt", "slug", "tvdb", "imdb", "tmdb"];
        let mut other = self.otherids.take().unwrap_or_default();
        for (k, v) in ids.iter() {
            if !known.contains(&k.as_str()) { other.add(k, v); }
        }
        if !other.as_slice().is_empty() { self.otherids = Some(other); }
    }
}

#[cfg(test)]
mod tests {
    use super::Episode;
    use crate::domain::{
        other_ids::OtherIds,
        rs_ids::{ApplyRsIds, RsIds},
    };
    use serde_json::json;

    #[test]
    fn episode_otherids_serializes_as_array_and_rejects_string() {
        let episode = Episode {
            serie: "serie-1".to_string(),
            season: 1,
            number: 1,
            otherids: Some(OtherIds(vec!["tvmaze:ep-1".to_string()])),
            ..Default::default()
        };
        let value = serde_json::to_value(&episode).unwrap();
        assert_eq!(value.get("otherids"), Some(&json!(["tvmaze:ep-1"])));

        let parsed: Episode = serde_json::from_value(json!({
            "serie": "serie-1",
            "season": 1,
            "number": 1,
            "otherids": ["foo:bar"]
        }))
        .unwrap();
        assert_eq!(parsed.otherids, Some(OtherIds(vec!["foo:bar".to_string()])));

        let invalid = serde_json::from_value::<Episode>(json!({
            "serie": "serie-1",
            "season": 1,
            "number": 1,
            "otherids": "foo:bar"
        }));
        assert!(invalid.is_err());
    }

    #[test]
    fn episode_apply_rs_ids_updates_only_present_values() {
        let mut episode = Episode {
            serie: "serie".to_string(),
            season: 1,
            number: 1,
            tmdb: Some(22),
            ..Default::default()
        };
        let mut ids = RsIds::default();
        ids.set("trakt", 55u64);
        ids.set("tvdb", 66u64);
        ids.set("imdb", "tt0000066");
        ids.set("slug", "episode-slug");

        episode.apply_rs_ids(&ids);

        assert_eq!(episode.trakt, Some(55));
        assert_eq!(episode.tvdb, Some(66));
        assert_eq!(episode.imdb.as_deref(), Some("tt0000066"));
        assert_eq!(episode.slug.as_deref(), Some("episode-slug"));
        assert_eq!(episode.tmdb, Some(22));
    }
}
