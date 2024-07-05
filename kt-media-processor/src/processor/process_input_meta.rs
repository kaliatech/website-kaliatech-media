use crate::processor::ScanResult;

use chrono::Utc;

use std::collections::HashMap;
use std::rc::Rc;

use crate::model;

use std::error::Error;
use std::path::Path;

pub struct OutputResult {
    pub media_albums: HashMap<String, Rc<model::MediaAlbum>>,
    pub media_files: HashMap<String, Rc<model::MediaFile>>,
}

pub fn process_input_meta(
    input_path: &Path,
    output_path: &Path,
    scan_result: &ScanResult,
) -> Result<OutputResult, Box<dyn Error>> {
    let mut media_albums: HashMap<String, Rc<model::MediaAlbum>> = HashMap::new();
    let media_files: HashMap<String, Rc<model::MediaFile>> = HashMap::new();

    let media_album_metas = &scan_result.media_album_metas;
    let media_file_metas = &scan_result.media_file_metas;
    for (media_album_meta_path, media_album_meta) in &*media_album_metas {
        // let media_album_json_file_meta = media_file_metas
        //     .get(format!("{}{}", media_album_meta_path, "album-meta.json").as_str());

        // // Default title to the scanned dir name
        // let mut title = media_album_meta
        //     .title
        //     .as_ref()
        //     //.unwrap_or(&String::default())
        //     .unwrap_or(&String::default())
        //     .clone();

        // // Override title with album-meta.json title if present
        // if let Some(media_album_meta_json) = media_album_json_file_meta {
        //     if let Some(title_value) = media_album_meta_json.title {
        //         title = title_value.clone();
        //     }
        // }

        let title = media_album_meta
            .title
            .as_ref()
            .unwrap_or_else(|| media_album_meta_path);

        let media_album = Rc::new(model::MediaAlbum {
            path: media_album_meta_path.clone(),
            title: title.clone(),
            ordinal: media_album_meta.ordinal.unwrap_or(-1),
            last_modified: media_album_meta.last_modified.unwrap_or_else(|| Utc::now()),
            sub_albums: HashMap::new(),
            media_files: HashMap::new(),
        });
        media_albums.insert(media_album_meta_path.clone(), media_album);
    }
    // for (key, value) in &*scan_result.media_album_metas {
    //     println!("{} / {}", key, value);
    // }
    // media_albums.clear();

    let output_result = OutputResult {
        media_albums: media_albums,
        media_files: media_files,
    };

    return Ok(output_result);
}
