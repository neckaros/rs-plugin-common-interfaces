use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;

pub use domain::{
    element_type::ElementType,
    external_images::{ExternalImage, ImageType},
    other_ids::OtherIds,
};
pub use lookup::{
    RsLookupBook, RsLookupEpisode, RsLookupMedia, RsLookupMovie, RsLookupPerson, RsLookupQuery,
    RsLookupSerie, RsLookupSerieSeason, RsLookupSong, RsLookupSourceResult, RsLookupWrapper,
};
pub use request::{
    RsCookie, RsCookies, RsRequest, RsRequestFiles, RsRequestPluginRequest, RsRequestStatus,
};
pub use url::{RsLink, RsLinkType};

pub use video::{RsAudio, RsResolution, RsVideoCodec, RsVideoFormat};

#[cfg(feature = "rusqlite")]
pub mod rusqlite;

pub mod lookup;
pub mod provider;
pub mod request;
pub mod url;

pub mod video;

pub mod domain;

pub const INTERFACE_VERSION: u16 = 1;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CustomParam {
    pub name: String,
    pub param: CustomParamTypes,
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginInformation {
    pub name: String,
    pub capabilities: Vec<PluginType>,
    pub publisher: String,
    pub description: String,
    pub credential_kind: Option<CredentialType>,
    pub repo: Option<String>,
    pub oauth_url: Option<String>,
    pub version: u16,

    #[serde(default)]
    pub settings: Vec<CustomParam>,

    pub interface_version: u16,
}

impl PluginInformation {
    pub fn capabilities_tostring(&self) -> String {
        self.capabilities
            .iter()
            .map(|plugin| plugin.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum PluginType {
    ImageClassification,
    UrlParser,
    Request,
    Lookup,
    LookupMetadata,
    Provider,
    VideoConvert,
    #[default]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsRemainingCredits {
    pub number: u64,
    pub unit: String,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[serde(rename_all = "camelCase", tag = "type")]
#[strum(serialize_all = "camelCase")]
pub enum CredentialType {
    Url,
    Password,
    Oauth {
        /// Oauth url to get code from user; use #redirecturi# in the url
        url: String,
    },
    #[default]
    Token,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum CustomParamTypes {
    Text(Option<String>),
    Url(Option<String>),
    Integer(Option<i64>),
    UInteger(Option<u64>),
    Float(Option<f64>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginCredential {
    pub kind: CredentialType,
    pub login: Option<String>,
    pub password: Option<String>,
    pub settings: Value,
    pub user_ref: Option<String>,
    pub refresh_token: Option<String>,
    pub expires: Option<i64>,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[strum(serialize_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub enum RsFileType {
    Directory,
    Photo,
    Video,
    Archive,
    Album,
    Book,
    #[default]
    Other,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum MediaType {
    Movie,
    Episode,
    Book,
    Song,
    #[strum(default)]
    Custom(String),
    #[default]
    Unknown,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum Gender {
    Male,
    Female,
    Animal,
    Other,
    #[strum(default)]
    Custom(String),
    #[default]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsPluginRequest<T> {
    pub request: T,
    pub plugin_settings: Value,
    pub credential: Option<PluginCredential>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn resolution_parsing() {
        assert_eq!(
            RsResolution::from_filename("Test.2024.S01E01.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"),
            RsResolution::FullHD
        );
        assert_eq!(
            RsResolution::from_filename("Test.2024.S01E01_720p VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"),
            RsResolution::HD
        );
        assert_eq!(
            RsResolution::from_filename("TestIn4k.2024.S01E01_VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"),
            RsResolution::Unknown
        );
        assert_eq!(
            RsResolution::from_filename(
                "TestIn4k.2024.S01E01_4K_VOSTFR.DSNP.WEB-DL.DDP5.Atmos.1.H.264"
            ),
            RsResolution::UHD
        );
    }

    #[test]
    fn resolution_string() {
        assert_eq!("4K", RsResolution::UHD.to_string());
        assert_eq!(
            RsResolution::from_str("1080p").unwrap(),
            RsResolution::FullHD
        );
        assert_eq!(
            RsResolution::from_str("erzr").unwrap(),
            RsResolution::Custom("erzr".to_owned())
        );
    }
    #[test]
    fn audio_parsing() {
        assert_eq!(
            RsAudio::from_filename("Test.2024.S01E01.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"),
            RsAudio::DDP51
        );
        assert_eq!(
            RsAudio::from_filename("Test.2024.S01E01_720p VOSTFR.DSNP.WEB-DL.DDP5.1._atmos_H.264"),
            RsAudio::Atmos
        );
        let list = RsAudio::list_from_filename(
            "TestIn4k.2024.S01E01_4K_VOSTFR.DSNP.WEB-DL.DDP5.1.Atmos.H.264",
        );
        assert_eq!(list.len(), 2);
        assert!(list.contains(&RsAudio::Atmos));
        assert!(list.contains(&RsAudio::DDP51));
    }

    #[test]
    fn videocodec_parsing() {
        assert_eq!(
            RsVideoCodec::from_filename("Test.2024.S01E01.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"),
            RsVideoCodec::H264
        );
        assert_eq!(
            RsVideoCodec::from_filename("Test.2024.S01E01_720p VOSTFR.DSNP.WEB-DL.DDP5.1.HEVC"),
            RsVideoCodec::H265
        );
        assert_eq!(
            RsVideoCodec::from_filename("TestIn4k.2024.S01E01_VOSTFR.DSNP.WEB-DL.DDP5.1.X.265"),
            RsVideoCodec::H265
        );
    }

    #[test]
    fn video_format_parsing() {
        assert_eq!(
            RsVideoFormat::from_filename(
                "Test.2024.S01E01.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264.mp4"
            ),
            RsVideoFormat::Mp4
        );
        assert_eq!(
            RsVideoFormat::from_filename(
                "Test.2024.S01E01_720p VOSTFR.DSNP.WEB-DL.DDP5.1._atmos_H.264"
            ),
            RsVideoFormat::Other
        );
        assert_eq!(
            RsVideoFormat::from_filename(
                "Test.2024.S01E01.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264.WMV"
            ),
            RsVideoFormat::Wmv
        );

        assert_eq!(RsVideoFormat::Mp4.to_string(), "mp4");
        assert_eq!(RsVideoFormat::from_str("mkv").unwrap(), RsVideoFormat::Mkv);
    }
}
