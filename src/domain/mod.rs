use serde::{Deserialize, Serialize};

pub mod backup;
pub mod book;
pub mod element_type;
pub mod episode;
pub mod external_images;
pub mod media;
pub mod movie;
pub mod other_ids;
pub mod person;
pub mod rs_ids;
pub mod serie;
pub mod tag;
pub mod tools;

#[cfg(feature = "rusqlite")]
pub mod rusqlite;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MediaElement {
    Media(media::Media),
    Movie(movie::Movie),
    Episode(episode::Episode),
    Serie(serie::Serie),
    Book(book::Book),
}
