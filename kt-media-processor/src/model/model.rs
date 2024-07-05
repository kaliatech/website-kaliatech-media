use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
    pub last_modified: Option<DateTime<Utc>>,
    #[serde(default)]
    pub sub_albums: HashMap<String, MediaAlbumMeta>,
    #[serde(default)]
    pub media_files: HashMap<String, Rc<MediaFileMeta>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaFileMeta {
    pub title: String,
    pub ordinal: i32,
    #[serde(
        serialize_with = "utils::serialize_dt",
        deserialize_with = "utils::deserialize_dt"
    )]
    pub last_modified: DateTime<Utc>,
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
    pub last_modified: DateTime<Utc>,
    pub sub_albums: HashMap<String, MediaAlbum>,
    pub media_files: HashMap<String, MediaFile>,
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
}
