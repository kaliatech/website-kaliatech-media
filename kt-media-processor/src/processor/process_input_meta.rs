use crate::scanner::ScanResult;

use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;

use crate::model;

use std::error::Error;
use std::path::Path;

use std::fs;
use std::io::Write;

pub struct OutputResult {
    pub media_albums: HashMap<String, Rc<RefCell<model::MediaAlbum>>>,
    pub media_files: HashMap<String, Rc<RefCell<model::MediaFile>>>,
}

pub fn process_input_meta(
    input_path: &Path,
    output_path: &Path,
    scan_result: &ScanResult,
) -> Result<OutputResult, Box<dyn Error>> {
    let mut media_albums: HashMap<String, Rc<RefCell<model::MediaAlbum>>> = HashMap::new();
    let media_files: HashMap<String, Rc<RefCell<model::MediaFile>>> = HashMap::new();

    let media_album_metas = &scan_result.media_album_metas;
    let media_file_metas = &scan_result.media_file_metas;

    if let Some(media_album_meta) = media_album_metas.get("") {
        let root_media_album =
            process_album(input_path, output_path, &String::from(""), media_album_meta)?;
        media_albums.insert(String::from(""), Rc::clone(&root_media_album));
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

fn process_album(
    input_path: &Path,
    output_path: &Path,
    media_album_meta_path_str: &str,
    media_album_meta: &Rc<RefCell<model::MediaAlbumMeta>>,
) -> Result<Rc<RefCell<model::MediaAlbum>>, Box<dyn Error>> {
    let media_album_meta = media_album_meta.borrow();

    let media_album = Rc::new(RefCell::new(model::MediaAlbum {
        path: media_album_meta_path_str.to_string(),
        title: media_album_meta
            .title
            .clone()
            .unwrap_or_else(|| "Unknown".to_owned()),
        ordinal: media_album_meta.ordinal.unwrap_or(-1),
        last_modified_dir: media_album_meta.last_modified_dir,
        sub_albums: HashMap::new(),
        media_files: HashMap::new(),
    }));

    // Iterate all files of the album
    for (media_file_meta_path, media_file_meta) in &media_album_meta.media_files {
        let media_file_meta = media_file_meta.borrow();
        let media_file = Rc::new(RefCell::new(model::MediaFile {
            path: media_file_meta_path.clone(),
            title: media_file_meta.title.clone(),
            ordinal: media_file_meta.ordinal,
            last_modified: media_file_meta.last_modified_file,
            variants: HashMap::new(),
        }));

        // resize, generate thumbnails, etc
        let file_src_path = &input_path.join(
            &media_file_meta_path
                .strip_prefix("/")
                .unwrap_or(media_album_meta_path_str),
        );

        crate::processor::process_file(
            file_src_path,
            output_path,
            media_file_meta,
            media_file.borrow_mut(),
        )?;

        // insert to the album
        media_album
            .borrow_mut()
            .media_files
            .insert(media_file_meta_path.clone(), media_file);
    }
    for (sub_album_meta_path, sub_album_meta) in &media_album_meta.sub_albums {
        // Recursively process sub-albums
        let sub_media_album = process_album(
            input_path,
            output_path,
            &sub_album_meta_path,
            sub_album_meta,
        )?;
        let _ = &media_album
            .borrow_mut()
            .sub_albums
            .insert(sub_album_meta_path.clone(), Rc::clone(&sub_media_album));
    }

    //TODO: Only write a new album.json if the last_modified_dir is newer than existing album.json (if any) last modified

    // Write album.json
    let file_out_path = output_path
        .join(
            media_album_meta_path_str
                .strip_prefix("/")
                .unwrap_or(media_album_meta_path_str),
        )
        .join("album.json");

    // dbg!(&output_path);
    // dbg!(&file_out_path);

    if let Some(parent_dir) = file_out_path.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    }

    let mut file_out = fs::File::create(&file_out_path).expect("Failed to create album.json file");

    let album_json_str = serde_json::to_string_pretty(&media_album)?;

    file_out
        .write_all(album_json_str.as_bytes())
        .expect("Failed to write to album.json");

    return Ok(media_album);
}
