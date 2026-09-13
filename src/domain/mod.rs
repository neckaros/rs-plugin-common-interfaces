use serde::{Deserialize, Serialize};

use crate::domain::{media::{FileEpisode, MediaItemReference}, movie::Movie, person::Person, serie::Serie, tag::Tag};

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

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ItemWithRelations<T> {
    #[serde(flatten)]
    pub item: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Relations>,
}



#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Relations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_details: Option<Vec<person::PersonWithRoles>>,
    /// Legacy plugin input and compact title snapshot roles. New plugins use peopleDetails[].roles.
    /// Omission means unknown; an explicit empty list means no roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_roles: Option<std::collections::HashMap<String, Vec<person::PersonType>>>,
    /// Legacy plugin input and compact title snapshot names. New plugins use peopleDetails[].characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_characters: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Compact title snapshot ranks. New plugins use peopleDetails[].rank.
    /// Lower values come first (zero is first); absent entries mean unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_ranks: Option<std::collections::HashMap<String, u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_details: Option<Vec<Tag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<MediaItemReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<MediaItemReference>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<FileEpisode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_details: Option<Vec<Serie>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub movies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movies_details: Option<Vec<Movie>>,

        #[serde(skip_serializing_if = "Option::is_none")]
    pub books: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub books_details: Option<Vec<book::Book>>,


    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_images: Option<Vec<crate::domain::external_images::ExternalImage>>,
}

impl Relations {
    /// Read inline plugin credits, falling back to legacy maps only for missing fields.
    /// Local references without profiles are included for existing-book imports.
    pub fn people_credits(&self) -> Vec<person::PersonWithRoles> {
        let mut credits = self.people_details.clone().unwrap_or_default();
        for reference in self.people.iter().flatten() {
            if !credits.iter().any(|credit| credit.person.id == reference.id) {
                credits.push(Person {
                    id: reference.id.clone(),
                    ..Default::default()
                }.into());
            }
        }
        for credit in &mut credits {
            let id = &credit.person.id;
            if credit.roles.is_none() {
                credit.roles = self.people_roles.as_ref().and_then(|map| map.get(id)).cloned();
            }
            if credit.characters.is_none() {
                credit.characters = self.people_characters.as_ref().and_then(|map| map.get(id)).cloned();
            }
            if credit.rank.is_none() {
                credit.rank = self.people_ranks.as_ref().and_then(|map| map.get(id)).copied();
            }
        }
        credits
    }
}
