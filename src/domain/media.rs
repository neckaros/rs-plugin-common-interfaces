use std::str::FromStr;

use crate::url::RsLink;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;

use crate::domain::backup::BackupFile;

pub const DEFAULT_MIME: &str = "application/octet-stream";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileEpisode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode_to: Option<u32>,
}

impl FromStr for FileEpisode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split("|").collect();
        if splitted.len() == 3 {
            Ok(FileEpisode {
                id: splitted[0].to_string(),
                season: splitted[1].parse::<u32>().ok().and_then(|i| {
                    if i == 0 {
                        None
                    } else {
                        Some(i)
                    }
                }),
                episode: splitted[2].parse::<u32>().ok().and_then(|i| {
                    if i == 0 {
                        None
                    } else {
                        Some(i)
                    }
                }),
                episode_to: None,
            })
        } else if splitted.len() == 2 {
            Ok(FileEpisode {
                id: splitted[0].to_string(),
                season: splitted[1].parse::<u32>().ok().and_then(|i| {
                    if i == 0 {
                        None
                    } else {
                        Some(i)
                    }
                }),
                episode: None,
                episode_to: None,
            })
        } else {
            Ok(FileEpisode {
                id: splitted[0].to_string(),
                season: None,
                episode: None,
                episode_to: None,
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MediaItemReference {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conf: Option<u16>,
}

impl FromStr for MediaItemReference {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split('|').collect();
        if splitted.len() == 2 {
            Ok(MediaItemReference {
                id: splitted[0].to_string(),
                conf: splitted[1].parse::<u16>().ok().and_then(|e| {
                    if e == 100 {
                        None
                    } else {
                        Some(e)
                    }
                }),
            })
        } else {
            Ok(MediaItemReference {
                id: splitted[0].to_string(),
                conf: None,
            })
        }
    }
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[strum(serialize_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub enum FileType {
    Directory,
    Photo,
    Video,
    Archive,
    Album,
    Book,
    #[default]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "type")]
    pub kind: FileType,
    pub mimetype: String,
    pub size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,

    pub added: Option<i64>,
    pub modified: Option<i64>,
    pub created: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_rating: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focal: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sspeed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_number: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acodecs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub achan: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcodecs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub long: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<MediaItemReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<FileEpisode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<MediaItemReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<FaceEmbedding>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<BackupFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbv: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbsize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<RsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploader: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploadkey: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_recognition_error: Option<String>,
}

impl Media {
    pub fn max_date(&self) -> i64 {
        *[
            self.created.unwrap_or(0),
            self.added.unwrap_or(0),
            self.modified.unwrap_or(0),
        ]
        .iter()
        .max()
        .unwrap_or(&0)
    }

    pub fn bytes_size(&self) -> Option<u64> {
        if self.iv.is_none() {
            self.size
        } else {
            //16 Bytes to store IV
            //4 to store encrypted thumb size = T (can be 0)
            //4 to store encrypted Info size = I (can be 0)
            //32 to store thumb mimetype
            //256 to store file mimetype
            //T Bytes for the encrypted thumb
            //I Bytes for the encrypted info
            if let Some(file_size) = self.size {
                Some(file_size + 16 + 4 + 4 + 32 + 256 + self.thumbsize.unwrap_or(0) + 0)
            } else {
                None
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsGpsPosition {
    pub lat: f64,
    pub long: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FaceEmbedding {
    pub id: String,
    pub embedding: Vec<f32>,
    pub media_ref: Option<String>,
    pub bbox: Option<FaceBBox>,
    pub confidence: Option<f32>,
    pub pose: Option<(f32, f32, f32)>,
    pub person_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FaceBBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_s: Option<f32>, // Seconds in video where face was detected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_percent: Option<u32>, // Percent (0-100) of video where face was detected
}
