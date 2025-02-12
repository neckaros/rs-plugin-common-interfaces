use serde::{Deserialize, Serialize};



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
}


#[cfg(feature = "rusqlite")]
pub mod external_images_rusqlite {
    use std::str::FromStr;
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