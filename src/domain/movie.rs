use crate::domain::tools::rating_serializer;
use crate::{
    domain::{
        other_ids::OtherIds,
        rs_ids::{ApplyRsIds, RsIds},
    },
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

impl ApplyRsIds for Movie {
    fn apply_rs_ids(&mut self, ids: &RsIds) {
        if let Some(redseat) = ids.redseat() {
            self.id = redseat.to_string();
        }
        if let Some(trakt) = ids.trakt() {
            self.trakt = Some(trakt);
        }
        if let Some(slug) = ids.slug() {
            self.slug = Some(slug.to_string());
        }
        if let Some(imdb) = ids.imdb() {
            self.imdb = Some(imdb.to_string());
        }
        if let Some(tmdb) = ids.tmdb() {
            self.tmdb = Some(tmdb);
        }
        let known: &[&str] = &["redseat", "trakt", "slug", "imdb", "tmdb"];
        let mut other = self.otherids.take().unwrap_or_default();
        for (k, v) in ids.iter() {
            if !known.contains(&k.as_str()) { other.add(k, v); }
        }
        if !other.as_slice().is_empty() { self.otherids = Some(other); }
    }
}

#[cfg(test)]
mod tests {
    use super::Movie;
    use crate::domain::{
        other_ids::OtherIds,
        rs_ids::{ApplyRsIds, RsIds},
    };
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

    #[test]
    fn movie_apply_rs_ids_updates_only_present_values() {
        let mut movie = Movie {
            id: "movie-old".to_string(),
            name: "Movie".to_string(),
            tmdb: Some(10),
            ..Default::default()
        };
        let mut ids = RsIds::default();
        ids.set("redseat", "movie-new");
        ids.set("trakt", 100u64);
        ids.set("imdb", "tt0000100");
        ids.set("slug", "movie-slug");

        movie.apply_rs_ids(&ids);

        assert_eq!(movie.id, "movie-new");
        assert_eq!(movie.trakt, Some(100));
        assert_eq!(movie.imdb.as_deref(), Some("tt0000100"));
        assert_eq!(movie.slug.as_deref(), Some("movie-slug"));
        assert_eq!(movie.tmdb, Some(10));
    }
}
