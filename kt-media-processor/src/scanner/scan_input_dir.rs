use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use std::path::Path;

use std::boxed::Box;
use std::rc::Rc;

use chrono::{DateTime, Utc};

use crate::model;
use crate::scanner::apply_album_meta_jsons::apply_album_meta_jsons;
use crate::utils;
use crate::utils::read_object_from_json_file;
use std::error::Error;

pub struct ScanResult {
    pub media_album_metas: HashMap<String, Rc<RefCell<model::MediaAlbumMeta>>>,
    pub media_file_metas: HashMap<String, Rc<RefCell<model::MediaFileMeta>>>,
}

pub fn scan_input_dir(input_path: &Path) -> Result<Box<ScanResult>, Box<dyn Error>> {
    // Glob all files and directories
    let files = utils::glob_files(input_path.as_ref());

    // Hashmaps to store all meta objects for albums and files
    let mut media_album_metas: HashMap<String, Rc<RefCell<model::MediaAlbumMeta>>> = HashMap::new();
    let mut media_album_jsons: HashMap<String, Rc<RefCell<model::MediaAlbumMeta>>> = HashMap::new();
    let mut media_file_metas: HashMap<String, Rc<RefCell<model::MediaFileMeta>>> = HashMap::new();
    let mut media_file_jsons: HashMap<String, Rc<RefCell<model::MediaFileMeta>>> = HashMap::new();

    // Manually insert the root album
    media_album_metas.insert(
        String::from(""),
        Rc::new(RefCell::new(model::MediaAlbumMeta {
            title: None,
            ordinal: None,
            last_modified_dir: Utc::now(), //TODO: get modified time of the root?
            last_modified_override: Some(Utc::now()),
            sub_albums: HashMap::new(),
            media_files: HashMap::new(),
        })),
    );

    // Iterate through all directories and files
    // Reverse the order to be sure directories come before files within the directory
    for file_path in files.iter().rev() {
        let sub_path = file_path
            .strip_prefix(input_path)
            .expect("Original file path does not start with the original parent path");

        let parent_path = file_path.parent().expect("Unable to get parent path");
        //let parent_path = format!("/{}", sub_path.parent().unwrap().to_string_lossy());
        //        let media_album_meta = media_album_metas.entry(parent_path.to_string());

        if file_path.is_dir() {
            scan_dir(
                input_path,
                file_path,
                parent_path,
                sub_path,
                &mut media_album_metas,
            );
        } else {
            scan_file(
                input_path,
                file_path,
                parent_path,
                sub_path,
                &mut media_album_metas,
                &mut media_album_jsons,
                &mut media_file_metas,
                &mut media_file_jsons,
            )
            .expect("Unable to scan file");
        }
    }

    // Iterate through all media_album_jsons, processing the 'album-meta.json' files
    apply_album_meta_jsons(&mut media_album_metas, &mut media_album_jsons)?;
    //process_file_meta_jsons

    let scan_result = ScanResult {
        media_album_metas: media_album_metas,
        media_file_metas: media_file_metas,
    };

    return Ok(Box::new(scan_result));
}

fn scan_dir(
    input_path: &Path,
    dir: &Path,
    parent_path: &Path,
    sub_path: &Path,
    //    _media_album_meta: &Entry<String, Rc<RefCell<model::MediaAlbumMeta>>>,
    media_album_metas: &mut HashMap<String, Rc<RefCell<model::MediaAlbumMeta>>>,
) {
    let last_modified_dir = match dir.metadata() {
        Ok(metadata) => match metadata.modified() {
            Ok(modified) => Some(DateTime::<Utc>::from(modified)),
            Err(_) => None, // Handle the error or log it as needed
        },
        Err(_) => None, // Handle the error or log it as needed
    };

    let new_media_album = Rc::new(RefCell::new(model::MediaAlbumMeta {
        title: sub_path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string()),
        ordinal: None,
        last_modified_override: None,
        last_modified_dir: last_modified_dir.unwrap_or_default(),
        sub_albums: HashMap::new(),
        media_files: HashMap::new(),
    }));

    media_album_metas.insert(
        format!("/{}", sub_path.to_string_lossy()),
        Rc::clone(&new_media_album),
    );

    //Insert into parent
    let parent_sub_path_str = parent_path
        .to_str()
        .unwrap_or_default()
        .trim_start_matches(input_path.to_str().unwrap_or_default());

    dbg!(&parent_sub_path_str);

    if let Entry::Occupied(mut o) = media_album_metas.entry(parent_sub_path_str.to_string()) {
        o.get_mut().borrow_mut().sub_albums.insert(
            format!("/{}/", sub_path.to_string_lossy()),
            Rc::clone(&new_media_album),
        );
    }

    // let parent_media_album_meta = media_album_metas.entry(parent_sub_path_str.to_string());
    // if let Some(parent_media_album_meta2) = parent_media_album_meta {
    //     let mut album_meta = parent_media_album_meta2.borrow_mut();
    //     album_meta.sub_albums.insert(
    //         format!("/{}/", sub_path.to_string_lossy()),
    //         Rc::clone(&new_media_album),
    //     );
    // }
    // match media_album_metas.entry(format!("/{}/", sub_path.to_string_lossy())) {
    //     Entry::Vacant(e) => {
    //         eprint!("Missing parent")
    //     }
    //     Entry::Occupied(mut e) => {
    //         e.get_mut()
    //             .borrow_mut()
    //             .sub_albums
    //             .insert(Rc::clone(&new_media_album));
    //     }
    // }
}

fn scan_file(
    input_path: &Path,
    file_path: &Path,
    parent_path: &Path,
    sub_path: &Path,
    media_album_metas: &mut HashMap<String, Rc<RefCell<model::MediaAlbumMeta>>>,
    media_album_jsons: &mut HashMap<String, Rc<RefCell<model::MediaAlbumMeta>>>,
    media_file_metas: &mut HashMap<String, Rc<RefCell<model::MediaFileMeta>>>,
    media_file_jsons: &mut HashMap<String, Rc<RefCell<model::MediaFileMeta>>>,
) -> Result<(), Box<dyn Error>> {
    let sub_path_str = format!("/{}", sub_path.to_string_lossy());
    // let file_path_str = format!(
    //     "{}{}",
    //     input_path.to_str().unwrap_or_default(),
    //     path.as_str()
    // );
    // let file_path = Path::new(&formatted_path);

    if file_path.ends_with("album-meta.json") {
        let album_meta_overrides: model::MediaAlbumMeta = read_object_from_json_file(file_path)?;
        let album_meta = Rc::new(RefCell::new(model::MediaAlbumMeta {
            title: album_meta_overrides.title,
            ordinal: album_meta_overrides.ordinal,
            last_modified_dir: Utc::now(), // TODO: not used, could maybe set to optional in the model and then use the json object directly
            last_modified_override: album_meta_overrides.last_modified_override,
            sub_albums: HashMap::new(),
            media_files: HashMap::new(),
        }));
        media_album_jsons.insert(
            format!("/{}", sub_path.to_string_lossy()),
            Rc::clone(&album_meta),
        );
        return Ok(());
    }

    if file_path.ends_with("-meta.json") {
        let file_meta_overrides: model::MediaFileMeta = read_object_from_json_file(file_path)?;
        let file_meta = Rc::new(RefCell::new(model::MediaFileMeta {
            title: file_meta_overrides.title,
            ordinal: file_meta_overrides.ordinal,
            last_modified_file: Utc::now(), // TODO: not used, could maybe set to optional in the model and then use the json object directly
            last_modified_override: file_meta_overrides.last_modified_override,
        }));
        media_file_jsons.insert(
            format!("/{}", sub_path.to_string_lossy()),
            Rc::clone(&file_meta),
        );
        return Ok(());
    }

    //TODO: Filter for only file extensions we can support
    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_lowercase();
    if !["jpg", "jpeg", "png", "gif", "mp4", "avif", "webp"].contains(&ext.as_str()) {
        return Ok(());
    }

    // If here, file is a media file we need to check and process
    let last_modified_file = match file_path.metadata() {
        Ok(metadata) => match metadata.modified() {
            Ok(modified) => Some(DateTime::<Utc>::from(modified)),
            Err(_) => None, // Handle the error or log it as needed
        },
        Err(_) => None, // Handle the error or log it as needed
    };

    let media_file_meta = Rc::new(RefCell::new(model::MediaFileMeta {
        title: file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string(),
        ordinal: 0,
        last_modified_file: last_modified_file.unwrap_or_default(),
        last_modified_override: None,
    }));

    // Insert in to media_file_metas list
    media_file_metas.insert(sub_path_str.clone(), Rc::clone(&media_file_meta));

    // Insert in to the media_album_meta map
    let parent_sub_path_str = parent_path
        .to_str()
        .unwrap_or_default()
        .trim_start_matches(input_path.to_str().unwrap_or_default());
    let media_album_meta = media_album_metas.entry(parent_sub_path_str.to_string());
    media_album_meta.and_modify(|album_meta| {
        album_meta
            .borrow_mut()
            .media_files
            .insert(sub_path_str.clone(), Rc::clone(&media_file_meta));
    });

    return Ok(());
}
