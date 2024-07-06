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
        let media_album = Rc::new(model::MediaAlbum {
            path: media_album_meta_path.clone(),
            title: media_album_meta
                .borrow()
                .title
                .clone()
                .unwrap_or_else(|| "Unknown".to_owned()),
            ordinal: media_album_meta.borrow().ordinal.unwrap_or(-1),
            last_modified: media_album_meta
                .borrow()
                .last_modified
                .unwrap_or_else(|| Utc::now()),
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
