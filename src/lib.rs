use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;

#[cfg(feature = "rusqlite")]
pub mod rusqlite;

pub mod request;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct PluginInformation {
    pub name: String,
    pub kind: PluginType,
    pub version: usize,
    pub publisher: String,
    pub description: String,
    pub credential_kind: Option<CredentialType>,
    pub oauth_url: Option<String>
}


fn text_contains(text: &str, contains: &str) -> bool {
    text.contains(&format!(".{}.", contains)) || text.starts_with(contains) || text.ends_with(contains)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
pub enum RsResolution {
    #[strum(serialize = "4K")]
	UHD,
    #[strum(serialize = "1080p")]
    FullHD,
    #[strum(serialize = "720p")]
    HD,
    #[strum(default)]
    Custom(String),
    #[default]
    Unknown,
}

impl RsResolution {
    pub fn from_filename(filename: &str) -> Self {
        let modified_filename = filename.replace(" ", ".").replace("-", ".").replace("_", ".").to_lowercase();
        if text_contains(&modified_filename, "1080p") {
            RsResolution::FullHD
        } else if text_contains(&modified_filename, "720p") {
            RsResolution::HD
        } else if text_contains(&modified_filename, "4k") {
            RsResolution::UHD
        } else {
            RsResolution::Unknown
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
pub enum RsVideoCodec {
	X265,
    H264,
    #[strum(default)]
    Custom(String),
    #[default]
    Unknown,
}

impl RsVideoCodec {
    pub fn from_filename(filename: &str) -> Self {
        let modified_filename = filename.replace(" ", ".").replace("-", ".").replace("_", ".").to_lowercase();
        if text_contains(&modified_filename, "x265") || text_contains(&modified_filename, "x.265") || text_contains(&modified_filename, "hevc") {
            RsVideoCodec::X265
        } else if text_contains(&modified_filename, "h264")|| text_contains(&modified_filename, "h.264") {
            RsVideoCodec::H264
        } else {
            RsVideoCodec::Unknown
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
pub enum RsAudio {
    #[strum(serialize = "Atmos")]
	Atmos,
    #[strum(serialize = "DDP5.1")]
	DDP51,
    #[strum(serialize = "DTSHD")]
    DTSHD,
    #[strum(serialize = "DTSX")]
    DTSX,
    #[strum(serialize = "DTS")]
    DTS,
    #[strum(serialize = "AC35.1")]
    AC351,
    #[strum(default)]
    Custom(String),
    #[default]
    Unknown,
}


impl RsAudio {
    pub fn from_filename(filename: &str) -> Self {
        let modified_filename = filename.replace(" ", ".").replace("-", ".").replace("_", ".").to_lowercase();
        if text_contains(&modified_filename, "atmos") {
            RsAudio::Atmos
        } else if text_contains(&modified_filename, "ddp5.1") {
            RsAudio::DDP51
        } else if text_contains(&modified_filename, "dtshd") {
            RsAudio::DTSHD
        } else if text_contains(&modified_filename, "dtsx") {
            RsAudio::DTSX
        } else if text_contains(&modified_filename, "dts") {
            RsAudio::DTS
        } else if text_contains(&modified_filename, "ac35.1") || text_contains(&modified_filename, "ac3.5.1") {
            RsAudio::AC351
        } else {
            RsAudio::Unknown
        }
    }

    pub fn list_from_filename(filename: &str) -> Vec<Self> {
        let mut result = vec![];
        let modified_filename = filename.replace(" ", ".").replace("-", ".").replace("_", ".").to_lowercase();
        if text_contains(&modified_filename, "atmos") {
            result.push(RsAudio::Atmos);
        } 
        if text_contains(&modified_filename, "ddp5.1") {
            result.push(RsAudio::DDP51);
        } 
        if text_contains(&modified_filename, "dtshd") {
            result.push(RsAudio::DTSHD);
        } 
        if text_contains(&modified_filename, "dtsx") {
            result.push(RsAudio::DTSX);
        } 
        if text_contains(&modified_filename, "dts") {
            result.push(RsAudio::DTS);
        } 
        if text_contains(&modified_filename, "ac35.1") || text_contains(&modified_filename, "ac3.5.1") {
            result.push(RsAudio::AC351);
        }
        result
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum PluginType {
	ImageClassification,
    UrlParser,
    Request,
    Lookup,
    #[default]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum CredentialType {
	Url,
	Password,
    Oauth,
    #[default]
    Token,
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
    pub expires: Option<u64>,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default)]
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


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn resolution_parsing() {
        assert_eq!(RsResolution::from_filename("Test.2024.S01E01.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"), RsResolution::FullHD);
        assert_eq!(RsResolution::from_filename("Test.2024.S01E01_720p VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"), RsResolution::HD);  
        assert_eq!(RsResolution::from_filename("TestIn4k.2024.S01E01_VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"), RsResolution::Unknown);
        assert_eq!(RsResolution::from_filename("TestIn4k.2024.S01E01_4K_VOSTFR.DSNP.WEB-DL.DDP5.Atmos.1.H.264"), RsResolution::UHD);
    }


    #[test]
    fn resolution_string() {
        assert_eq!("4K", RsResolution::UHD.to_string());
        assert_eq!(RsResolution::from_str("1080p").unwrap(), RsResolution::FullHD);
        assert_eq!(RsResolution::from_str("erzr").unwrap(), RsResolution::Custom("erzr".to_owned()));
    }
    #[test]
    fn audio_parsing() {
        assert_eq!(RsAudio::from_filename("Test.2024.S01E01.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"), RsAudio::DDP51);
        assert_eq!(RsAudio::from_filename("Test.2024.S01E01_720p VOSTFR.DSNP.WEB-DL.DDP5.1._atmos_H.264"), RsAudio::Atmos);  
        let list = RsAudio::list_from_filename("TestIn4k.2024.S01E01_4K_VOSTFR.DSNP.WEB-DL.DDP5.1.Atmos.H.264");
        assert_eq!(list.len(), 2);
        assert!(list.contains(&RsAudio::Atmos));
        assert!(list.contains(&RsAudio::DDP51));
    }

    #[test]
    fn videocodec_parsing() {
        assert_eq!(RsVideoCodec::from_filename("Test.2024.S01E01.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"), RsVideoCodec::H264);
        assert_eq!(RsVideoCodec::from_filename("Test.2024.S01E01_720p VOSTFR.DSNP.WEB-DL.DDP5.1.HEVC"), RsVideoCodec::X265);  
        assert_eq!(RsVideoCodec::from_filename("TestIn4k.2024.S01E01_VOSTFR.DSNP.WEB-DL.DDP5.1.X.265"), RsVideoCodec::X265);
        
    }
}
