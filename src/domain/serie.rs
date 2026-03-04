use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};

use crate::domain::{
    other_ids::OtherIds,
    rs_ids::{ApplyRsIds, RsIds},
    tools::rating_serializer,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(from = "String", into = "String")]
#[strum(serialize_all = "snake_case")]
pub enum SerieType {
    Tv,
    TvShort,
    TvSpecial,
    Manga,
    Anime,
    Movie,
    Special,
    Ova,
    Ona,
    Music,
    Novel,
    OneShot,
    LightNovel,
    Doujinshi,
    Manhwa,
    Manhua,
    Oel,
    Cm,
    Pv,
    Book,
    #[strum(default)]
    Custom(String),
}

impl SerieType {
    pub fn from_string(value: &str) -> Self {
        let normalized = value.to_ascii_lowercase();
        match SerieType::try_from(normalized.as_str()) {
            Ok(SerieType::Custom(_)) => SerieType::Custom(value.to_string()),
            Ok(parsed) => parsed,
            Err(_) => SerieType::Custom(value.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SerieType::Tv => "tv".to_string(),
            SerieType::TvShort => "tv_short".to_string(),
            SerieType::TvSpecial => "tv_special".to_string(),
            SerieType::Manga => "manga".to_string(),
            SerieType::Anime => "anime".to_string(),
            SerieType::Movie => "movie".to_string(),
            SerieType::Special => "special".to_string(),
            SerieType::Ova => "ova".to_string(),
            SerieType::Ona => "ona".to_string(),
            SerieType::Music => "music".to_string(),
            SerieType::Novel => "novel".to_string(),
            SerieType::OneShot => "one_shot".to_string(),
            SerieType::LightNovel => "light_novel".to_string(),
            SerieType::Doujinshi => "doujinshi".to_string(),
            SerieType::Manhwa => "manhwa".to_string(),
            SerieType::Manhua => "manhua".to_string(),
            SerieType::Oel => "oel".to_string(),
            SerieType::Cm => "cm".to_string(),
            SerieType::Pv => "pv".to_string(),
            SerieType::Book => "book".to_string(),
            SerieType::Custom(value) => value.clone(),
        }
    }
}

impl From<String> for SerieType {
    fn from(value: String) -> Self {
        SerieType::from_string(&value)
    }
}

impl From<SerieType> for String {
    fn from(value: SerieType) -> Self {
        value.to_string()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum SerieStatus {
    Returning,
    InProduction,
    PostProduction,
    Planned,
    Rumored,
    Ended,
    Released,
    Canceled,
    Pilot,
    #[strum(default)]
    Other(String),
    #[default]
    Unknown,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Serie {
    #[serde(default)]
    pub id: String,

    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub kind: Option<SerieType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SerieStatus>,
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imdb: Option<String>,
    pub slug: Option<String>,
    pub tmdb: Option<u64>,
    pub trakt: Option<u64>,
    pub tvdb: Option<u64>,
    pub otherids: Option<OtherIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openlibrary_work_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anilist_manga_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mangadex_manga_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myanimelist_manga_id: Option<u64>,

    #[serde(serialize_with = "rating_serializer")]
    pub imdb_rating: Option<f32>,
    pub imdb_votes: Option<u64>,
    #[serde(serialize_with = "rating_serializer")]
    pub trakt_rating: Option<f32>,
    pub trakt_votes: Option<u64>,

    pub trailer: Option<String>,

    pub year: Option<u16>,

    pub max_created: Option<i64>,

    #[serde(default)]
    pub modified: u64,
    #[serde(default)]
    pub added: u64,

    #[serde(default)]
    pub posterv: u64,
    #[serde(default)]
    pub backgroundv: u64,
    #[serde(default)]
    pub cardv: u64,
}

impl From<Serie> for RsIds {
    fn from(value: Serie) -> Self {
        let mut ids = RsIds::default();
        if let Some(v) = value.trakt { ids.set("trakt", v); }
        if let Some(v) = value.slug { ids.set("slug", v); }
        if let Some(v) = value.tvdb { ids.set("tvdb", v); }
        if let Some(v) = value.imdb { ids.set("imdb", v); }
        if let Some(v) = value.tmdb { ids.set("tmdb", v); }
        if let Some(v) = value.anilist_manga_id { ids.set("anilist", v); }
        if let Some(v) = value.mangadex_manga_uuid { ids.set("mangadex", v); }
        if let Some(v) = value.myanimelist_manga_id { ids.set("mal", v); }
        if let Some(v) = value.openlibrary_work_id { ids.set("olwid", v); }
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

impl ApplyRsIds for Serie {
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
        if let Some(tvdb) = ids.tvdb() {
            self.tvdb = Some(tvdb);
        }
        if let Some(imdb) = ids.imdb() {
            self.imdb = Some(imdb.to_string());
        }
        if let Some(tmdb) = ids.tmdb() {
            self.tmdb = Some(tmdb);
        }
        if let Some(olwid) = ids.openlibrary_work_id() {
            self.openlibrary_work_id = Some(olwid.to_string());
        }
        if let Some(anilist) = ids.anilist_manga_id() {
            self.anilist_manga_id = Some(anilist);
        }
        if let Some(mangadex) = ids.mangadex_manga_uuid() {
            self.mangadex_manga_uuid = Some(mangadex.to_string());
        }
        if let Some(mal) = ids.myanimelist_manga_id() {
            self.myanimelist_manga_id = Some(mal);
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{Serie, SerieType};
    use crate::domain::{
        other_ids::OtherIds,
        rs_ids::{ApplyRsIds, RsIds},
    };

    #[test]
    fn serie_type_serde() {
        let known: SerieType = serde_json::from_str("\"tv\"").unwrap();
        assert_eq!(known, SerieType::Tv);
        let known_snake_case: SerieType = serde_json::from_str("\"one_shot\"").unwrap();
        assert_eq!(known_snake_case, SerieType::OneShot);
        let known_case_insensitive: SerieType = serde_json::from_str("\"TV_SPECIAL\"").unwrap();
        assert_eq!(known_case_insensitive, SerieType::TvSpecial);

        let custom: SerieType = serde_json::from_str("\"manhwa\"").unwrap();
        assert_eq!(custom, SerieType::Manhwa);
        let custom: SerieType = serde_json::from_str("\"webtoon\"").unwrap();
        assert_eq!(custom, SerieType::Custom("webtoon".to_string()));

        assert_eq!(
            serde_json::to_string(&SerieType::Anime).unwrap(),
            "\"anime\""
        );
        assert_eq!(
            serde_json::to_string(&SerieType::Custom("lightnovel".to_string())).unwrap(),
            "\"lightnovel\""
        );
        assert_eq!(
            serde_json::to_string(&SerieType::LightNovel).unwrap(),
            "\"light_novel\""
        );
    }

    #[test]
    fn serie_otherids_serializes_as_array_and_rejects_string() {
        let serie = Serie {
            id: "serie-1".to_string(),
            name: "Serie 1".to_string(),
            otherids: Some(OtherIds(vec!["tvmaze:123".to_string()])),
            ..Default::default()
        };
        let value = serde_json::to_value(&serie).unwrap();
        assert_eq!(value.get("otherids"), Some(&json!(["tvmaze:123"])));

        let parsed: Serie = serde_json::from_value(json!({
            "id": "serie-1",
            "name": "Serie 1",
            "otherids": ["anidb:12"]
        }))
        .unwrap();
        assert_eq!(
            parsed.otherids,
            Some(OtherIds(vec!["anidb:12".to_string()]))
        );

        let invalid = serde_json::from_value::<Serie>(json!({
            "id": "serie-1",
            "name": "Serie 1",
            "otherids": "anidb:12"
        }));
        assert!(invalid.is_err());
    }

    #[test]
    fn serie_apply_rs_ids_updates_only_present_values() {
        let mut serie = Serie {
            id: "serie-old".to_string(),
            name: "Serie 1".to_string(),
            tvdb: Some(500),
            ..Default::default()
        };
        let mut ids = RsIds::default();
        ids.set("redseat", "serie-new");
        ids.set("trakt", 101u64);
        ids.set("imdb", "tt0000101");
        ids.set("tmdb", 201u64);
        ids.set("slug", "serie-slug");
        ids.set("anilist", 301u64);
        ids.set("mangadex", "uuid-123");
        ids.set("mal", 401u64);
        ids.set("olwid", "olw-123");

        serie.apply_rs_ids(&ids);

        assert_eq!(serie.id, "serie-new");
        assert_eq!(serie.trakt, Some(101));
        assert_eq!(serie.imdb.as_deref(), Some("tt0000101"));
        assert_eq!(serie.tmdb, Some(201));
        assert_eq!(serie.slug.as_deref(), Some("serie-slug"));
        assert_eq!(serie.tvdb, Some(500));
        assert_eq!(serie.anilist_manga_id, Some(301));
        assert_eq!(serie.mangadex_manga_uuid.as_deref(), Some("uuid-123"));
        assert_eq!(serie.myanimelist_manga_id, Some(401));
        assert_eq!(serie.openlibrary_work_id.as_deref(), Some("olw-123"));
    }
}
