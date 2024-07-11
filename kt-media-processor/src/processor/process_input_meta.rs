use indexmap::IndexMap;

use crate::scanner::ScanResult;

use std::cell::RefCell;

use std::rc::Rc;

use crate::model;
use crate::processor;

use std::error::Error;
use std::fs;
use std::path::Path;
use crate::utils::glob_files;

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
    let mut media_files: IndexMap<String, Rc<RefCell<model::MediaFile>>> = IndexMap::new();

    let media_album_metas = &scan_result.media_album_metas;
    let _media_file_metas = &scan_result.media_file_metas;

    if let Some(media_album_meta) = media_album_metas.get("") {
        let root_media_album =
            processor::process_album(&mut media_albums, &mut media_files, input_path, output_path, &String::from(""), media_album_meta);
        match root_media_album {
            Ok(root_media_album) => {
                media_albums.insert(String::from(""), Rc::clone(&root_media_album));
            }
            Err(e) => {
                eprintln!("Error processing root media album: {}", e);
                // Decide on further actions here, e.g., continue, return an error, etc.
                // For example, to stop processing and return an error:
                return Err(e);
            }
        }
    }

    //TODO: Now iterate through output files and remove any files and albums (directories) that
    //      are not in the processed output.
    let output_files = glob_files(output_path);
    for file in output_files {
        let sub_path = format!("/{}", file.strip_prefix(output_path).unwrap().to_str().unwrap());
        if file.is_dir() {
            if !media_albums.contains_key(&sub_path) &&
                !media_files.contains_key(&sub_path) {
                println!("Remove dir: {:?}", sub_path);
                fs::remove_dir_all(file)?;
            }
        }
    }

    let output_result = OutputResult {
        media_albums: media_albums,
        media_files: media_files,
    };

    return Ok(output_result);
}
