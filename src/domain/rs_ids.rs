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
}



impl RsIds {
    pub fn try_add(&mut self, value: String) -> Result<(), RsIdsError> {
        if !Self::is_id(&value) {
            return Err(RsIdsError::NotAMediaId(value))
        }
        let elements = value.split(":").collect::<Vec<_>>();
        let source = elements.first().ok_or(RsIdsError::InvalidId())?;
        let id = elements.get(1).ok_or(RsIdsError::InvalidId())?;

        if *source == "redseat" {
            self.redseat = Some(id.to_string());
            Ok(())
        } else if *source == "imdb" {
            self.imdb = Some(id.to_string());
            Ok(())
        } else if *source == "trakt" {
            let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
            self.trakt = Some(id);
            Ok(())
        } else if *source == "tmdb" {
            let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
            self.tmdb = Some(id);
            Ok(())
        } else if *source == "tvdb" {
            let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
            self.tvdb = Some(id);
            Ok(())
        } else if *source == "tvrage" {
            let id: u64 = id.parse().map_err(|_| RsIdsError::NotAMediaId(value))?;
            self.tvrage = Some(id);
            Ok(())
        } else{
            Err(RsIdsError::NotAMediaId(value))
        }  
    }

    pub fn into_best(self) -> Option<String> {
        self.as_redseat().or(self.into_best_external())
    }

    pub fn into_best_external(self) -> Option<String> {
        self.as_trakt().or(self.as_imdb()).or(self.as_tmdb()).or(self.as_tvdb())
    }
    pub fn as_best_external(&self) -> Option<String> {
        self.as_trakt().or(self.as_imdb()).or(self.as_tmdb()).or(self.as_tvdb())
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

    pub fn as_id(&self) -> Result<String, RsIdsError> {
        if let Some(imdb) = &self.imdb {
            Ok(format!("imdb:{}", imdb))
        } else if let Some(trakt) = &self.trakt {
            Ok(format!("trakt:{}", trakt))
        } else if let Some(tmdb) = &self.tmdb {
            Ok(format!("tmdb:{}", tmdb))
        } else if let Some(tvdb) = &self.tvdb {
            Ok(format!("tmdb:{}", tvdb))
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
        ids
    }
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



#[cfg(test)]
mod tests {

    use self::RsIdsError;

    use super::*;

    #[test]
    fn test_parse() -> Result<(), RsIdsError> {
        let toparse = String::from("trakt:905982");
        let parsed: Result<RsIds, _> = toparse.try_into();
        assert!(parsed.is_ok() == true);
        assert!(parsed.unwrap().trakt == Some(905982));
        Ok(())
    }
}