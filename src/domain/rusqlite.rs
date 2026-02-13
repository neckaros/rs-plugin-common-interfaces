use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, ToSql};

use super::serie::SerieStatus;
use super::media::{FileType, RsGpsPosition};
use super::movie::MovieStatus;

impl FromSql for SerieStatus {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|as_string| {
            SerieStatus::try_from(&*as_string).map_err(|_| FromSqlError::InvalidType)
        })
    }
}

impl ToSql for SerieStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}



// Medias


impl FromSql for RsGpsPosition {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|as_string| {
            let mut splitted = as_string.split(",");
            let lat = splitted
                .next()
                .and_then(|f| f.parse::<f64>().ok())
                .ok_or(FromSqlError::InvalidType)?;
            let long = splitted
                .next()
                .and_then(|f| f.parse::<f64>().ok())
                .ok_or(FromSqlError::InvalidType)?;
            Ok(RsGpsPosition { lat, long })
        })
    }
}

impl FromSql for FileType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|as_string| {
            let r = FileType::try_from(&*as_string).map_err(|_| FromSqlError::InvalidType);
            r
        })
    }
}

impl ToSql for FileType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}


// Movie
impl FromSql for MovieStatus {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|as_string| {
            MovieStatus::try_from(&*as_string).map_err(|_| FromSqlError::InvalidType)
        })
    }
}

impl ToSql for MovieStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}
