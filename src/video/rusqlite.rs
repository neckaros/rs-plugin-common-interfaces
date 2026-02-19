use std::str::FromStr;

use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};

use crate::{video::RsVideoTranscodeStatus, RsVideoFormat};

impl FromSql for RsVideoTranscodeStatus {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|as_string| {
            let r =
                RsVideoTranscodeStatus::from_str(&as_string).map_err(|_| FromSqlError::InvalidType);
            r
        })
    }
}
impl ToSql for RsVideoTranscodeStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let l = (&self.clone()).to_string();
        Ok(ToSqlOutput::from(l))
    }
}
