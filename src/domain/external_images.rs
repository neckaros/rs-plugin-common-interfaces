use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};



#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Display, EnumString)]
#[serde(rename_all = "camelCase")]
pub enum ImageType {
    Poster,
    Background,
    Still,
    Card,
    ClearLogo,
    ClearArt,
    Custom(String)
}


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExternalImage {
    #[serde(rename = "type")]
    pub kind: Option<ImageType>,
    pub url: String,
    pub aspect_ratio: Option<f64>,
    pub height: Option<i64>,
    pub lang: Option<String>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<i64>,
    pub width: Option<i64>,
}


#[cfg(feature = "rusqlite")]
pub mod external_images_rusqlite {
    use std::str::FromStr;
    use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, ToSql};

    use super::{ExternalImage, ImageType};

    impl FromSql for ExternalImage {
        fn column_result(value: ValueRef) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|as_string| {
                let r = serde_json::from_str(&as_string).map_err(|_| FromSqlError::InvalidType);
                r
            })
        }
    }
    impl ToSql for ExternalImage {
        fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
            let r = serde_json::to_string(self).map_err(|_| FromSqlError::InvalidType)?;
            Ok(ToSqlOutput::from(r))
        }
    }

    impl FromSql for ImageType {
        fn column_result(value: ValueRef) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|as_string| {
                ImageType::from_str(&as_string).map_err(|_| FromSqlError::InvalidType)
            })
        }
    }
    
    
    impl ToSql for ImageType {
        fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
            let l = (&self.clone()).to_string();
            Ok(ToSqlOutput::from(l))
        }
    }    
}