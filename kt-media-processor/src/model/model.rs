use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils;

// #[derive(Serialize, Deserialize)]
// pub struct MediaRoot {
//     pub albums: HashMap<String, MediaAlbum>,
// }

#[derive(Serialize, Deserialize)]
pub struct MediaAlbumMeta {
    pub title: Option<String>,
    pub ordinal: Option<i32>,
    #[serde(
        serialize_with = "utils::serialize_dt",
        deserialize_with = "utils::deserialize_dt"
    )]
    pub last_modified: DateTime<Utc>,
    pub sub_albums: HashMap<String, MediaAlbum>,
    pub media_files: HashMap<String, MediaFile>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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
