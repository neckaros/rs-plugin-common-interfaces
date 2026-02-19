use core::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::domain::other_ids::OtherIds;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
pub enum RsIdsError {
    InvalidId(),
    NotAMediaId(String),
    NoMediaIdRequired(Box<RsIds>),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for RsIdsError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for RsIdsError {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redseat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trakt: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tvdb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imdb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmdb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tvrage: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_ids: Option<OtherIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn13: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openlibrary_edition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openlibrary_work_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_books_volume_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anilist_manga_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mangadex_manga_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myanimelist_manga_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asin: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum RsDecimalKey {
    NegInf,
    Finite(i64),
    PosInf,
    NaN(u64),
}

fn normalize_manga_decimal(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn decimal_key(value: f64) -> RsDecimalKey {
    if value.is_nan() {
        return RsDecimalKey::NaN(value.to_bits());
    }
    if value == f64::INFINITY {
        return RsDecimalKey::PosInf;
    }
    if value == f64::NEG_INFINITY {
        return RsDecimalKey::NegInf;
    }
    RsDecimalKey::Finite((normalize_manga_decimal(value) * 1000.0).round() as i64)
}

fn optional_decimal_key(value: Option<f64>) -> Option<RsDecimalKey> {
    value.map(decimal_key)
}

impl PartialEq for RsIds {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for RsIds {}

impl PartialOrd for RsIds {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RsIds {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.redseat.cmp(&other.redseat);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.trakt.cmp(&other.trakt);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.slug.cmp(&other.slug);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.tvdb.cmp(&other.tvdb);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.imdb.cmp(&other.imdb);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.tmdb.cmp(&other.tmdb);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.tvrage.cmp(&other.tvrage);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.other_ids.cmp(&other.other_ids);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.isbn13.cmp(&other.isbn13);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self
            .openlibrary_edition_id
            .cmp(&other.openlibrary_edition_id);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.openlibrary_work_id.cmp(&other.openlibrary_work_id);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self
            .google_books_volume_id
            .cmp(&other.google_books_volume_id);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.anilist_manga_id.cmp(&other.anilist_manga_id);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.mangadex_manga_uuid.cmp(&other.mangadex_manga_uuid);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.myanimelist_manga_id.cmp(&other.myanimelist_manga_id);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = optional_decimal_key(self.volume).cmp(&optional_decimal_key(other.volume));
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = optional_decimal_key(self.chapter).cmp(&optional_decimal_key(other.chapter));
        if ord != Ordering::Equal {
            return ord;
        }
        self.asin.cmp(&other.asin)
    }
}

impl RsIds {
    fn parse_manga_details(
        details: &[&str],
        value: &str,
    ) -> Result<(Option<f64>, Option<f64>), RsIdsError> {
        let mut volume = None;
        let mut chapter = None;

        for detail in details {
            let detail_parts = detail.split(':').collect::<Vec<_>>();
            if detail_parts.len() != 2 {
                return Err(RsIdsError::NotAMediaId(value.to_string()));
            }
            let key = detail_parts[0].to_lowercase();
            let parsed_value: f64 = detail_parts[1]
                .parse()
                .map_err(|_| RsIdsError::NotAMediaId(value.to_string()))?;
            if !parsed_value.is_finite() {
                return Err(RsIdsError::NotAMediaId(value.to_string()));
            }
            let parsed_value = normalize_manga_decimal(parsed_value);
            match key.as_str() {
                "volume" => {
                    if volume.is_some() {
                        return Err(RsIdsError::NotAMediaId(value.to_string()));
                    }
                    volume = Some(parsed_value);
                }
                "chapter" => {
                    if chapter.is_some() {
                        return Err(RsIdsError::NotAMediaId(value.to_string()));
                    }
                    chapter = Some(parsed_value);
                }
                _ => return Err(RsIdsError::NotAMediaId(value.to_string())),
            }
        }

        Ok((volume, chapter))
    }

    fn manga_details_suffix(&self) -> String {
        let mut suffix = String::new();
        if let Some(volume) = self.volume {
            suffix.push_str(&format!("|volume:{}", normalize_manga_decimal(volume)));
        }
        if let Some(chapter) = self.chapter {
            suffix.push_str(&format!("|chapter:{}", normalize_manga_decimal(chapter)));
        }
        suffix
    }

    pub fn try_add(&mut self, value: String) -> Result<(), RsIdsError> {
        if !Self::is_id(&value) {
            return Err(RsIdsError::NotAMediaId(value));
        }
        let pipe_elements = value.split('|').collect::<Vec<_>>();
        let base = pipe_elements.first().ok_or(RsIdsError::InvalidId())?;
        let details = &pipe_elements[1..];
        let elements = base.split(':').collect::<Vec<_>>();
        let source = elements
            .first()
            .ok_or(RsIdsError::InvalidId())?
            .to_lowercase();
        let id = elements.get(1).ok_or(RsIdsError::InvalidId())?;
        let is_manga_source = matches!(
            source.as_str(),
            "anilist"
                | "anilist_manga_id"
                | "mangadex"
                | "mangadex_manga_uuid"
                | "mal"
                | "myanimelist_manga_id"
        );
        if !is_manga_source && !details.is_empty() {
            return Err(RsIdsError::NotAMediaId(value));
        }

        match source.as_str() {
            "redseat" => {
                self.redseat = Some(id.to_string());
                Ok(())
            }
            "imdb" => {
                self.imdb = Some(id.to_string());
                Ok(())
            }
            "trakt" => {
                let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
                self.trakt = Some(id);
                Ok(())
            }
            "tmdb" => {
                let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
                self.tmdb = Some(id);
                Ok(())
            }
            "tvdb" => {
                let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
                self.tvdb = Some(id);
                Ok(())
            }
            "tvrage" => {
                let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
                self.tvrage = Some(id);
                Ok(())
            }
            "isbn13" => {
                self.isbn13 = Some(id.to_string());
                Ok(())
            }
            "oleid" | "openlibrary_edition_id" => {
                self.openlibrary_edition_id = Some(id.to_string());
                Ok(())
            }
            "olwid" | "openlibrary_work_id" => {
                self.openlibrary_work_id = Some(id.to_string());
                Ok(())
            }
            "gbvid" | "google_books_volume_id" => {
                self.google_books_volume_id = Some(id.to_string());
                Ok(())
            }
            "anilist" | "anilist_manga_id" => {
                let (volume, chapter) = Self::parse_manga_details(details, &value)?;
                let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
                self.anilist_manga_id = Some(id);
                self.volume = volume;
                self.chapter = chapter;
                Ok(())
            }
            "mangadex" | "mangadex_manga_uuid" => {
                let (volume, chapter) = Self::parse_manga_details(details, &value)?;
                self.mangadex_manga_uuid = Some(id.to_string());
                self.volume = volume;
                self.chapter = chapter;
                Ok(())
            }
            "mal" | "myanimelist_manga_id" => {
                let (volume, chapter) = Self::parse_manga_details(details, &value)?;
                let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
                self.myanimelist_manga_id = Some(id);
                self.volume = volume;
                self.chapter = chapter;
                Ok(())
            }
            "asin" => {
                self.asin = Some(id.to_string());
                Ok(())
            }
            _ => {
                self.add_other(&source, id);
                Ok(())
            }
        }
    }

    pub fn into_best(self) -> Option<String> {
        self.as_redseat().or(self.into_best_external())
    }

    pub fn into_best_external(self) -> Option<String> {
        self.as_trakt()
            .or(self.as_imdb())
            .or(self.as_tmdb())
            .or(self.as_tvdb())
            .or(self.as_isbn13())
            .or(self.as_openlibrary_edition_id())
            .or(self.as_openlibrary_work_id())
            .or(self.as_google_books_volume_id())
            .or(self.as_anilist_manga_id())
            .or(self.as_mangadex_manga_uuid())
            .or(self.as_myanimelist_manga_id())
            .or(self.as_asin())
    }
    pub fn as_best_external(&self) -> Option<String> {
        self.as_trakt()
            .or(self.as_imdb())
            .or(self.as_tmdb())
            .or(self.as_tvdb())
            .or(self.as_isbn13())
            .or(self.as_openlibrary_work_id())
            .or(self.as_openlibrary_edition_id())
            .or(self.as_google_books_volume_id())
            .or(self.as_anilist_manga_id())
            .or(self.as_mangadex_manga_uuid())
            .or(self.as_myanimelist_manga_id())
            .or(self.as_asin())
    }

    pub fn into_best_external_or_local(self) -> Option<String> {
        self.as_best_external().or(self.as_redseat())
    }

    pub fn from_imdb(imdb: String) -> Self {
        Self {
            imdb: Some(imdb),
            ..Default::default()
        }
    }
    pub fn as_imdb(&self) -> Option<String> {
        self.imdb.as_ref().map(|i| format!("imdb:{}", i))
    }

    pub fn from_trakt(trakt: u64) -> Self {
        Self {
            trakt: Some(trakt),
            ..Default::default()
        }
    }
    pub fn as_trakt(&self) -> Option<String> {
        self.trakt.map(|i| format!("trakt:{}", i))
    }
    pub fn as_id_for_trakt(&self) -> Option<String> {
        if let Some(trakt) = self.trakt {
            Some(trakt.to_string())
        } else {
            self.imdb.as_ref().map(|imdb| imdb.to_string())
        }
    }

    pub fn from_tvdb(tvdb: u64) -> Self {
        Self {
            tvdb: Some(tvdb),
            ..Default::default()
        }
    }
    pub fn as_tvdb(&self) -> Option<String> {
        self.tvdb.map(|i| format!("tvdb:{}", i))
    }
    pub fn try_tvdb(self) -> Result<u64, RsIdsError> {
        self.tvdb
            .ok_or(RsIdsError::NoMediaIdRequired(Box::new(self.clone())))
    }

    pub fn from_tmdb(tmdb: u64) -> Self {
        Self {
            tmdb: Some(tmdb),
            ..Default::default()
        }
    }
    pub fn as_tmdb(&self) -> Option<String> {
        self.tmdb.map(|i| format!("tmdb:{}", i))
    }
    pub fn try_tmdb(self) -> Result<u64, RsIdsError> {
        self.tmdb
            .ok_or(RsIdsError::NoMediaIdRequired(Box::new(self.clone())))
    }

    pub fn from_redseat(redseat: String) -> Self {
        Self {
            redseat: Some(redseat),
            ..Default::default()
        }
    }
    pub fn as_redseat(&self) -> Option<String> {
        self.redseat.as_ref().map(|i| format!("redseat:{}", i))
    }
    pub fn as_isbn13(&self) -> Option<String> {
        self.isbn13.as_ref().map(|i| format!("isbn13:{}", i))
    }
    pub fn as_openlibrary_edition_id(&self) -> Option<String> {
        self.openlibrary_edition_id
            .as_ref()
            .map(|i| format!("oleid:{}", i))
    }
    pub fn as_openlibrary_work_id(&self) -> Option<String> {
        self.openlibrary_work_id
            .as_ref()
            .map(|i| format!("olwid:{}", i))
    }
    pub fn as_google_books_volume_id(&self) -> Option<String> {
        self.google_books_volume_id
            .as_ref()
            .map(|i| format!("gbvid:{}", i))
    }
    pub fn as_anilist_manga_id(&self) -> Option<String> {
        self.anilist_manga_id.map(|i| format!("anilist:{}", i))
    }
    pub fn as_anilist_manga_id_with_details(&self) -> Option<String> {
        self.anilist_manga_id
            .map(|i| format!("anilist:{}{}", i, self.manga_details_suffix()))
    }
    pub fn as_mangadex_manga_uuid(&self) -> Option<String> {
        self.mangadex_manga_uuid
            .as_ref()
            .map(|i| format!("mangadex:{}", i))
    }
    pub fn as_mangadex_manga_uuid_with_details(&self) -> Option<String> {
        self.mangadex_manga_uuid
            .as_ref()
            .map(|i| format!("mangadex:{}{}", i, self.manga_details_suffix()))
    }
    pub fn as_myanimelist_manga_id(&self) -> Option<String> {
        self.myanimelist_manga_id.map(|i| format!("mal:{}", i))
    }
    pub fn as_myanimelist_manga_id_with_details(&self) -> Option<String> {
        self.myanimelist_manga_id
            .map(|i| format!("mal:{}{}", i, self.manga_details_suffix()))
    }
    pub fn as_asin(&self) -> Option<String> {
        self.asin.as_ref().map(|i| format!("asin:{}", i))
    }

    pub fn as_id(&self) -> Result<String, RsIdsError> {
        if let Some(imdb) = &self.imdb {
            Ok(format!("imdb:{}", imdb))
        } else if let Some(trakt) = &self.trakt {
            Ok(format!("trakt:{}", trakt))
        } else if let Some(tmdb) = &self.tmdb {
            Ok(format!("tmdb:{}", tmdb))
        } else if let Some(tvdb) = &self.tvdb {
            Ok(format!("tvdb:{}", tvdb))
        } else {
            Err(RsIdsError::NoMediaIdRequired(Box::new(self.clone())))
        }
    }

    pub fn add_other(&mut self, key: &str, value: &str) {
        if key.trim().is_empty() {
            return;
        }
        self.other_ids
            .get_or_insert_with(OtherIds::default)
            .add(key, value);
    }

    pub fn has_other_key(&self, key: &str) -> bool {
        self.other_ids
            .as_ref()
            .is_some_and(|other_ids| other_ids.has_key(key))
    }

    pub fn get_other(&self, key: &str) -> Option<String> {
        self.other_ids
            .as_ref()
            .and_then(|other_ids| other_ids.get(key))
    }

    pub fn has_other(&self, key: &str, value: &str) -> bool {
        self.other_ids
            .as_ref()
            .is_some_and(|other_ids| other_ids.contains(key, value))
    }

    /// check if the provided id need parsing like "trakt:xxxxx" and is not directly the local id from this server
    pub fn is_id(id: &str) -> bool {
        let base = id.split('|').next().unwrap_or(id);
        base.contains(":") && base.split(':').count() == 2
    }
}

impl TryFrom<Vec<String>> for RsIds {
    type Error = RsIdsError;

    fn try_from(values: Vec<String>) -> Result<Self, RsIdsError> {
        let mut ids = Self::default();
        for value in values {
            ids.try_add(value)?;
        }
        Ok(ids)
    }
}

impl TryFrom<String> for RsIds {
    type Error = RsIdsError;
    fn try_from(value: String) -> Result<Self, RsIdsError> {
        let mut id = RsIds::default();
        id.try_add(value)?;
        Ok(id)
    }
}

impl From<RsIds> for Vec<String> {
    fn from(value: RsIds) -> Self {
        let mut ids = vec![];
        if let Some(id) = value.as_redseat() {
            ids.push(id)
        }
        if let Some(id) = value.as_imdb() {
            ids.push(id)
        }
        if let Some(id) = value.as_tmdb() {
            ids.push(id.to_string())
        }
        if let Some(id) = value.as_trakt() {
            ids.push(id.to_string())
        }
        if let Some(id) = value.as_tvdb() {
            ids.push(id.to_string())
        }
        if let Some(id) = value.as_isbn13() {
            ids.push(id)
        }
        if let Some(id) = value.as_openlibrary_edition_id() {
            ids.push(id)
        }
        if let Some(id) = value.as_openlibrary_work_id() {
            ids.push(id)
        }
        if let Some(id) = value.as_google_books_volume_id() {
            ids.push(id)
        }
        if let Some(id) = value.as_anilist_manga_id_with_details() {
            ids.push(id)
        }
        if let Some(id) = value.as_mangadex_manga_uuid_with_details() {
            ids.push(id)
        }
        if let Some(id) = value.as_myanimelist_manga_id_with_details() {
            ids.push(id)
        }
        if let Some(id) = value.as_asin() {
            ids.push(id)
        }
        if let Some(other_ids) = value.other_ids {
            ids.extend(other_ids.into_vec());
        }
        ids
    }
}

#[cfg(feature = "rusqlite")]
pub mod external_images_rusqlite {
    use rusqlite::{
        types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
        ToSql,
    };

    use super::RsIds;

    impl FromSql for RsIds {
        fn column_result(value: ValueRef) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|as_string| {
                let r = serde_json::from_str(&as_string).map_err(|_| FromSqlError::InvalidType);
                r
            })
        }
    }
    impl ToSql for RsIds {
        fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
            let r = serde_json::to_string(self).map_err(|_| FromSqlError::InvalidType)?;
            Ok(ToSqlOutput::from(r))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_existing_movie_show_ids_regression() -> Result<(), RsIdsError> {
        let parsed: RsIds = "trakt:905982".to_string().try_into()?;
        assert_eq!(parsed.trakt, Some(905982));

        let parsed: RsIds = "imdb:tt1234567".to_string().try_into()?;
        assert_eq!(parsed.imdb, Some("tt1234567".to_string()));

        let parsed: RsIds = "tmdb:42".to_string().try_into()?;
        assert_eq!(parsed.tmdb, Some(42));

        let parsed: RsIds = "tvdb:99".to_string().try_into()?;
        assert_eq!(parsed.tvdb, Some(99));
        assert_eq!(parsed.as_best_external(), Some("tvdb:99".to_string()));
        assert_eq!(parsed.as_id()?, "tvdb:99");

        Ok(())
    }

    #[test]
    fn test_parse_short_prefixes() -> Result<(), RsIdsError> {
        let mut ids = RsIds::default();
        ids.try_add("isbn13:9780143127741".to_string())?;
        ids.try_add("oleid:OL12345M".to_string())?;
        ids.try_add("olwid:OL6789W".to_string())?;
        ids.try_add("gbvid:abcDEF_123".to_string())?;
        ids.try_add("anilist:123".to_string())?;
        ids.try_add("mangadex:7f2f8cdd-b241-4f27-a6fe-13f7f7fb9164".to_string())?;
        ids.try_add("mal:456".to_string())?;
        ids.try_add("asin:B08XYZ1234".to_string())?;

        assert_eq!(ids.isbn13.as_deref(), Some("9780143127741"));
        assert_eq!(ids.openlibrary_edition_id.as_deref(), Some("OL12345M"));
        assert_eq!(ids.openlibrary_work_id.as_deref(), Some("OL6789W"));
        assert_eq!(ids.google_books_volume_id.as_deref(), Some("abcDEF_123"));
        assert_eq!(ids.anilist_manga_id, Some(123));
        assert_eq!(
            ids.mangadex_manga_uuid.as_deref(),
            Some("7f2f8cdd-b241-4f27-a6fe-13f7f7fb9164")
        );
        assert_eq!(ids.myanimelist_manga_id, Some(456));
        assert_eq!(ids.asin.as_deref(), Some("B08XYZ1234"));
        Ok(())
    }

    #[test]
    fn test_parse_manga_pipe_details() -> Result<(), RsIdsError> {
        let mut ids = RsIds::default();
        ids.try_add("anilist:123|volume:1|chapter:2.5".to_string())?;
        assert_eq!(ids.anilist_manga_id, Some(123));
        assert_eq!(ids.volume, Some(1.0));
        assert_eq!(ids.chapter, Some(2.5));

        ids.try_add("mal:456|chapter:10.5".to_string())?;
        assert_eq!(ids.myanimelist_manga_id, Some(456));
        assert_eq!(ids.volume, None);
        assert_eq!(ids.chapter, Some(10.5));

        ids.try_add("mangadex:uuid-1|volume:3".to_string())?;
        assert_eq!(ids.mangadex_manga_uuid.as_deref(), Some("uuid-1"));
        assert_eq!(ids.volume, Some(3.0));
        assert_eq!(ids.chapter, None);
        Ok(())
    }

    #[test]
    fn test_parse_long_aliases() -> Result<(), RsIdsError> {
        let mut ids = RsIds::default();
        ids.try_add("openlibrary_edition_id:OL1M".to_string())?;
        ids.try_add("openlibrary_work_id:OL2W".to_string())?;
        ids.try_add("google_books_volume_id:vol123".to_string())?;
        ids.try_add("anilist_manga_id:111".to_string())?;
        ids.try_add("mangadex_manga_uuid:uuid-1".to_string())?;
        ids.try_add("myanimelist_manga_id:222".to_string())?;

        assert_eq!(ids.openlibrary_edition_id.as_deref(), Some("OL1M"));
        assert_eq!(ids.openlibrary_work_id.as_deref(), Some("OL2W"));
        assert_eq!(ids.google_books_volume_id.as_deref(), Some("vol123"));
        assert_eq!(ids.anilist_manga_id, Some(111));
        assert_eq!(ids.mangadex_manga_uuid.as_deref(), Some("uuid-1"));
        assert_eq!(ids.myanimelist_manga_id, Some(222));
        Ok(())
    }

    #[test]
    fn test_case_insensitive_parsing() -> Result<(), RsIdsError> {
        let mut ids = RsIds::default();
        ids.try_add("AnIlIsT:55".to_string())?;
        ids.try_add("MAL:77".to_string())?;
        ids.try_add("OLEID:OLX".to_string())?;
        ids.try_add("GBVID:gbx".to_string())?;

        assert_eq!(ids.anilist_manga_id, Some(55));
        assert_eq!(ids.myanimelist_manga_id, Some(77));
        assert_eq!(ids.openlibrary_edition_id.as_deref(), Some("OLX"));
        assert_eq!(ids.google_books_volume_id.as_deref(), Some("gbx"));
        Ok(())
    }

    #[test]
    fn test_unknown_source_is_stored_as_other_id() -> Result<(), RsIdsError> {
        let mut ids = RsIds::default();
        ids.try_add("AniDb:1234".to_string())?;
        assert!(ids.has_other_key("anidb"));
        assert_eq!(ids.get_other("ANIDB"), Some("1234".to_string()));
        assert!(ids.has_other("anidb", "1234"));
        Ok(())
    }

    #[test]
    fn test_add_other_replaces_existing_key_value() {
        let mut ids = RsIds::default();
        ids.add_other("custom", "first");
        ids.add_other("CUSTOM", "second");
        assert_eq!(ids.get_other("custom"), Some("second".to_string()));
        assert_eq!(
            ids.other_ids,
            Some(OtherIds(vec!["custom:second".to_string()]))
        );
    }

    #[test]
    fn test_manga_with_details_methods_keep_base_as_methods() {
        let ids = RsIds {
            anilist_manga_id: Some(123),
            myanimelist_manga_id: Some(456),
            mangadex_manga_uuid: Some("uuid-2".to_string()),
            volume: Some(1.0),
            chapter: Some(2.0),
            ..Default::default()
        };

        assert_eq!(ids.as_anilist_manga_id(), Some("anilist:123".to_string()));
        assert_eq!(ids.as_myanimelist_manga_id(), Some("mal:456".to_string()));
        assert_eq!(
            ids.as_mangadex_manga_uuid(),
            Some("mangadex:uuid-2".to_string())
        );
        assert_eq!(
            ids.as_anilist_manga_id_with_details(),
            Some("anilist:123|volume:1|chapter:2".to_string())
        );
        assert_eq!(
            ids.as_myanimelist_manga_id_with_details(),
            Some("mal:456|volume:1|chapter:2".to_string())
        );
        assert_eq!(
            ids.as_mangadex_manga_uuid_with_details(),
            Some("mangadex:uuid-2|volume:1|chapter:2".to_string())
        );
    }

    #[test]
    fn test_numeric_parse_failure_for_anilist_and_mal() {
        let mut ids = RsIds::default();
        assert!(matches!(
            ids.try_add("anilist:not-a-number".to_string()),
            Err(RsIdsError::NotAMediaId(_))
        ));
        assert!(matches!(
            ids.try_add("myanimelist_manga_id:bad".to_string()),
            Err(RsIdsError::NotAMediaId(_))
        ));
    }

    #[test]
    fn test_parse_failure_for_invalid_manga_pipe_details() {
        let mut ids = RsIds::default();
        assert!(matches!(
            ids.try_add("anilist:123|volume".to_string()),
            Err(RsIdsError::NotAMediaId(_))
        ));
        assert!(matches!(
            ids.try_add("anilist:123|arc:1".to_string()),
            Err(RsIdsError::NotAMediaId(_))
        ));
        assert!(matches!(
            ids.try_add("mal:456|chapter:abc".to_string()),
            Err(RsIdsError::NotAMediaId(_))
        ));
        assert!(matches!(
            ids.try_add("mangadex:uuid|chapter:1|chapter:2".to_string()),
            Err(RsIdsError::NotAMediaId(_))
        ));
    }

    #[test]
    fn test_parse_failure_for_non_manga_pipe_details() {
        let mut ids = RsIds::default();
        assert!(matches!(
            ids.try_add("imdb:tt1234567|chapter:1".to_string()),
            Err(RsIdsError::NotAMediaId(_))
        ));
    }

    #[test]
    fn test_roundtrip_vec_rsids_vec_uses_canonical_prefixes() -> Result<(), RsIdsError> {
        let input = vec![
            "openlibrary_edition_id:OL3M".to_string(),
            "openlibrary_work_id:OL4W".to_string(),
            "google_books_volume_id:vol-3".to_string(),
            "anilist_manga_id:999".to_string(),
            "mangadex_manga_uuid:uuid-3".to_string(),
            "myanimelist_manga_id:1111".to_string(),
            "isbn13:9780316769488".to_string(),
            "asin:B012345678".to_string(),
        ];
        let ids = RsIds::try_from(input)?;
        let output: Vec<String> = ids.into();

        assert!(output.contains(&"oleid:OL3M".to_string()));
        assert!(output.contains(&"olwid:OL4W".to_string()));
        assert!(output.contains(&"gbvid:vol-3".to_string()));
        assert!(output.contains(&"anilist:999".to_string()));
        assert!(output.contains(&"mangadex:uuid-3".to_string()));
        assert!(output.contains(&"mal:1111".to_string()));
        assert!(output.contains(&"isbn13:9780316769488".to_string()));
        assert!(output.contains(&"asin:B012345678".to_string()));
        Ok(())
    }

    #[test]
    fn test_roundtrip_vec_rsids_vec_uses_pipe_format_for_manga_details() -> Result<(), RsIdsError> {
        let input = vec!["anilist:999|chapter:2|volume:1".to_string()];
        let ids = RsIds::try_from(input)?;
        let output: Vec<String> = ids.into();

        assert!(output.contains(&"anilist:999|volume:1|chapter:2".to_string()));
        Ok(())
    }

    #[test]
    fn test_roundtrip_vec_rsids_vec_preserves_other_ids() -> Result<(), RsIdsError> {
        let input = vec![
            "foo:1".to_string(),
            "bar:value-2".to_string(),
            "imdb:tt1234567".to_string(),
        ];
        let ids = RsIds::try_from(input)?;
        let output: Vec<String> = ids.into();

        assert!(output.contains(&"foo:1".to_string()));
        assert!(output.contains(&"bar:value-2".to_string()));
        assert!(output.contains(&"imdb:tt1234567".to_string()));
        Ok(())
    }

    #[test]
    fn test_best_external_selection_for_book_ids_only() {
        let ids = RsIds {
            isbn13: Some("9780131103627".to_string()),
            openlibrary_edition_id: Some("OL5M".to_string()),
            openlibrary_work_id: Some("OL6W".to_string()),
            google_books_volume_id: Some("vol-5".to_string()),
            anilist_manga_id: Some(12),
            mangadex_manga_uuid: Some("uuid-5".to_string()),
            myanimelist_manga_id: Some(34),
            asin: Some("B00TEST000".to_string()),
            ..Default::default()
        };
        assert_eq!(
            ids.as_best_external(),
            Some("isbn13:9780131103627".to_string())
        );

        let ids = RsIds {
            openlibrary_edition_id: Some("OL5M".to_string()),
            openlibrary_work_id: Some("OL6W".to_string()),
            ..Default::default()
        };
        assert_eq!(ids.as_best_external(), Some("oleid:OL5M".to_string()));

        let ids = RsIds {
            anilist_manga_id: Some(12),
            mangadex_manga_uuid: Some("uuid-5".to_string()),
            myanimelist_manga_id: Some(34),
            asin: Some("B00TEST000".to_string()),
            ..Default::default()
        };
        assert_eq!(ids.as_best_external(), Some("anilist:12".to_string()));
    }

    #[cfg(feature = "rusqlite")]
    #[test]
    fn test_rusqlite_roundtrip_rsids_with_other_ids() -> rusqlite::Result<()> {
        use rusqlite::Connection;

        let conn = Connection::open_in_memory()?;
        conn.execute("CREATE TABLE test_rsids (ids TEXT NOT NULL)", [])?;

        let mut ids = RsIds::default();
        ids.add_other("foo", "42");
        ids.add_other("bar", "abc");
        conn.execute("INSERT INTO test_rsids (ids) VALUES (?1)", [&ids])?;

        let loaded: RsIds =
            conn.query_row("SELECT ids FROM test_rsids LIMIT 1", [], |row| row.get(0))?;

        assert!(loaded.has_other("foo", "42"));
        assert!(loaded.has_other("bar", "abc"));
        Ok(())
    }
}
