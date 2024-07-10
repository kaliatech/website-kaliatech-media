use indexmap::IndexMap;

use crate::scanner::ScanResult;

use std::cell::RefCell;

use std::rc::Rc;

use crate::model;
use crate::processor;

use std::error::Error;
use std::path::Path;

pub struct OutputResult {
    pub media_albums: IndexMap<String, Rc<RefCell<model::MediaAlbum>>>,
    pub media_files: IndexMap<String, Rc<RefCell<model::MediaFile>>>,
}

pub fn process_input_meta(
    input_path: &Path,
    output_path: &Path,
    scan_result: &ScanResult,
) -> Result<OutputResult, Box<dyn Error>> {
    let mut media_albums: IndexMap<String, Rc<RefCell<model::MediaAlbum>>> = IndexMap::new();
    let media_files: IndexMap<String, Rc<RefCell<model::MediaFile>>> = IndexMap::new();

    let media_album_metas = &scan_result.media_album_metas;
    let _media_file_metas = &scan_result.media_file_metas;

    if let Some(media_album_meta) = media_album_metas.get("") {
        let root_media_album =
            processor::process_album(input_path, output_path, &String::from(""), media_album_meta);
        match root_media_album {
            Ok(root_media_album) => {
                media_albums.insert(String::from(""), Rc::clone(&root_media_album));
            },
            Err(e) => {
                eprintln!("Error processing root media album: {}", e);
                // Decide on further actions here, e.g., continue, return an error, etc.
                // For example, to stop processing and return an error:
                return Err(e);
            }
        }
    }

    // for (media_album_meta_path, media_album_meta) in &*media_album_metas {
    //     let media_album = process_album(&media_album_meta_path, media_album_meta)?;
    //     media_albums.insert(media_album_meta_path.clone(), Rc::clone(&media_album));
    // }

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
