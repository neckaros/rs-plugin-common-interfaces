use serde::{Deserialize, Serialize};
use strum_macros::EnumString;


fn text_contains(text: &str, contains: &str) -> bool {
    text.contains(&format!(".{}.", contains)) || text.starts_with(contains) || text.ends_with(contains)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, strum_macros::EnumString)]
#[serde(rename_all = "lowercase")] 
#[strum(serialize_all = "lowercase")]
pub enum RsVideoFormat {
	Mp4,
	M4v,
    Mov,
    Mkv,
    WebM,
    Wmv,
    Avi,
    Other
}

impl RsVideoFormat {

    pub fn from_filename(filename: &str) -> Self {
        let filename = filename.to_lowercase();
        if filename.ends_with(".mkv") {
            Self::Mkv
        } else if filename.ends_with(".mp4") {
            Self::Mp4
        } else if filename.ends_with(".m4v") {
            Self::M4v
        } else if filename.ends_with(".mov") {
            Self::Mov
        } else if filename.ends_with(".webm") {
            Self::WebM
        } else if filename.ends_with(".wmv") {
            Self::Wmv
        } else if filename.ends_with(".avi") {
            Self::Avi
        } else {
            Self::Other
        }
    }

    pub fn as_mime(&self) -> &str {
        match self {
            RsVideoFormat::Mp4 => "video/mp4",
            RsVideoFormat::M4v => "video/x-m4v",
            RsVideoFormat::Mov => "video/quicktime",
            RsVideoFormat::Mkv => "application/x-matroska",
            RsVideoFormat::WebM => "video/webm",
            RsVideoFormat::Wmv => "video/x-ms-wmv",
            RsVideoFormat::Avi => "video/x-msvideo",
            RsVideoFormat::Other => "application/octet-stream",
        }
    }
    pub fn from_mime(mime: &str) -> Option<Self> {
        match mime {
            "video/mp4" => Some(RsVideoFormat::Mp4),
            "video/x-m4v" => Some(RsVideoFormat::M4v),
            "video/quicktime" => Some(RsVideoFormat::Mov),
            "application/x-matroska" => Some(RsVideoFormat::Mkv),
            "video/webm" => Some(RsVideoFormat::WebM),
            "video/x-ms-wmv" => Some(RsVideoFormat::Wmv),
            "video/x-msvideo" => Some(RsVideoFormat::Avi),
            "application/octet-stream" => Some(RsVideoFormat::Other),
            _ => None
        }
    }
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
        let modified_filename = filename.replace([' ', '-', '_'], ".").to_lowercase();
        if text_contains(&modified_filename, "1080p") {
            RsResolution::FullHD
        } else if text_contains(&modified_filename, "720p") {
            RsResolution::HD
        } else if text_contains(&modified_filename, "4k") || text_contains(&modified_filename, "2160p") {
            RsResolution::UHD
        } else {
            RsResolution::Unknown
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
pub enum RsVideoCodec {
	H265,
    H264,
    AV1,
    #[strum(default)]
    Custom(String),
    #[default]
    Unknown,
}

impl RsVideoCodec {
    pub fn from_filename(filename: &str) -> Self {
        let modified_filename = filename.replace([' ', '-', '_'], ".").to_lowercase();
        if text_contains(&modified_filename, "x265") || text_contains(&modified_filename, "x.265") || text_contains(&modified_filename, "hevc") || text_contains(&modified_filename, "h265") {
            RsVideoCodec::H265
        } else if text_contains(&modified_filename, "h264")|| text_contains(&modified_filename, "h.264") || text_contains(&modified_filename, "x.264")  || text_contains(&modified_filename, "x264"){
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
    AAC,
    MP3,
    #[strum(default)]
    Custom(String),
    #[default]
    Unknown,
}


impl RsAudio {
    pub fn from_filename(filename: &str) -> Self {
        let modified_filename = filename.replace([' ', '-', '_'], ".").to_lowercase();
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
        let modified_filename = filename.replace([' ', '-', '_'], ".").to_lowercase();
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