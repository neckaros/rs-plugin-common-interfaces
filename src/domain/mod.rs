use serde::{Deserialize, Serialize};

pub mod tools;
pub mod element_type;
pub mod external_images;
pub mod rs_ids;
pub mod media;
pub mod book;
pub mod serie;
pub mod episode;
pub mod movie;
pub mod person;
pub mod backup;

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
