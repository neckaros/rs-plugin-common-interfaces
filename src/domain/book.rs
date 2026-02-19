use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::{other_ids::OtherIds, rs_ids::RsIds};

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
}

impl BookForUpdate {
    pub fn has_update(&self) -> bool {
        self != &BookForUpdate::default()
    }
}


#[cfg(test)]
mod tests {
    use super::Book;
    use crate::domain::other_ids::OtherIds;
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
}
