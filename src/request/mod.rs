use std::{collections::HashMap, str::FromStr};

use crate::domain::media::{FileEpisode, Media, MediaForUpdate};
use crate::lookup::RsLookupMatchType;
use crate::{CustomParamTypes, PluginCredential, RsFileType, RsVideoFormat};
use crate::{RsAudio, RsResolution, RsVideoCodec};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;
use urlencoding::decode;

pub mod error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsCookie {
    pub domain: String,
    pub http_only: bool,
    pub path: String,
    pub secure: bool,
    pub expiration: Option<f64>,
    pub name: String,
    pub value: String,
}

impl FromStr for RsCookie {
    type Err = error::RequestError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        //let [domain, httpOnly, path, secure, expiration, name, value ] = line.split(';');
        let mut splitted = line.split(';');
        Ok(RsCookie {
            domain: splitted
                .next()
                .ok_or(error::RequestError::UnableToParseCookieString(
                    "domain".to_owned(),
                    line.to_owned(),
                ))?
                .to_owned(),
            http_only: "true"
                == splitted
                    .next()
                    .ok_or(error::RequestError::UnableToParseCookieString(
                        "http_only".to_owned(),
                        line.to_owned(),
                    ))?,
            path: splitted
                .next()
                .ok_or(error::RequestError::UnableToParseCookieString(
                    "path".to_owned(),
                    line.to_owned(),
                ))?
                .to_owned(),
            secure: "true"
                == splitted
                    .next()
                    .ok_or(error::RequestError::UnableToParseCookieString(
                        "secure".to_owned(),
                        line.to_owned(),
                    ))?,
            expiration: {
                let t = splitted
                    .next()
                    .ok_or(error::RequestError::UnableToParseCookieString(
                        "expiration".to_owned(),
                        line.to_owned(),
                    ))?
                    .to_owned();
                if t.is_empty() {
                    None
                } else {
                    Some(t.parse().map_err(|_| {
                        error::RequestError::UnableToParseCookieString(
                            "expiration parsing".to_owned(),
                            line.to_owned(),
                        )
                    })?)
                }
            },
            name: splitted
                .next()
                .ok_or(error::RequestError::UnableToParseCookieString(
                    "name".to_owned(),
                    line.to_owned(),
                ))?
                .to_owned(),
            value: splitted
                .next()
                .ok_or(error::RequestError::UnableToParseCookieString(
                    "value".to_owned(),
                    line.to_owned(),
                ))?
                .to_owned(),
        })
    }
}

impl RsCookie {
    pub fn netscape(&self) -> String {
        let second = if self.domain.starts_with('.') {
            "TRUE"
        } else {
            "FALSE"
        };
        let secure = if self.secure { "TRUE" } else { "FALSE" };
        let expiration = if let Some(expiration) = self.expiration {
            (expiration as u32).to_string()
        } else {
            "".to_owned()
        };
        //return [domain, domain.startsWith('.') ? 'TRUE' : 'FALSE', path, secure ? 'TRUE' : 'FALSE', expiration.split('.')[0], name, value].join('\t')
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.domain, second, self.path, secure, expiration, self.name, self.value
        )
    }

    pub fn header(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

pub trait RsCookies {
    fn header_value(&self) -> String;
    fn headers(&self) -> (String, String);
}

impl RsCookies for Vec<RsCookie> {
    fn header_value(&self) -> String {
        self.iter()
            .map(|t| t.header())
            .collect::<Vec<String>>()
            .join("; ")
    }

    fn headers(&self) -> (String, String) {
        (
            "cookie".to_owned(),
            self.iter()
                .map(|t| t.header())
                .collect::<Vec<String>>()
                .join("; "),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsRequest {
    pub upload_id: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default)]
    pub status: RsRequestStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,

    /// If true this request can be saved for later use and will remain valid
    /// If Permanent is true but status is intermediate the process will go through request plugins to try to get a permanant link
    #[serde(default)]
    pub permanent: bool,
    /// If true can be played/downloaded instantly (streamable link, no need to add to service first)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant: Option<bool>,

    pub json_body: Option<Value>,
    #[serde(default)]
    pub method: RsRequestMethod,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<(String, String)>>,
    /// some downloader like YTDL require detailed cookies. You can create Header equivalent  with `headers` fn on the vector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<RsCookie>>,
    /// If must choose between multiple files. Recall plugin with a `selected_file` containing one of the name in this list to get link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<RsRequestFiles>>,
    /// one of the `files` selected for download
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book: Option<FileEpisode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<RsResolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_format: Option<RsVideoFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videocodec: Option<RsVideoCodec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<RsAudio>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<u64>,

    #[serde(default)]
    pub ignore_origin_duplicate: bool,

    // Download-specific fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<RsFileType>,

    // Lookup fields (text to search in database, NOT IDs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_lookup: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_lookup: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums_lookup: Option<Vec<String>>,
}

impl RsRequest {
    pub fn set_cookies(&mut self, cookies: Vec<RsCookie>) {
        let mut existing = if let Some(headers) = &self.headers {
            headers.to_owned()
        } else {
            vec![]
        };
        existing.push(cookies.headers());
        self.headers = Some(existing);
    }

    pub fn filename_or_extract_from_url(&self) -> Option<String> {
        if self.filename.is_some() {
            self.filename.clone()
        } else {
            self.url
                .split('/')
                .last()
                .and_then(|segment| {
                    segment
                        .split('?')
                        .next()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                })
                .and_then(|potential| {
                    let extension = potential
                        .split('.')
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>();
                    if extension.len() > 1
                        && extension.last().unwrap_or(&"".to_string()).len() > 2
                        && extension.last().unwrap_or(&"".to_string()).len() < 5
                    {
                        let decoded = decode(&potential)
                            .map(|x| x.into_owned())
                            .unwrap_or(potential); // Decodes the URL
                        Some(decoded)
                    } else {
                        None
                    }
                })
        }
    }

    pub fn parse_filename(&mut self) {
        if let Some(filename) = &self.filename {
            let resolution = RsResolution::from_filename(filename);
            if resolution != RsResolution::Unknown {
                self.resolution = Some(resolution);
            }
            let video_format = RsVideoFormat::from_filename(filename);
            if video_format != RsVideoFormat::Other {
                self.video_format = Some(video_format);
            }
            let videocodec = RsVideoCodec::from_filename(filename);
            if videocodec != RsVideoCodec::Unknown {
                self.videocodec = Some(videocodec);
            }
            let audio = RsAudio::list_from_filename(filename);
            if !audio.is_empty() {
                self.audio = Some(audio);
            }

            let re = Regex::new(r"(?i)s(\d+)e(\d+)").unwrap();
            if let Some(caps) = re.captures(filename) {
                self.season = caps[1].parse::<u32>().ok();
                self.episode = caps[2].parse::<u32>().ok();
            }
        }
    }

    pub fn parse_subfilenames(&mut self) {
        if let Some(ref mut files) = self.files {
            for file in files {
                file.parse_filename();
            }
        }
    }
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum RsRequestStatus {
    /// No plugin yet processed this request
    #[default]
    Unprocessed,
    /// All plugin processed but with no result
    Processed,
    ///if remain in this state after all plugin it will go through YtDl to try to extract medias
    NeedParsing,
    /// Link can be processed but first need to be added to the service and downloaded
    ///   -First call this plugin again with `add` method
    ///   -Check status and once ready call `process` again
    RequireAdd,
    /// Modified but need a second pass of plugins
    Intermediate,
    /// Multiple files found, current plugin need to be recalled with a `selected_file``
    NeedFileSelection,
    /// `url` is ready but should be proxied by the server as it contains sensitive informations (like token)
    FinalPrivate,
    /// `url` is ready and can be directly sent to _any_ user directly (using redirect)
    FinalPublic,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum RsRequestMethod {
    #[default]
    Get,
    Post,
    Patch,
    Delete,
    Head,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsRequestFiles {
    pub name: String,
    pub size: u64,

    pub mime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<RsResolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_format: Option<RsVideoFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videocodec: Option<RsVideoCodec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<RsAudio>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<u64>,

    // Lookup fields (text to search in database, NOT IDs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_lookup: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_lookup: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums_lookup: Option<Vec<String>>,
}

impl RsRequestFiles {
    pub fn parse_filename(&mut self) {
        let resolution = RsResolution::from_filename(&self.name);
        if resolution != RsResolution::Unknown {
            self.resolution = Some(resolution);
        }
        let video_format = RsVideoFormat::from_filename(&self.name);
        if video_format != RsVideoFormat::Other {
            self.video_format = Some(video_format);
        }
        let videocodec = RsVideoCodec::from_filename(&self.name);
        if videocodec != RsVideoCodec::Unknown {
            self.videocodec = Some(videocodec);
        }
        let audio = RsAudio::list_from_filename(&self.name);
        if !audio.is_empty() {
            self.audio = Some(audio);
        }

        let re = Regex::new(r"(?i)s(\d+)e(\d+)").unwrap();
        if let Some(caps) = re.captures(&self.name) {
            self.season = caps[1].parse::<u32>().ok();
            self.episode = caps[2].parse::<u32>().ok();
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsRequestPluginRequest {
    pub request: RsRequest,
    pub credential: Option<PluginCredential>,
    pub params: Option<HashMap<String, CustomParamTypes>>,
}

/// Groups multiple download requests together, optionally combining them into a single media item
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsGroupDownload {
    /// If true, all requests will be grouped into a single media item (album)
    #[serde(default)]
    pub group: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_mime: Option<String>,
    pub requests: Vec<RsRequest>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub infos: Option<MediaForUpdate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<RsLookupMatchType>,
}

/// Status of a processing task added via request_add
#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display, EnumString, Default,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum RsProcessingStatus {
    #[default]
    Pending,
    Processing,
    Finished,
    Error,
    Paused,
}

/// Response from request_add plugin method
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsRequestAddResponse {
    /// Processing ID returned by the plugin service
    pub processing_id: String,
    /// Initial status
    #[serde(default)]
    pub status: RsProcessingStatus,
    /// Relative ETA in milliseconds until completion (host converts to absolute timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eta: Option<i64>,
}

/// Response from get_progress plugin method
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsProcessingProgress {
    /// Processing ID
    pub processing_id: String,
    /// Progress percentage (0-100)
    pub progress: u32,
    /// Current status
    pub status: RsProcessingStatus,
    /// Error message if status is Error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Relative ETA in milliseconds until completion (host converts to absolute timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eta: Option<i64>,
    /// Updated request with final URL when finished
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<Box<RsRequest>>,
}

/// Request for pause/remove/get_progress plugin methods
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RsProcessingActionRequest {
    /// Processing ID to act on
    pub processing_id: String,
    /// Credential for the plugin
    pub credential: Option<PluginCredential>,
    /// Optional params
    pub params: Option<HashMap<String, CustomParamTypes>>,
}

#[cfg(test)]
mod tests {

    use self::error::RequestError;

    use super::*;

    #[test]
    fn test_cookie_parsing() -> Result<(), RequestError> {
        let parsed = RsCookie::from_str(".twitter.com;false;/;true;1722364794.437907;kdt;w1j")?;
        assert!(parsed.domain == ".twitter.com".to_owned());
        assert!(parsed.http_only == false);
        assert!(parsed.path == "/".to_owned());
        assert!(parsed.secure == true);
        assert!(parsed.expiration == Some(1722364794.437907));
        assert!(parsed.name == "kdt".to_owned());
        assert!(parsed.value == "w1j".to_owned());
        Ok(())
    }

    #[test]
    fn test_cookie_parsing_no_expi() -> Result<(), RequestError> {
        let parsed = RsCookie::from_str(".twitter.com;false;/;true;;kdt;w1j")?;
        assert!(parsed.domain == ".twitter.com".to_owned());
        assert!(parsed.http_only == false);
        assert!(parsed.path == "/".to_owned());
        assert!(parsed.secure == true);
        assert!(parsed.expiration == None);
        assert!(parsed.name == "kdt".to_owned());
        assert!(parsed.value == "w1j".to_owned());
        Ok(())
    }

    #[test]
    fn test_netscape() -> Result<(), RequestError> {
        let parsed = RsCookie::from_str(".twitter.com;false;/;true;1722364794.437907;kdt;w1j")?;
        assert!(parsed.netscape() == ".twitter.com\tTRUE\t/\tTRUE\t1722364794\tkdt\tw1j");
        Ok(())
    }
    #[test]
    fn test_netscape_doublequote() -> Result<(), RequestError> {
        let parsed = RsCookie::from_str(
            ".twitter.com;true;/;true;1726506480.700665;ads_prefs;\"HBESAAA=\"",
        )?;
        assert!(
            parsed.netscape() == ".twitter.com\tTRUE\t/\tTRUE\t1726506480\tads_prefs\t\"HBESAAA=\""
        );
        Ok(())
    }

    #[test]
    fn test_parse_filename() -> Result<(), RequestError> {
        let req = RsRequest {
            url: "http://www.test.com/filename.mp4?toto=3".to_string(),
            filename: Some("test.mkv".to_owned()),
            ..Default::default()
        };
        assert_eq!(
            req.filename_or_extract_from_url(),
            Some("test.mkv".to_string())
        );
        let req = RsRequest {
            url: "http://www.test.com/filename.mp4?toto=3".to_string(),
            ..Default::default()
        };
        assert_eq!(
            req.filename_or_extract_from_url(),
            Some("filename.mp4".to_string()),
            "We are expecting a filename from the url"
        );
        let req = RsRequest {
            url: "http://www.test.com/notfilename?toto=3".to_string(),
            ..Default::default()
        };
        assert_eq!(
            req.filename_or_extract_from_url(),
            None,
            "Should return none as there is no filename with extensiopn in url"
        );
        let req = RsRequest {
            url: "http://www.test.com/notfilename.toolong?toto=3".to_string(),
            ..Default::default()
        };
        assert_eq!(
            req.filename_or_extract_from_url(),
            None,
            "Should return none as too long after dot is not an extension"
        );
        let req = RsRequest {
            url: "http://www.test.com/filename%20test.mp4?toto=3".to_string(),
            ..Default::default()
        };
        assert_eq!(
            req.filename_or_extract_from_url(),
            Some("filename test.mp4".to_string()),
            "Should decode URL-encoded filename"
        );
        Ok(())
    }

    #[test]
    fn test_header() -> Result<(), RequestError> {
        let parsed = vec![
            RsCookie::from_str(
                ".twitter.com;true;/;true;1726506480.700665;ads_prefs;\"HBESAAA=\"",
            )?,
            RsCookie::from_str(".twitter.com;false;/;true;1722364794.437907;kdt;w1j")?,
        ];
        println!("header: {}", parsed.header_value());
        assert!(parsed.header_value() == "ads_prefs=\"HBESAAA=\"; kdt=w1j");
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(), RequestError> {
        let mut req = RsRequest {
            filename: Some(
                "Shogun.2024.S01E01.Anjin.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264-NTb.mkv".to_owned(),
            ),
            ..Default::default()
        };
        req.parse_filename();
        assert_eq!(req.season.unwrap(), 1);
        assert_eq!(req.episode.unwrap(), 1);
        assert_eq!(req.resolution.unwrap(), RsResolution::FullHD);
        assert_eq!(req.videocodec.unwrap(), RsVideoCodec::H264);
        assert_eq!(req.video_format.unwrap(), RsVideoFormat::Mkv);
        assert_eq!(req.audio.unwrap().len(), 1);
        Ok(())
    }

    #[test]
    fn test_parse2() -> Result<(), RequestError> {
        let mut req = RsRequest {
            filename: Some("Shogun.2024.S01E05.MULTi.HDR.DV.2160p.WEB.H265-FW".to_owned()),
            ..Default::default()
        };
        req.parse_filename();
        assert_eq!(req.season.expect("a season"), 1);
        assert_eq!(req.episode.expect("an episode"), 5);
        assert_eq!(req.resolution.expect("a resolution"), RsResolution::UHD);
        assert_eq!(req.videocodec.expect("a videocodec"), RsVideoCodec::H265);

        Ok(())
    }
}
