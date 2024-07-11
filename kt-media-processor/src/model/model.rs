use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;

use indexmap::IndexMap;

use crate::utils;

// #[derive(Serialize, Deserialize)]
// pub struct MediaRoot {
//     pub albums: HashMap<String, MediaAlbum>,
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaAlbumSource {
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
    pub thumbnail: Option<String>,

    #[serde(default)]
    pub sub_albums: HashMap<String, Rc<RefCell<MediaAlbumSource>>>,

    #[serde(default)]
    pub media_files: HashMap<String, Rc<RefCell<MediaFileSource>>>,

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Encoding {
    JPEG,
    AVIF,
    WEBP,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaEncodingRequest {
    pub encoding: Encoding,
    pub width: u32,
    pub height: u32,
    pub keep_aspect: bool, // if fa, image will be cropped to center
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaFileSource {
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

    #[serde(default)]
    pub thumbnail: Option<String>,

    pub sub_albums: IndexMap<String, Rc<RefCell<MediaAlbum>>>,
    pub media_files: IndexMap<String, Rc<RefCell<MediaFile>>>,

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaFile {
    pub path: String,
    pub title: String,
    pub ordinal: i32,
    pub media_type: MediaFileType,
    #[serde(
        serialize_with = "utils::serialize_dt",
        deserialize_with = "utils::deserialize_dt"
    )]
    pub last_modified: DateTime<Utc>,
    pub width: u32,
    pub height: u32,
    pub variants: IndexMap<String, Rc<RefCell<MediaFileVariant>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaFileVariant {
    pub path: String,
    pub mime_type: String,
    pub width: u32,
    pub height: u32,
    pub duration: Option<f64>,
    pub bytes: u32,
    pub is_thumbnail: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MediaFileType {
    UNKNOWN,
    IMAGE,
    VIDEO,
    IMAGE360,
    VIDEO360,
}
