use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;

#[cfg(feature = "rusqlite")]
pub mod rusqlite;

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum Resolution {
    #[strum(serialize = "4K")]
	FourK,
    #[strum(serialize = "1080p")]
    FullHD,
    #[strum(serialize = "720p")]
    HD,
    Custom(String),
    Other,
    #[default]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum RsResolution {
    #[strum(serialize = "4K")]
	FourK,
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
        if modified_filename.contains(".1080p.") || modified_filename.starts_with("1080p") || modified_filename.ends_with("1080p") {
            RsResolution::FullHD
        } else if modified_filename.contains(".720p.") || modified_filename.starts_with("720p") || modified_filename.ends_with("720p") {
            RsResolution::HD
        } else if modified_filename.contains(".720p.") || modified_filename.starts_with("720p") || modified_filename.ends_with("720p") {
            RsResolution::FullHD
        }  else if modified_filename.contains(".4K.") || modified_filename.starts_with("4K") || modified_filename.ends_with("4K") {
            RsResolution::FourK
        } else {
            RsResolution::Unknown
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
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
        if modified_filename.contains(".atmos.") || modified_filename.starts_with("atmos") || modified_filename.ends_with("atmos") {
            RsAudio::Atmos
        } else if modified_filename.contains(".ddp5.1.") || modified_filename.starts_with("ddp5.1") || modified_filename.ends_with("ddp5.1") {
            RsAudio::DDP51
        } else if modified_filename.contains(".dtshd.") || modified_filename.starts_with("dtshd") || modified_filename.ends_with("dtshd") {
            RsAudio::DTSHD
        } else if modified_filename.contains(".dtsx.") || modified_filename.starts_with("dtsx") || modified_filename.ends_with("dtsx") {
            RsAudio::DTSX
        } else if modified_filename.contains(".dts.") || modified_filename.starts_with("dts") || modified_filename.ends_with("dts") {
            RsAudio::DTS
        } else if modified_filename.contains(".ac35.1.") || modified_filename.starts_with("ac35.1") || modified_filename.ends_with("ac35.1") {
            RsAudio::AC351
        } else {
            RsAudio::Unknown
        }
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


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn resolution_parsing() {
        assert_eq!(RsResolution::from_filename("Test.2024.S01E01.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"), RsResolution::FullHD);
        assert_eq!(RsResolution::from_filename("Test.2024.S01E01_720p VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"), RsResolution::HD);  
        assert_eq!(RsResolution::from_filename("TestIn4k.2024.S01E01_VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"), RsResolution::Unknown);
        assert_eq!(RsResolution::from_filename("TestIn4k.2024.S01E01_4K_VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"), RsResolution::Unknown);
    }

    #[test]
    fn resolution_string() {
        assert_eq!("4K", RsResolution::FourK.to_string());
        assert_eq!(RsResolution::from_str("1080p").unwrap(), RsResolution::FullHD);
        assert_eq!(RsResolution::from_str("erzr").unwrap(), RsResolution::Custom("erzr".to_owned()));
    }
    #[test]
    fn audio_parsing() {
        assert_eq!(RsAudio::from_filename("Test.2024.S01E01.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264"), RsAudio::DDP51);
        assert_eq!(RsAudio::from_filename("Test.2024.S01E01_720p VOSTFR.DSNP.WEB-DL.DDP5.1._atmos_H.264"), RsAudio::Atmos);  
    }
}
