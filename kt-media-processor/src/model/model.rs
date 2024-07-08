use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;

use crate::utils;

// #[derive(Serialize, Deserialize)]
// pub struct MediaRoot {
//     pub albums: HashMap<String, MediaAlbum>,
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaAlbumMeta {
    #[serde(default)]
    pub title: Option<String>,
    pub ordinal: Option<i32>,

    #[serde(
        serialize_with = "utils::serialize_dt_opt",
        deserialize_with = "utils::deserialize_dt_opt",
        default
    )]
    pub last_modified_override: Option<DateTime<Utc>>,

    #[serde(
        serialize_with = "utils::serialize_dt",
        deserialize_with = "utils::deserialize_dt",
        default
    )]
    pub last_modified_dir: DateTime<Utc>,

    #[serde(default)]
    pub sub_albums: HashMap<String, Rc<RefCell<MediaAlbumMeta>>>,
    #[serde(default)]
    pub media_files: HashMap<String, Rc<RefCell<MediaFileMeta>>>,
}

pub enum Encoding {
    JPEG,
    AVIF,
    WEBP,
}

pub struct MediaEncodingRequest {
    pub encoding: Encoding,
    pub width: u32,
    pub height: u32,
    pub keep_aspect: bool, // if fa, image will be cropped to center
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaFileMeta {
    pub title: String,
    pub ordinal: i32,
    #[serde(
        serialize_with = "utils::serialize_dt_opt",
        deserialize_with = "utils::deserialize_dt_opt",
        default
    )]
    pub last_modified_override: Option<DateTime<Utc>>,

    #[serde(
        serialize_with = "utils::serialize_dt",
        deserialize_with = "utils::deserialize_dt",
        default
    )]
    pub last_modified_file: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaAlbum {
    pub path: String,
    pub title: String,
    pub ordinal: i32,
    #[serde(
        serialize_with = "utils::serialize_dt",
        deserialize_with = "utils::deserialize_dt"
    )]
    pub last_modified_dir: DateTime<Utc>,
    pub sub_albums: HashMap<String, Rc<RefCell<MediaAlbum>>>,
    pub media_files: HashMap<String, Rc<RefCell<MediaFile>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaFile {
    pub path: String,
    pub title: String,
    pub ordinal: i32,
    #[serde(
        serialize_with = "utils::serialize_dt",
        deserialize_with = "utils::deserialize_dt"
    )]
    pub last_modified: DateTime<Utc>,
    pub width: u32,
    pub height: u32,
    pub variants: HashMap<String, Rc<RefCell<MediaFileVariant>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaFileVariant {
    pub path: String,
    pub mime_type: String,
    pub width: u32,
    pub height: u32,
    pub bytes: u32,
    pub is_thumbnail: bool,
}
