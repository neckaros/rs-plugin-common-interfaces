use std::str::FromStr;

use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, ToSql};

use crate::PluginType;

impl FromSql for PluginType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|as_string| {
            let r = PluginType::from_str(&as_string).map_err(|_| FromSqlError::InvalidType);
            r
        })
    }
}


impl ToSql for PluginType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let l = &self.clone();
        let r = l.to_string();
        Ok(ToSqlOutput::from(r))
    }
}