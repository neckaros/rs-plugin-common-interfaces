use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct OtherIds(pub Vec<String>);

impl OtherIds {
    fn normalize_key(key: &str) -> String {
        key.trim().to_ascii_lowercase()
    }

    fn split_entry(entry: &str) -> Option<(&str, &str)> {
        entry.split_once(':')
    }

    fn key_of(entry: &str) -> Option<String> {
        Self::split_entry(entry).map(|(key, _)| Self::normalize_key(key))
    }

    pub fn add(&mut self, key: &str, value: &str) {
        let normalized_key = Self::normalize_key(key);
        if normalized_key.is_empty() {
            return;
        }
        let entry = format!("{normalized_key}:{value}");
        if let Some(index) = self
            .0
            .iter()
            .position(|existing| Self::key_of(existing).as_deref() == Some(normalized_key.as_str()))
        {
            self.0[index] = entry;
        } else {
            self.0.push(entry);
        }
    }

    pub fn has_key(&self, key: &str) -> bool {
        let normalized_key = Self::normalize_key(key);
        if normalized_key.is_empty() {
            return false;
        }
        self.0
            .iter()
            .any(|entry| Self::key_of(entry).as_deref() == Some(normalized_key.as_str()))
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let normalized_key = Self::normalize_key(key);
        if normalized_key.is_empty() {
            return None;
        }
        self.0.iter().find_map(|entry| {
            let (entry_key, entry_value) = Self::split_entry(entry)?;
            if Self::normalize_key(entry_key) == normalized_key {
                Some(entry_value.to_string())
            } else {
                None
            }
        })
    }

    pub fn contains(&self, key: &str, value: &str) -> bool {
        self.get(key).as_deref() == Some(value)
    }

    pub fn as_slice(&self) -> &[String] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<String> {
        self.0
    }
}

impl From<Vec<String>> for OtherIds {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl From<OtherIds> for Vec<String> {
    fn from(value: OtherIds) -> Self {
        value.0
    }
}

#[cfg(feature = "rusqlite")]
pub mod other_ids_rusqlite {
    use rusqlite::{
        types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
        ToSql,
    };

    use super::OtherIds;

    impl FromSql for OtherIds {
        fn column_result(value: ValueRef) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|as_string| {
                serde_json::from_str(&as_string).map_err(|_| FromSqlError::InvalidType)
            })
        }
    }
    impl ToSql for OtherIds {
        fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
            let serialized = serde_json::to_string(self).map_err(|_| FromSqlError::InvalidType)?;
            Ok(ToSqlOutput::from(serialized))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OtherIds;

    #[test]
    fn test_add_get_contains_and_has_key() {
        let mut ids = OtherIds::default();
        ids.add("IMDB", "tt123");
        assert!(ids.has_key("imdb"));
        assert!(ids.contains("imdb", "tt123"));
        assert_eq!(ids.get("ImDb"), Some("tt123".to_string()));
    }

    #[test]
    fn test_replace_existing_key_with_latest_value() {
        let mut ids = OtherIds::default();
        ids.add("tmdb", "1");
        ids.add("TMDB", "2");
        assert_eq!(ids.as_slice(), &["tmdb:2".to_string()]);
    }

    #[test]
    fn test_preserves_values_and_key_normalization() {
        let mut ids = OtherIds::default();
        ids.add("Custom", "provider:value");
        assert_eq!(ids.as_slice(), &["custom:provider:value".to_string()]);
        assert_eq!(ids.get("custom"), Some("provider:value".to_string()));
    }

    #[cfg(feature = "rusqlite")]
    #[test]
    fn test_rusqlite_roundtrip_other_ids() -> rusqlite::Result<()> {
        use rusqlite::Connection;

        let conn = Connection::open_in_memory()?;
        conn.execute("CREATE TABLE test_other_ids (ids TEXT NOT NULL)", [])?;

        let ids = OtherIds(vec!["imdb:tt123".to_string(), "tmdb:42".to_string()]);
        conn.execute("INSERT INTO test_other_ids (ids) VALUES (?1)", [&ids])?;

        let loaded: OtherIds =
            conn.query_row("SELECT ids FROM test_other_ids LIMIT 1", [], |row| {
                row.get(0)
            })?;
        assert_eq!(loaded, ids);
        Ok(())
    }
}
