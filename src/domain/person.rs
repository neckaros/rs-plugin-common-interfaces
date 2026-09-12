use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    domain::{
        other_ids::OtherIds,
        rs_ids::{ApplyRsIds, RsIds},
    },
    url::RsLink,
    Gender,
};

/// A person's primary type. Plugins map provider labels to canonical variants.
/// Unknown strings are preserved verbatim; no aliases or translations are applied.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(from = "String", into = "String")]
pub enum PersonType {
    Actor,
    Director,
    Writer,
    Producer,
    Creator,
    Author,
    Family,
    Friends,
    Singer,
    Custom(String),
}

impl PersonType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Actor => "Actor",
            Self::Director => "Director",
            Self::Writer => "Writer",
            Self::Producer => "Producer",
            Self::Creator => "Creator",
            Self::Author => "Author",
            Self::Family => "Family",
            Self::Friends => "Friends",
            Self::Singer => "Singer",
            Self::Custom(value) => value,
        }
    }
}

impl From<String> for PersonType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Actor" => Self::Actor,
            "Director" => Self::Director,
            "Writer" => Self::Writer,
            "Producer" => Self::Producer,
            "Creator" => Self::Creator,
            "Author" => Self::Author,
            "Family" => Self::Family,
            "Friends" => Self::Friends,
            "Singer" => Self::Singer,
            _ => Self::Custom(value),
        }
    }
}

impl From<&str> for PersonType {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

impl From<PersonType> for String {
    fn from(value: PersonType) -> Self {
        match value {
            PersonType::Custom(value) => value,
            value => value.as_str().to_owned(),
        }
    }
}

impl std::str::FromStr for PersonType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(value.into())
    }
}

impl std::fmt::Display for PersonType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::ToSql for PersonType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Borrowed(
            rusqlite::types::ValueRef::Text(self.as_str().as_bytes()),
        ))
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::types::FromSql for PersonType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Ok(value.as_str()?.into())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,
    pub name: String,
    pub socials: Option<Vec<RsLink>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub kind: Option<PersonType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portrait: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<i64>,
    pub modified: u64,
    pub added: u64,
    pub posterv: u32,
    #[serde(default)]
    pub generated: bool,

    pub imdb: Option<String>,
    pub slug: Option<String>,
    pub tmdb: Option<u64>,
    pub trakt: Option<u64>,

    pub death: Option<i64>,
    pub gender: Option<Gender>,
    pub country: Option<String>,
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otherids: Option<OtherIds>,
}

impl ApplyRsIds for Person {
    fn apply_rs_ids(&mut self, ids: &RsIds) {
        if let Some(trakt) = ids.trakt() {
            self.trakt = Some(trakt);
        }
        if let Some(slug) = ids.slug() {
            self.slug = Some(slug.to_string());
        }
        if let Some(imdb) = ids.imdb() {
            self.imdb = Some(imdb.to_string());
        }
        if let Some(tmdb) = ids.tmdb() {
            self.tmdb = Some(tmdb);
        }
        let known: &[&str] = &["redseat", "trakt", "slug", "imdb", "tmdb"];
        let mut other = self.otherids.take().unwrap_or_default();
        for (k, v) in ids.iter() {
            if !known.contains(&k.as_str()) {
                other.add(k, v);
            }
        }
        if !other.as_slice().is_empty() {
            self.otherids = Some(other);
        }
    }
}

impl From<Person> for RsIds {
    fn from(value: Person) -> Self {
        let mut ids = RsIds::default();
        if let Some(v) = value.trakt {
            ids.set("trakt", v);
        }
        if let Some(v) = value.slug {
            ids.set("slug", v);
        }
        if let Some(v) = value.imdb {
            ids.set("imdb", v);
        }
        if let Some(v) = value.tmdb {
            ids.set("tmdb", v);
        }
        if let Some(other) = value.otherids {
            for entry in other.into_vec() {
                let _ = ids.try_add(entry);
            }
        }
        if ids.try_add(value.id.clone()).is_err() {
            ids.set("redseat", value.id);
        }
        ids
    }
}

#[cfg(test)]
mod person_type_tests {
    use super::*;

    #[test]
    fn canonical_types_use_plain_strings() {
        let cases = [
            (PersonType::Actor, "Actor"),
            (PersonType::Director, "Director"),
            (PersonType::Writer, "Writer"),
            (PersonType::Producer, "Producer"),
            (PersonType::Creator, "Creator"),
            (PersonType::Author, "Author"),
            (PersonType::Family, "Family"),
            (PersonType::Friends, "Friends"),
            (PersonType::Singer, "Singer"),
        ];
        for (kind, wire) in cases {
            let person = Person {
                kind: Some(kind.clone()),
                ..Default::default()
            };
            let json = serde_json::to_value(&person).unwrap();
            assert_eq!(json["type"], wire);
            assert!(json.get("kind").is_none());
            assert_eq!(
                serde_json::from_value::<Person>(json).unwrap().kind,
                Some(kind.clone())
            );
            assert_eq!(wire.parse::<PersonType>().unwrap(), kind);
            assert_eq!(kind.to_string(), wire);
        }
    }

    #[test]
    fn custom_and_legacy_strings_are_preserved_without_mapping() {
        for value in [
            "custom name",
            "acting",
            "acteur",
            "actor",
            "Acting",
            "",
            "  Family friend  ",
            "Réalisateur",
        ] {
            let kind = PersonType::Custom(value.into());
            let mut json = serde_json::to_value(Person::default()).unwrap();
            json["type"] = value.into();
            let person: Person = serde_json::from_value(json).unwrap();
            assert_eq!(person.kind, Some(kind));
            assert_eq!(serde_json::to_value(person).unwrap()["type"], value);
        }
    }

    #[test]
    fn missing_and_null_types_remain_optional_and_objects_are_rejected() {
        let mut json = serde_json::to_value(Person::default()).unwrap();
        assert!(json.get("type").is_none());
        assert!(serde_json::from_value::<Person>(json.clone())
            .unwrap()
            .kind
            .is_none());
        json["type"] = serde_json::Value::Null;
        assert!(serde_json::from_value::<Person>(json.clone())
            .unwrap()
            .kind
            .is_none());
        for invalid in [serde_json::json!({"Custom": "actor"}), serde_json::json!(3)] {
            json["type"] = invalid;
            assert!(serde_json::from_value::<Person>(json.clone()).is_err());
        }
    }

    #[cfg(feature = "rusqlite")]
    #[test]
    fn sqlite_types_remain_plain_text() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE people (type TEXT)", []).unwrap();
        for kind in [PersonType::Actor, PersonType::Custom("custom name".into())] {
            conn.execute("INSERT INTO people (type) VALUES (?)", [&kind])
                .unwrap();
            let (text, restored): (String, PersonType) = conn
                .query_row(
                    "SELECT type, type FROM people ORDER BY rowid DESC LIMIT 1",
                    [],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .unwrap();
            assert_eq!(text, kind.as_str());
            assert_eq!(restored, kind);
        }
    }
}

/// A person viewed in the context of a book, movie, or show relationship.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PersonWithRoles {
    #[serde(flatten)]
    pub person: Person,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<PersonType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<String>>,
}

#[cfg(test)]
mod credit_tests {
    use super::*;
    use crate::domain::Relations;

    #[test]
    fn relationship_roles_preserve_strings_and_legacy_payloads() {
        let legacy = serde_json::json!({"peopleDetails": [Person { id: "tmdb:1".into(), name: "Person".into(), ..Default::default() }]});
        let relations: Relations = serde_json::from_value(legacy).unwrap();
        assert!(relations.people_roles.is_none());
        let credit = PersonWithRoles {
            person: Person { id: "local".into(), name: "Person".into(), kind: Some(PersonType::Actor), ..Default::default() },
            roles: Some(vec![PersonType::Director, PersonType::Custom("custom name".into())]),
            characters: Some(vec!["Character A".into(), "Character B".into()]),
        };
        let wire = serde_json::to_value(&credit).unwrap();
        assert_eq!(wire["type"], "Actor");
        assert_eq!(wire["characters"], serde_json::json!(["Character A", "Character B"]));
        assert_eq!(wire["roles"], serde_json::json!(["Director", "custom name"]));
        assert_eq!(serde_json::from_value::<PersonWithRoles>(wire).unwrap(), credit);
        let relations = Relations { people_roles: Some(std::collections::HashMap::from([
            ("tmdb:1".into(), vec![PersonType::Actor, PersonType::Director])
        ])), ..Default::default() };
        assert_eq!(serde_json::to_value(relations).unwrap()["peopleRoles"]["tmdb:1"], serde_json::json!(["Actor","Director"]));
    }
}
