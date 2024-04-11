use std::str::FromStr;

use crate::{RsLink, RsLinkType};
use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, ToSql};



impl FromSql for RsLink {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|as_string| {
            let r = serde_json::from_str(&as_string).map_err(|_| FromSqlError::InvalidType);
            r
        })
    }
}

impl ToSql for RsLink {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let r = serde_json::to_string(self).map_err(|_| FromSqlError::InvalidType)?;
        Ok(ToSqlOutput::from(r))
    }
}

impl FromSql for RsLinkType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|as_string| {
            let r = RsLinkType::from_str(&as_string).map_err(|_| FromSqlError::InvalidType);
            r
        })
    }
}


impl ToSql for RsLinkType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let l = (&self.clone()).to_string();
        Ok(ToSqlOutput::from(l))
    }
}

