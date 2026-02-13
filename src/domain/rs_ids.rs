use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, strum_macros::AsRefStr)]
pub enum RsIdsError {
    InvalidId(),
	NotAMediaId(String),
    NoMediaIdRequired(Box<RsIds>)

}

// region:    --- Error Boilerplate

impl core::fmt::Display for RsIdsError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for RsIdsError {}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Default)]
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
    pub other_ids: Option<Vec<String>>,
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
    pub asin: Option<String>,
}



impl RsIds {
    pub fn try_add(&mut self, value: String) -> Result<(), RsIdsError> {
        if !Self::is_id(&value) {
            return Err(RsIdsError::NotAMediaId(value))
        }
        let elements = value.split(":").collect::<Vec<_>>();
        let source = elements.first().ok_or(RsIdsError::InvalidId())?.to_lowercase();
        let id = elements.get(1).ok_or(RsIdsError::InvalidId())?;

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
                let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
                self.anilist_manga_id = Some(id);
                Ok(())
            }
            "mangadex" | "mangadex_manga_uuid" => {
                self.mangadex_manga_uuid = Some(id.to_string());
                Ok(())
            }
            "mal" | "myanimelist_manga_id" => {
                let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
                self.myanimelist_manga_id = Some(id);
                Ok(())
            }
            "asin" => {
                self.asin = Some(id.to_string());
                Ok(())
            }
            _ => Err(RsIdsError::NotAMediaId(value))
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
            .or(self.as_openlibrary_edition_id())
            .or(self.as_openlibrary_work_id())
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
        } else { self.imdb.as_ref().map(|imdb| imdb.to_string()) }
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
        self.tvdb.ok_or(RsIdsError::NoMediaIdRequired(Box::new(self.clone())))
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
        self.tmdb.ok_or(RsIdsError::NoMediaIdRequired(Box::new(self.clone())))
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
        self.openlibrary_edition_id.as_ref().map(|i| format!("oleid:{}", i))
    }
    pub fn as_openlibrary_work_id(&self) -> Option<String> {
        self.openlibrary_work_id.as_ref().map(|i| format!("olwid:{}", i))
    }
    pub fn as_google_books_volume_id(&self) -> Option<String> {
        self.google_books_volume_id.as_ref().map(|i| format!("gbvid:{}", i))
    }
    pub fn as_anilist_manga_id(&self) -> Option<String> {
        self.anilist_manga_id.map(|i| format!("anilist:{}", i))
    }
    pub fn as_mangadex_manga_uuid(&self) -> Option<String> {
        self.mangadex_manga_uuid.as_ref().map(|i| format!("mangadex:{}", i))
    }
    pub fn as_myanimelist_manga_id(&self) -> Option<String> {
        self.myanimelist_manga_id.map(|i| format!("mal:{}", i))
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

    /// check if the provided id need parsing like "trakt:xxxxx" and is not directly the local id from this server
    pub fn is_id(id: &str) -> bool {
        id.contains(":") && id.split(":").count() == 2
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
        if let Some(id) = value.as_anilist_manga_id() {
            ids.push(id)
        }
        if let Some(id) = value.as_mangadex_manga_uuid() {
            ids.push(id)
        }
        if let Some(id) = value.as_myanimelist_manga_id() {
            ids.push(id)
        }
        if let Some(id) = value.as_asin() {
            ids.push(id)
        }
        ids
    }
}


#[cfg(feature = "rusqlite")]
pub mod external_images_rusqlite {
    use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, ToSql};

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
        assert_eq!(ids.as_best_external(), Some("isbn13:9780131103627".to_string()));

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
}
