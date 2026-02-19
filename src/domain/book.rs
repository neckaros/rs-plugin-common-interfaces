use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::{
    media::MediaItemReference, other_ids::OtherIds, person::Person, rs_ids::{ApplyRsIds, RsIds}, tag::Tag
};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    #[serde(default)]
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serie_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airdate: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn13: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openlibrary_edition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openlibrary_work_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_books_volume_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asin: Option<String>,
    pub otherids: Option<OtherIds>,
    #[serde(default)]
    pub modified: u64,
    #[serde(default)]
    pub added: u64,
}

impl From<Book> for RsIds {
    fn from(value: Book) -> Self {
        RsIds {
            redseat: Some(value.id),
            isbn13: value.isbn13,
            openlibrary_edition_id: value.openlibrary_edition_id,
            openlibrary_work_id: value.openlibrary_work_id,
            google_books_volume_id: value.google_books_volume_id,
            asin: value.asin,
            other_ids: value.otherids,
            volume: value.volume,
            chapter: value.chapter,
            ..Default::default()
        }
    }
}

impl ApplyRsIds for Book {
    fn apply_rs_ids(&mut self, ids: &RsIds) {
        if let Some(redseat) = ids.redseat.as_ref() {
            self.id = redseat.clone();
        }
        if let Some(isbn13) = ids.isbn13.as_ref() {
            self.isbn13 = Some(isbn13.clone());
        }
        if let Some(openlibrary_edition_id) = ids.openlibrary_edition_id.as_ref() {
            self.openlibrary_edition_id = Some(openlibrary_edition_id.clone());
        }
        if let Some(openlibrary_work_id) = ids.openlibrary_work_id.as_ref() {
            self.openlibrary_work_id = Some(openlibrary_work_id.clone());
        }
        if let Some(google_books_volume_id) = ids.google_books_volume_id.as_ref() {
            self.google_books_volume_id = Some(google_books_volume_id.clone());
        }
        if let Some(asin) = ids.asin.as_ref() {
            self.asin = Some(asin.clone());
        }
        if let Some(other_ids) = ids.other_ids.as_ref() {
            self.otherids = Some(other_ids.clone());
        }
        if let Some(volume) = ids.volume {
            self.volume = Some(volume);
        }
        if let Some(chapter) = ids.chapter {
            self.chapter = Some(chapter);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookForUpdate {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub serie_ref: Option<String>,
    pub volume: Option<f64>,
    pub chapter: Option<f64>,
    pub year: Option<u16>,
    pub airdate: Option<i64>,
    pub overview: Option<String>,
    pub pages: Option<u32>,
    pub params: Option<Value>,
    pub lang: Option<String>,
    pub original: Option<String>,
    pub isbn13: Option<String>,
    pub openlibrary_edition_id: Option<String>,
    pub openlibrary_work_id: Option<String>,
    pub google_books_volume_id: Option<String>,
    pub asin: Option<String>,
    pub otherids: Option<OtherIds>,

    pub people_lookup: Option<Vec<Person>>,


    pub people: Option<Vec<Person>>,
    pub tags: Option<Vec<Tag>>,

    pub add_tags: Option<Vec<MediaItemReference>>,
    pub remove_tags: Option<Vec<String>>,
    pub tags_lookup: Option<Vec<Tag>>,

    pub add_people: Option<Vec<MediaItemReference>>,
    pub remove_people: Option<Vec<String>>,
}

impl BookForUpdate {
    pub fn has_update(&self) -> bool {
        self != &BookForUpdate::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Book;
    use crate::domain::{
        other_ids::OtherIds,
        rs_ids::{ApplyRsIds, RsIds},
    };
    use serde_json::json;

    #[test]
    fn book_otherids_serializes_as_array_and_rejects_string() {
        let book = Book {
            id: "book-1".to_string(),
            name: "Book 1".to_string(),
            otherids: Some(OtherIds(vec!["goodreads:321".to_string()])),
            ..Default::default()
        };
        let value = serde_json::to_value(&book).unwrap();
        assert_eq!(value.get("otherids"), Some(&json!(["goodreads:321"])));

        let parsed: Book = serde_json::from_value(json!({
            "id": "book-1",
            "name": "Book 1",
            "otherids": ["custom:1"]
        }))
        .unwrap();
        assert_eq!(
            parsed.otherids,
            Some(OtherIds(vec!["custom:1".to_string()]))
        );

        let invalid = serde_json::from_value::<Book>(json!({
            "id": "book-1",
            "name": "Book 1",
            "otherids": "custom:1"
        }));
        assert!(invalid.is_err());
    }

    #[test]
    fn book_apply_rs_ids_updates_only_present_values() {
        let mut book = Book {
            id: "book-old".to_string(),
            name: "Book 1".to_string(),
            openlibrary_work_id: Some("olw-old".to_string()),
            chapter: Some(7.0),
            ..Default::default()
        };
        let ids = RsIds {
            redseat: Some("book-new".to_string()),
            isbn13: Some("9783161484100".to_string()),
            openlibrary_edition_id: Some("ole-new".to_string()),
            google_books_volume_id: Some("gb-1".to_string()),
            asin: Some("B00TEST".to_string()),
            other_ids: Some(OtherIds(vec!["goodreads:42".to_string()])),
            volume: Some(3.0),
            ..Default::default()
        };

        book.apply_rs_ids(&ids);

        assert_eq!(book.id, "book-new");
        assert_eq!(book.isbn13.as_deref(), Some("9783161484100"));
        assert_eq!(book.openlibrary_edition_id.as_deref(), Some("ole-new"));
        assert_eq!(book.openlibrary_work_id.as_deref(), Some("olw-old"));
        assert_eq!(book.google_books_volume_id.as_deref(), Some("gb-1"));
        assert_eq!(book.asin.as_deref(), Some("B00TEST"));
        assert_eq!(
            book.otherids,
            Some(OtherIds(vec!["goodreads:42".to_string()]))
        );
        assert_eq!(book.volume, Some(3.0));
        assert_eq!(book.chapter, Some(7.0));
    }
}
