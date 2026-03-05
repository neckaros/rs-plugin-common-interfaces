use std::collections::BTreeMap;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};

use crate::domain::other_ids::OtherIds;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
pub enum RsIdsError {
    InvalidId(),
    NotAMediaId(String),
    NoMediaIdRequired(Box<RsIds>),
    InvalidBase64Ids(String),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for RsIdsError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for RsIdsError {}

// endregion: --- Error Boilerplate

pub trait ApplyRsIds {
    fn apply_rs_ids(&mut self, ids: &RsIds);
}

/// Keys that store numeric (u64) values and should be serialized as numbers in JSON.
const NUMERIC_KEYS: &[&str] = &["trakt", "tmdb", "tvdb", "tvrage", "anilist", "mal"];

/// Priority order for selecting the "best" external ID.
const EXTERNAL_PRIORITY: &[&str] = &[
    "trakt", "imdb", "tmdb", "tvdb", "isbn13", "oleid", "olwid", "gbvid", "anilist", "mangadex",
    "mal", "asin",
];

/// Alias mappings: (alias → canonical key).
/// Used to normalize long-form and camelCase names to short canonical names.
const KEY_ALIASES: &[(&str, &str)] = &[
    // snake_case long form
    ("openlibrary_edition_id", "oleid"),
    ("openlibrary_work_id", "olwid"),
    ("google_books_volume_id", "gbvid"),
    ("anilist_manga_id", "anilist"),
    ("mangadex_manga_uuid", "mangadex"),
    ("myanimelist_manga_id", "mal"),
    // camelCase (from old JSON format)
    ("openlibraryeditionid", "oleid"),
    ("openlibraryworkid", "olwid"),
    ("googlebooksvolumeid", "gbvid"),
    ("anilistmangaid", "anilist"),
    ("mangadexmangauuid", "mangadex"),
    ("myanimelistmangaid", "mal"),
];

/// A collection of media IDs stored as a flat key-value map.
///
/// All values are stored as strings internally. Typed accessors (`trakt()`, `tmdb()`, etc.)
/// parse on access. Keys are case-insensitive and long-form aliases are normalized to
/// canonical short names (e.g., `openlibrary_edition_id` → `oleid`).
///
/// Values may contain pipe-separated details for extra metadata:
/// `"123|volume:1|chapter:2.5"`. Use `split_details()` to parse these.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct RsIds(pub BTreeMap<String, String>);

macro_rules! str_accessor {
    ($name:ident, $key:expr) => {
        pub fn $name(&self) -> Option<&str> {
            self.get($key)
        }
    };
}

macro_rules! u64_accessor {
    ($name:ident, $key:expr) => {
        pub fn $name(&self) -> Option<u64> {
            self.get_u64($key)
        }
    };
}

macro_rules! from_str_factory {
    ($name:ident, $key:expr) => {
        pub fn $name(value: String) -> Self {
            let mut ids = Self::default();
            ids.set($key, value);
            ids
        }
    };
}

macro_rules! from_u64_factory {
    ($name:ident, $key:expr) => {
        pub fn $name(value: u64) -> Self {
            let mut ids = Self::default();
            ids.set($key, value);
            ids
        }
    };
}

impl RsIds {
    /// Normalize a key to its canonical lowercase form, resolving aliases.
    fn canonicalize_key(key: &str) -> String {
        let lower = key.to_ascii_lowercase();
        for &(alias, canonical) in KEY_ALIASES {
            if lower == alias {
                return canonical.to_string();
            }
        }
        lower
    }

    // -- Core accessors --

    /// Get the raw string value for a key. Returns the full value including any pipe details.
    pub fn get(&self, key: &str) -> Option<&str> {
        let canonical = Self::canonicalize_key(key);
        self.0.get(&canonical).map(|s| s.as_str())
    }

    /// Get a value parsed as u64. Only parses the base value (before any `|` pipe separator).
    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.get(key).and_then(|v| {
            let (base, _) = Self::split_details(v);
            base.parse().ok()
        })
    }

    /// Get a value parsed as f64. Only parses the base value (before any `|` pipe separator).
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(|v| {
            let (base, _) = Self::split_details(v);
            base.parse().ok()
        })
    }

    /// Set a key-value pair. The key is canonicalized.
    pub fn set(&mut self, key: &str, value: impl ToString) {
        let canonical = Self::canonicalize_key(key);
        if canonical.is_empty() {
            return;
        }
        self.0.insert(canonical, value.to_string());
    }

    /// Check if a key exists in the map.
    pub fn has(&self, key: &str) -> bool {
        let canonical = Self::canonicalize_key(key);
        self.0.contains_key(&canonical)
    }

    /// Remove a key and return its value.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        let canonical = Self::canonicalize_key(key);
        self.0.remove(&canonical)
    }

    /// Iterate over all key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.0.iter()
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // -- Convenience typed accessors --

    str_accessor!(imdb, "imdb");
    str_accessor!(slug, "slug");
    str_accessor!(redseat, "redseat");
    str_accessor!(isbn13, "isbn13");
    str_accessor!(asin, "asin");
    str_accessor!(openlibrary_edition_id, "oleid");
    str_accessor!(openlibrary_work_id, "olwid");
    str_accessor!(google_books_volume_id, "gbvid");
    str_accessor!(mangadex_manga_uuid, "mangadex");
    u64_accessor!(trakt, "trakt");
    u64_accessor!(tmdb, "tmdb");
    u64_accessor!(tvdb, "tvdb");
    u64_accessor!(tvrage, "tvrage");
    u64_accessor!(anilist_manga_id, "anilist");
    u64_accessor!(myanimelist_manga_id, "mal");

    // -- Factory methods --

    from_str_factory!(from_imdb, "imdb");
    from_str_factory!(from_redseat, "redseat");
    from_u64_factory!(from_trakt, "trakt");
    from_u64_factory!(from_tvdb, "tvdb");
    from_u64_factory!(from_tmdb, "tmdb");

    // -- Parsing --

    /// Parse a `"key:value"` or `"key:value|detail:val"` string and add it to the map.
    /// The key is case-insensitive and aliases are resolved.
    /// No type validation is performed — typed accessors return `None` for unparseable values.
    pub fn try_add(&mut self, value: String) -> Result<(), RsIdsError> {
        if !Self::is_id(&value) {
            return Err(RsIdsError::NotAMediaId(value));
        }
        let base = value.split('|').next().ok_or(RsIdsError::InvalidId())?;
        let (source_raw, id_value) = base.split_once(':').ok_or(RsIdsError::InvalidId())?;
        let canonical_key = Self::canonicalize_key(source_raw);

        // Store the value. If there are pipe details, include them.
        let value_to_store = if let Some(pipe_start) = value.find('|') {
            format!("{}{}", id_value, &value[pipe_start..])
        } else {
            id_value.to_string()
        };

        self.0.insert(canonical_key, value_to_store);
        Ok(())
    }

    /// Check if a string looks like an external ID (`"key:value"` format).
    pub fn is_id(id: &str) -> bool {
        let base = id.split('|').next().unwrap_or(id);
        base.contains(':') && base.split(':').count() == 2
    }

    // -- Pipe detail helpers --

    /// Split a value into its base and pipe-separated detail pairs.
    ///
    /// Consumers should use the pipe separator `|` to attach extra metadata to their IDs.
    ///
    /// # Example
    /// ```
    /// use rs_plugin_common_interfaces::domain::rs_ids::RsIds;
    /// let (base, details) = RsIds::split_details("123|volume:1|chapter:2.5");
    /// assert_eq!(base, "123");
    /// assert_eq!(details, vec![("volume", "1"), ("chapter", "2.5")]);
    /// ```
    pub fn split_details(value: &str) -> (&str, Vec<(&str, &str)>) {
        let mut parts = value.split('|');
        let base = parts.next().unwrap_or(value);
        let details = parts.filter_map(|part| part.split_once(':')).collect();
        (base, details)
    }

    /// Search all values for a pipe-separated detail with the given key.
    /// Also checks top-level entries as a fallback (for backward compatibility).
    /// Returns the first match found.
    pub fn find_detail<'a>(&'a self, detail_key: &str) -> Option<&'a str> {
        // First check top-level entries (backward compat: old JSON had volume/chapter as fields)
        if let Some(v) = self.get(detail_key) {
            return Some(v);
        }
        // Then search pipe details in all values
        let lower_key = detail_key.to_ascii_lowercase();
        for value in self.0.values() {
            let (_, details) = Self::split_details(value);
            for (k, _v) in &details {
                if k.to_ascii_lowercase() == lower_key {
                    // Re-split to get the reference with the right lifetime
                    // (details borrows from value which borrows from self)
                    let (_, details2) = Self::split_details(value);
                    for (k2, v2) in details2 {
                        if k2.to_ascii_lowercase() == lower_key {
                            return Some(v2);
                        }
                    }
                }
            }
        }
        None
    }

    /// Like `find_detail`, but parses the result as f64.
    pub fn find_detail_f64(&self, detail_key: &str) -> Option<f64> {
        self.find_detail(detail_key).and_then(|v| v.parse().ok())
    }

    // -- String formatters --

    /// Format a single entry as `"key:value"`.
    pub fn as_string(&self, key: &str) -> Option<String> {
        let canonical = Self::canonicalize_key(key);
        self.0
            .get(&canonical)
            .map(|v| format!("{}:{}", canonical, v))
    }

    // -- Selection --

    /// Return the best external ID as `"key:value"`, using a priority order.
    /// Falls back to the first non-redseat entry if none of the priority keys exist.
    pub fn as_best_external(&self) -> Option<String> {
        for key in EXTERNAL_PRIORITY {
            if let Some(formatted) = self.as_string(key) {
                return Some(formatted);
            }
        }
        // Fall back to first non-redseat, non-priority entry
        self.0
            .iter()
            .find(|(k, _)| {
                k.as_str() != "redseat" && !EXTERNAL_PRIORITY.contains(&k.as_str())
            })
            .map(|(k, v)| format!("{}:{}", k, v))
    }

    /// Consuming version of `as_best_external`.
    pub fn into_best_external(self) -> Option<String> {
        self.as_best_external()
    }

    /// Return the redseat ID if present, otherwise the best external.
    pub fn into_best(self) -> Option<String> {
        self.as_string("redseat").or_else(|| self.as_best_external())
    }

    /// Return the best external ID, falling back to redseat.
    pub fn into_best_external_or_local(self) -> Option<String> {
        self.as_best_external()
            .or_else(|| self.as_string("redseat"))
    }

    /// Return all external IDs (everything except redseat) as `"key:value"` strings.
    pub fn as_all_external_ids(&self) -> Vec<String> {
        self.0
            .iter()
            .filter(|(k, _)| k.as_str() != "redseat")
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect()
    }

    /// Return all IDs as `"key:value"` strings.
    pub fn as_all_ids(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect()
    }

    /// Return all IDs wrapped in an `OtherIds`.
    pub fn as_all_other_ids(&self) -> OtherIds {
        OtherIds(self.as_all_ids())
    }

    // -- Base64url encoding --

    /// Serialize to `ids:<base64url(json)>` format for use in URL path parameters.
    /// Encodes all IDs as a single opaque string that can be passed in place of `source:value`.
    pub fn to_url_id(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        let encoded = URL_SAFE_NO_PAD.encode(json.as_bytes());
        format!("ids:{}", encoded)
    }

    /// Decode from a base64url-encoded JSON string (without the `ids:` prefix).
    pub fn from_base64url(encoded: &str) -> Result<Self, RsIdsError> {
        let bytes = URL_SAFE_NO_PAD
            .decode(encoded)
            .map_err(|_| RsIdsError::InvalidBase64Ids(encoded.to_string()))?;
        let json_str = std::str::from_utf8(&bytes)
            .map_err(|_| RsIdsError::InvalidBase64Ids(encoded.to_string()))?;
        serde_json::from_str(json_str)
            .map_err(|_| RsIdsError::InvalidBase64Ids(encoded.to_string()))
    }

    /// Return the first available TV/movie ID (imdb → trakt → tmdb → tvdb) or error.
    pub fn as_id(&self) -> Result<String, RsIdsError> {
        for key in &["imdb", "trakt", "tmdb", "tvdb"] {
            if let Some(formatted) = self.as_string(key) {
                return Ok(formatted);
            }
        }
        Err(RsIdsError::NoMediaIdRequired(Box::new(self.clone())))
    }

    /// Try to get tvdb as u64, or error.
    pub fn try_tvdb(&self) -> Result<u64, RsIdsError> {
        self.tvdb()
            .ok_or_else(|| RsIdsError::NoMediaIdRequired(Box::new(self.clone())))
    }

    /// Try to get tmdb as u64, or error.
    pub fn try_tmdb(&self) -> Result<u64, RsIdsError> {
        self.tmdb()
            .ok_or_else(|| RsIdsError::NoMediaIdRequired(Box::new(self.clone())))
    }

    // -- Matching & merging --

    /// Check if two instances share at least one common ID (same key AND same value).
    /// Uses a merge-join on the sorted iterators for O(n + m) with zero allocations.
    pub fn has_common_id(&self, other: &RsIds) -> bool {
        let mut a = self.0.iter();
        let mut b = other.0.iter();
        let mut pair_a = a.next();
        let mut pair_b = b.next();
        while let (Some((ka, va)), Some((kb, vb))) = (pair_a, pair_b) {
            match ka.cmp(kb) {
                std::cmp::Ordering::Less => pair_a = a.next(),
                std::cmp::Ordering::Greater => pair_b = b.next(),
                std::cmp::Ordering::Equal => {
                    if va == vb {
                        return true;
                    }
                    pair_a = a.next();
                    pair_b = b.next();
                }
            }
        }
        false
    }

    /// Merge entries from `other` into `self`. Existing entries in `self` are preserved
    /// (self takes priority on key conflicts).
    pub fn merge(&mut self, other: &RsIds) {
        for (k, v) in &other.0 {
            self.0.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }

    /// Apply these IDs to a target that implements `ApplyRsIds`.
    pub fn apply_to<T: ApplyRsIds>(&self, target: &mut T) {
        target.apply_rs_ids(self);
    }
}

// -- Serde --

impl Serialize for RsIds {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (key, value) in &self.0 {
            // Serialize known numeric keys as numbers when parseable
            if NUMERIC_KEYS.contains(&key.as_str()) {
                if let Ok(num) = value.parse::<u64>() {
                    map.serialize_entry(key, &num)?;
                    continue;
                }
            }
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for RsIds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RsIdsDeVisitor)
    }
}

struct RsIdsDeVisitor;

impl<'de> serde::de::Visitor<'de> for RsIdsDeVisitor {
    type Value = RsIds;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a map of ID key-value pairs")
    }

    fn visit_map<M>(self, mut map: M) -> Result<RsIds, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let mut ids = RsIds::default();
        while let Some(key) = map.next_key::<String>()? {
            let canonical = RsIds::canonicalize_key(&key);

            // Handle old format: otherIds / other_ids was a JSON array ["foo:1", "bar:2"]
            if canonical == "other_ids" || canonical == "otherids" {
                if let Ok(entries) = map.next_value::<Vec<String>>() {
                    for entry in entries {
                        let _ = ids.try_add(entry);
                    }
                }
                continue;
            }

            // Accept string, number, float, or bool values — convert to string
            let value: serde_json::Value = map.next_value()?;
            let string_value = match value {
                serde_json::Value::String(s) => s,
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Null => continue,
                _ => continue,
            };

            ids.0.insert(canonical, string_value);
        }
        Ok(ids)
    }
}

// -- TryFrom / From conversions --

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

impl TryFrom<OtherIds> for RsIds {
    type Error = RsIdsError;

    fn try_from(value: OtherIds) -> Result<Self, RsIdsError> {
        Self::try_from(value.into_vec())
    }
}

impl TryFrom<String> for RsIds {
    type Error = RsIdsError;
    fn try_from(value: String) -> Result<Self, RsIdsError> {
        // Base64url-encoded multi-ID format: "ids:<base64url(json)>"
        if let Some(encoded) = value.strip_prefix("ids:") {
            return Self::from_base64url(encoded);
        }
        let mut ids = RsIds::default();
        ids.try_add(value)?;
        Ok(ids)
    }
}

impl From<RsIds> for Vec<String> {
    fn from(value: RsIds) -> Self {
        value.as_all_ids()
    }
}

// -- rusqlite support --

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
                serde_json::from_str(&as_string).map_err(|_| FromSqlError::InvalidType)
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

// -- Tests --

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_existing_movie_show_ids_regression() -> Result<(), RsIdsError> {
        let parsed: RsIds = "trakt:905982".to_string().try_into()?;
        assert_eq!(parsed.trakt(), Some(905982));

        let parsed: RsIds = "imdb:tt1234567".to_string().try_into()?;
        assert_eq!(parsed.imdb(), Some("tt1234567"));

        let parsed: RsIds = "tmdb:42".to_string().try_into()?;
        assert_eq!(parsed.tmdb(), Some(42));

        let parsed: RsIds = "tvdb:99".to_string().try_into()?;
        assert_eq!(parsed.tvdb(), Some(99));
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

        assert_eq!(ids.isbn13(), Some("9780143127741"));
        assert_eq!(ids.openlibrary_edition_id(), Some("OL12345M"));
        assert_eq!(ids.openlibrary_work_id(), Some("OL6789W"));
        assert_eq!(ids.google_books_volume_id(), Some("abcDEF_123"));
        assert_eq!(ids.anilist_manga_id(), Some(123));
        assert_eq!(
            ids.mangadex_manga_uuid(),
            Some("7f2f8cdd-b241-4f27-a6fe-13f7f7fb9164")
        );
        assert_eq!(ids.myanimelist_manga_id(), Some(456));
        assert_eq!(ids.asin(), Some("B08XYZ1234"));
        Ok(())
    }

    #[test]
    fn test_parse_pipe_details_generic() -> Result<(), RsIdsError> {
        let mut ids = RsIds::default();
        ids.try_add("anilist:123|volume:1|chapter:2.5".to_string())?;
        assert_eq!(ids.anilist_manga_id(), Some(123));
        assert_eq!(ids.find_detail_f64("volume"), Some(1.0));
        assert_eq!(ids.find_detail_f64("chapter"), Some(2.5));

        // Pipe details work on any key
        ids.try_add("custom:abc|extra:42".to_string())?;
        assert_eq!(ids.get("custom"), Some("abc|extra:42"));
        let (base, details) = RsIds::split_details(ids.get("custom").unwrap());
        assert_eq!(base, "abc");
        assert_eq!(details, vec![("extra", "42")]);

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

        assert_eq!(ids.openlibrary_edition_id(), Some("OL1M"));
        assert_eq!(ids.openlibrary_work_id(), Some("OL2W"));
        assert_eq!(ids.google_books_volume_id(), Some("vol123"));
        assert_eq!(ids.anilist_manga_id(), Some(111));
        assert_eq!(ids.mangadex_manga_uuid(), Some("uuid-1"));
        assert_eq!(ids.myanimelist_manga_id(), Some(222));
        Ok(())
    }

    #[test]
    fn test_case_insensitive_parsing() -> Result<(), RsIdsError> {
        let mut ids = RsIds::default();
        ids.try_add("AnIlIsT:55".to_string())?;
        ids.try_add("MAL:77".to_string())?;
        ids.try_add("OLEID:OLX".to_string())?;
        ids.try_add("GBVID:gbx".to_string())?;

        assert_eq!(ids.anilist_manga_id(), Some(55));
        assert_eq!(ids.myanimelist_manga_id(), Some(77));
        assert_eq!(ids.openlibrary_edition_id(), Some("OLX"));
        assert_eq!(ids.google_books_volume_id(), Some("gbx"));
        Ok(())
    }

    #[test]
    fn test_unknown_source_stored_in_map() -> Result<(), RsIdsError> {
        let mut ids = RsIds::default();
        ids.try_add("AniDb:1234".to_string())?;
        assert!(ids.has("anidb"));
        assert_eq!(ids.get("ANIDB"), Some("1234"));
        Ok(())
    }

    #[test]
    fn test_set_replaces_existing_key_value() {
        let mut ids = RsIds::default();
        ids.set("custom", "first");
        ids.set("CUSTOM", "second");
        assert_eq!(ids.get("custom"), Some("second"));
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
    fn test_roundtrip_vec_rsids_vec_preserves_pipe_details() -> Result<(), RsIdsError> {
        let input = vec!["anilist:999|chapter:2|volume:1".to_string()];
        let ids = RsIds::try_from(input)?;
        let output: Vec<String> = ids.into();

        // Pipe details are preserved as-is (order from input)
        assert!(output.contains(&"anilist:999|chapter:2|volume:1".to_string()));
        Ok(())
    }

    #[test]
    fn test_roundtrip_vec_rsids_vec_preserves_unknown_ids() -> Result<(), RsIdsError> {
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
    fn test_as_all_ids_returns_all_set_ids() {
        let mut ids = RsIds::default();
        ids.set("redseat", "rs-1");
        ids.set("imdb", "tt1234567");
        ids.set("custom", "abc");
        ids.set("foo", "bar");

        let all = ids.as_all_ids();
        assert!(all.contains(&"redseat:rs-1".to_string()));
        assert!(all.contains(&"imdb:tt1234567".to_string()));
        assert!(all.contains(&"custom:abc".to_string()));
        assert!(all.contains(&"foo:bar".to_string()));
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn test_best_external_selection_for_book_ids_only() {
        let mut ids = RsIds::default();
        ids.set("isbn13", "9780131103627");
        ids.set("oleid", "OL5M");
        ids.set("olwid", "OL6W");
        ids.set("gbvid", "vol-5");
        ids.set("anilist", "12");
        ids.set("mangadex", "uuid-5");
        ids.set("mal", "34");
        ids.set("asin", "B00TEST000");

        assert_eq!(
            ids.as_best_external(),
            Some("isbn13:9780131103627".to_string())
        );

        let mut ids = RsIds::default();
        ids.set("oleid", "OL5M");
        ids.set("olwid", "OL6W");
        assert_eq!(ids.as_best_external(), Some("oleid:OL5M".to_string()));

        let mut ids = RsIds::default();
        ids.set("anilist", "12");
        ids.set("mangadex", "uuid-5");
        ids.set("mal", "34");
        ids.set("asin", "B00TEST000");
        assert_eq!(ids.as_best_external(), Some("anilist:12".to_string()));
    }

    #[test]
    fn test_try_from_other_ids_to_rsids() -> Result<(), RsIdsError> {
        let input = OtherIds(vec![
            "imdb:tt1234567".to_string(),
            "tmdb:42".to_string(),
            "foo:bar".to_string(),
        ]);
        let ids = RsIds::try_from(input)?;

        assert_eq!(ids.imdb(), Some("tt1234567"));
        assert_eq!(ids.tmdb(), Some(42));
        assert_eq!(ids.get("foo"), Some("bar"));
        Ok(())
    }

    #[test]
    fn test_serde_json_roundtrip() {
        let mut ids = RsIds::default();
        ids.set("trakt", "905982");
        ids.set("imdb", "tt1234567");
        ids.set("custom", "abc");

        let json = serde_json::to_string(&ids).unwrap();
        let parsed: RsIds = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.trakt(), Some(905982));
        assert_eq!(parsed.imdb(), Some("tt1234567"));
        assert_eq!(parsed.get("custom"), Some("abc"));
    }

    #[test]
    fn test_serde_deserialize_numeric_values() {
        // Old format: numeric fields were numbers
        let json = r#"{"trakt": 905982, "imdb": "tt123", "tmdb": 42}"#;
        let ids: RsIds = serde_json::from_str(json).unwrap();
        assert_eq!(ids.trakt(), Some(905982));
        assert_eq!(ids.imdb(), Some("tt123"));
        assert_eq!(ids.tmdb(), Some(42));
    }

    #[test]
    fn test_serde_serialize_numeric_keys_as_numbers() {
        let mut ids = RsIds::default();
        ids.set("trakt", "905982");
        ids.set("imdb", "tt123");

        let json = serde_json::to_string(&ids).unwrap();
        // trakt should be serialized as a number
        assert!(json.contains("\"trakt\":905982") || json.contains("\"trakt\": 905982"));
        // imdb should be a string
        assert!(json.contains("\"imdb\":\"tt123\"") || json.contains("\"imdb\": \"tt123\""));
    }

    #[test]
    fn test_serde_deserialize_old_other_ids_format() {
        let json = r#"{"imdb": "tt123", "otherIds": ["foo:1", "bar:2"]}"#;
        let ids: RsIds = serde_json::from_str(json).unwrap();
        assert_eq!(ids.imdb(), Some("tt123"));
        assert_eq!(ids.get("foo"), Some("1"));
        assert_eq!(ids.get("bar"), Some("2"));
    }

    #[test]
    fn test_serde_deserialize_old_camelcase_keys() {
        let json = r#"{"openlibraryEditionId": "OL5M", "anilistMangaId": 123}"#;
        let ids: RsIds = serde_json::from_str(json).unwrap();
        assert_eq!(ids.openlibrary_edition_id(), Some("OL5M"));
        assert_eq!(ids.anilist_manga_id(), Some(123));
    }

    #[test]
    fn test_split_details_no_details() {
        let (base, details) = RsIds::split_details("123");
        assert_eq!(base, "123");
        assert!(details.is_empty());
    }

    #[test]
    fn test_split_details_with_details() {
        let (base, details) = RsIds::split_details("123|volume:1|chapter:2.5");
        assert_eq!(base, "123");
        assert_eq!(details, vec![("volume", "1"), ("chapter", "2.5")]);
    }

    #[test]
    fn test_find_detail_in_pipe_values() {
        let mut ids = RsIds::default();
        ids.set("anilist", "123|volume:1|chapter:2.5");
        assert_eq!(ids.find_detail_f64("volume"), Some(1.0));
        assert_eq!(ids.find_detail_f64("chapter"), Some(2.5));
        assert_eq!(ids.find_detail("volume"), Some("1"));
    }

    #[test]
    fn test_find_detail_top_level_fallback() {
        let mut ids = RsIds::default();
        ids.set("volume", "3.0");
        assert_eq!(ids.find_detail_f64("volume"), Some(3.0));
    }

    #[test]
    fn test_has_common_id_match() {
        let mut a = RsIds::default();
        a.set("imdb", "tt1234567");
        a.set("tmdb", "42");

        let mut b = RsIds::default();
        b.set("trakt", "999");
        b.set("tmdb", "42");

        assert!(a.has_common_id(&b));
    }

    #[test]
    fn test_has_common_id_same_key_different_value() {
        let mut a = RsIds::default();
        a.set("tmdb", "42");

        let mut b = RsIds::default();
        b.set("tmdb", "99");

        assert!(!a.has_common_id(&b));
    }

    #[test]
    fn test_has_common_id_no_overlap() {
        let mut a = RsIds::default();
        a.set("imdb", "tt123");

        let mut b = RsIds::default();
        b.set("trakt", "456");

        assert!(!a.has_common_id(&b));
    }

    #[test]
    fn test_has_common_id_empty() {
        let a = RsIds::default();
        let mut b = RsIds::default();
        b.set("imdb", "tt123");

        assert!(!a.has_common_id(&b));
        assert!(!b.has_common_id(&a));
        assert!(!a.has_common_id(&a));
    }

    #[test]
    fn test_merge_priority() {
        let mut a = RsIds::default();
        a.set("imdb", "tt111");
        a.set("tmdb", "42");

        let mut b = RsIds::default();
        b.set("imdb", "tt222");
        b.set("trakt", "999");

        a.merge(&b);
        assert_eq!(a.imdb(), Some("tt111")); // self wins
        assert_eq!(a.tmdb(), Some(42));
        assert_eq!(a.trakt(), Some(999)); // added from other
    }

    #[test]
    fn test_merge_adds_missing() {
        let mut a = RsIds::default();
        a.set("imdb", "tt123");

        let mut b = RsIds::default();
        b.set("trakt", "456");
        b.set("tmdb", "789");

        a.merge(&b);
        assert_eq!(a.len(), 3);
        assert_eq!(a.trakt(), Some(456));
        assert_eq!(a.tmdb(), Some(789));
    }

    #[test]
    fn test_merge_empty() {
        let mut a = RsIds::default();
        a.set("imdb", "tt123");

        let empty = RsIds::default();
        a.merge(&empty);
        assert_eq!(a.len(), 1);

        let mut b = RsIds::default();
        b.merge(&a);
        assert_eq!(b.imdb(), Some("tt123"));
    }

    #[cfg(feature = "rusqlite")]
    #[test]
    fn test_rusqlite_roundtrip_rsids() -> rusqlite::Result<()> {
        use rusqlite::Connection;

        let conn = Connection::open_in_memory()?;
        conn.execute("CREATE TABLE test_rsids (ids TEXT NOT NULL)", [])?;

        let mut ids = RsIds::default();
        ids.set("foo", "42");
        ids.set("bar", "abc");
        conn.execute("INSERT INTO test_rsids (ids) VALUES (?1)", [&ids])?;

        let loaded: RsIds =
            conn.query_row("SELECT ids FROM test_rsids LIMIT 1", [], |row| row.get(0))?;

        assert_eq!(loaded.get("foo"), Some("42"));
        assert_eq!(loaded.get("bar"), Some("abc"));
        Ok(())
    }

    #[test]
    fn test_to_url_id_roundtrip() {
        let mut ids = RsIds::default();
        ids.set("trakt", "905982");
        ids.set("imdb", "tt1234567");

        let url_id = ids.to_url_id();
        assert!(url_id.starts_with("ids:"));
        assert!(RsIds::is_id(&url_id));

        let decoded: RsIds = url_id.try_into().unwrap();
        assert_eq!(decoded.trakt(), Some(905982));
        assert_eq!(decoded.imdb(), Some("tt1234567"));
    }

    #[test]
    fn test_from_base64url_multi() {
        let json = r#"{"trakt":905982,"imdb":"tt1234567","tmdb":12345}"#;
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json.as_bytes());
        let url_id = format!("ids:{}", encoded);

        let ids: RsIds = url_id.try_into().unwrap();
        assert_eq!(ids.trakt(), Some(905982));
        assert_eq!(ids.imdb(), Some("tt1234567"));
        assert_eq!(ids.tmdb(), Some(12345));
    }

    #[test]
    fn test_from_base64url_invalid() {
        let result: Result<RsIds, _> = "ids:not-valid-base64!!!".to_string().try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_is_id_base64url() {
        let mut ids = RsIds::default();
        ids.set("trakt", "905982");
        let url_id = ids.to_url_id();
        assert!(RsIds::is_id(&url_id));
    }

    #[test]
    fn test_try_from_single_still_works() {
        let ids: RsIds = "trakt:905982".to_string().try_into().unwrap();
        assert_eq!(ids.trakt(), Some(905982));
    }
}
