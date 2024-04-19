use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum ElementType {
    Tag,
    Person,
    Media,
	Movie,
    Serie,
	Episode,
    Book,
    Song,
    #[default]
    Unknown,
}

#[cfg(feature = "rusqlite")]
pub mod element_type_rusqlite {
    use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, ToSql};

    use super::ElementType;

    impl FromSql for ElementType {
        fn column_result(value: ValueRef) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|as_string| {
                ElementType::from_str(&as_string).map_err(|_| FromSqlError::InvalidType)
            })
        }
    }
    
    
    impl ToSql for ElementType {
        fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
            let l = (&self.clone()).to_string();
            Ok(ToSqlOutput::from(l))
        }
    }    
}