use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::RsRequest;

#[cfg(feature = "rusqlite")]
pub mod rusqlite;

fn text_contains(text: &str, contains: &str) -> bool {
    text.contains(&format!(".{}.", contains)) || text.starts_with(contains) || text.ends_with(contains)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, strum_macros::EnumString, Default)]
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
    #[default]
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
    XVID,
    #[strum(default)]
    Custom(String),
    #[default]
    Unknown,
}

impl RsVideoCodec {
    pub fn from_filename(filename: &str) -> Self {
        let modified_filename = filename.replace([' ', '-', '_'], ".").to_lowercase();
        if text_contains(&modified_filename, "x265") || text_contains(&modified_filename, "x.265") || text_contains(&modified_filename, "hevc") || text_contains(&modified_filename, "h265")  || text_contains(&modified_filename, "h.265") {
            RsVideoCodec::H265
        } else if text_contains(&modified_filename, "h264")|| text_contains(&modified_filename, "h.264") || text_contains(&modified_filename, "x.264")  || text_contains(&modified_filename, "x264"){
            RsVideoCodec::H264
        } else if text_contains(&modified_filename, "av1"){
            RsVideoCodec::AV1
        } else if text_contains(&modified_filename, "xvid"){
            RsVideoCodec::XVID
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
    #[strum(serialize = "AAC5.1")]
    AAC51,
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
        } else if text_contains(&modified_filename, "ddp5.1") || text_contains(&modified_filename, "ddp51") || text_contains(&modified_filename, "dolby.digital.plus.5.1") || text_contains(&modified_filename, "dd51") {
            RsAudio::DDP51
        } else if text_contains(&modified_filename, "dtshd") {
            RsAudio::DTSHD
        } else if text_contains(&modified_filename, "dtsx") {
            RsAudio::DTSX
        } else if text_contains(&modified_filename, "dts") {
            RsAudio::DTS
        } else if text_contains(&modified_filename, "ac35.1") || text_contains(&modified_filename, "ac3.5.1") {
            RsAudio::AC351
        } else if text_contains(&modified_filename, "aac5.1") || text_contains(&modified_filename, "aac51") {
            RsAudio::AAC51
        } else if text_contains(&modified_filename, "aac") {
            RsAudio::AAC
        } else if text_contains(&modified_filename, "mp3") {
            RsAudio::MP3
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



#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum VideoOverlayPosition {
	TopLeft,
    #[default]
    TopRight,
    BottomLeft,
    BottomRight,
    BottomCenter,
    TopCenter,
    Center
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum VideoAlignment {
    #[default]
    Center,
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum VideoOverlayType {
    #[default]
	Watermark,
    
    File,
}

impl VideoOverlayPosition {
    pub fn as_filter(&self, margin: f64) -> String {
        match self {
            VideoOverlayPosition::TopLeft => format!("main_w*{}:main_h*{}",margin, margin),
            VideoOverlayPosition::TopRight => format!("(main_w-w):min(main_h,main_w)*{}", margin),
            VideoOverlayPosition::BottomLeft => format!("main_w*{}:(main_h-h)", margin),
            VideoOverlayPosition::BottomRight => "(main_w-w):(main_h-h)".to_string(),
            VideoOverlayPosition::BottomCenter => format!("main_w*{}:(main_h-h)", margin),//TODO
            VideoOverlayPosition::TopCenter => format!("main_w*{}:main_h*{}",margin, margin), //TODO
            VideoOverlayPosition::Center => format!("main_w*{}:main_h*{}",margin, margin), //TODO
        }
    }
    pub fn as_ass_alignment(&self) -> String {
        match self {
            VideoOverlayPosition::TopLeft => String::from("7"),
            VideoOverlayPosition::TopCenter => String::from("8"),
            VideoOverlayPosition::TopRight => String::from("9"),
            VideoOverlayPosition::Center => String::from("5"),
            VideoOverlayPosition::BottomLeft => String::from("1"),
            VideoOverlayPosition::BottomCenter => String::from("2"),
            VideoOverlayPosition::BottomRight => String::from("3"),
        }
    }


}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct VideoConvertInterval {
    start: f64,
    duration: Option<f64>,
    /// will default to current input
    input: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]

pub struct VideoOverlay {
    #[serde(rename = "type")]
    pub kind: VideoOverlayType,
    pub path: String,
    #[serde(default)]
    pub position: VideoOverlayPosition,
    pub margin: Option<f64>,
    pub ratio: f32,
    pub opacity: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct VideoTextOverlay {
    pub text: String,
    pub font_color: Option<String>,
    pub font: Option<String>,
    #[serde(default)]
    pub position: VideoOverlayPosition,
    pub margin_vertical: Option<i32>,
    pub margin_horizontal: Option<i32>,
    pub margin_right: Option<i32>,
    pub margin_bottom: Option<i32>,
    pub font_size: i32,
    pub opacity: Option<f32>,
    pub shadow_color: Option<String>,
    pub shado_opacity: Option<f32>,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct VideoConvertRequest {
    pub id: String,
    pub format: RsVideoFormat,
    pub codec: Option<RsVideoCodec>,
    pub crf: Option<u16>,
    #[serde(default)]
    pub no_audio: bool,
    pub width: Option<String>,
    pub height: Option<String>,
    pub framerate: Option<u16>,
    pub crop_width: Option<u16>,
    pub crop_height: Option<u16>,
    pub aspect_ratio: Option<String>,
    pub aspect_ratio_alignment: Option<VideoAlignment>,
    pub overlay: Option<VideoOverlay>,
    pub texts: Option<Vec<VideoTextOverlay>>,
    #[serde(default)]
    pub intervals: Vec<VideoConvertInterval>,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsVideoTranscodeJob {
    pub source: RsRequest,
    pub request: VideoConvertRequest,
    pub status: RsVideoTranscodeStatus,
    pub progress: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, strum_macros::EnumString, Default)]
#[serde(rename_all = "lowercase")] 
#[strum(serialize_all = "lowercase")]
pub enum RsVideoTranscodeStatus {
    #[default]
    Pending,
	Downloading,
    Queued,
    Processing,
    Completed,
    Failed,
    Canceled,
}
