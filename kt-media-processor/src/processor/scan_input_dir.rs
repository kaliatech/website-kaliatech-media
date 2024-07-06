use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;

use std::boxed::Box;
use std::rc::Rc;

use chrono::{DateTime, Utc};

use crate::model;
use crate::utils;
use crate::utils::read_object_from_json_file;
use std::error::Error;

pub struct ScanResult {
    pub media_album_metas: HashMap<String, Rc<RefCell<model::MediaAlbumMeta>>>,
    pub media_file_metas: HashMap<String, Rc<RefCell<model::MediaFileMeta>>>,
}

pub fn scan_input_dir(input_path: &Path) -> Result<Box<ScanResult>, Box<dyn Error>> {
    let files = utils::glob_files(input_path.as_ref());

    let mut media_album_metas: HashMap<String, Rc<RefCell<model::MediaAlbumMeta>>> = HashMap::new();
    let mut media_file_metas: HashMap<String, Rc<RefCell<model::MediaFileMeta>>> = HashMap::new();

    media_album_metas.insert(
        String::from("/"),
        Rc::new(RefCell::new(model::MediaAlbumMeta {
            title: None,
            ordinal: None,
            last_modified: Some(Utc::now()),
            sub_albums: HashMap::new(),
            media_files: HashMap::new(),
        })),
    );

    // Reverse the order to be sure directories come before files within the directory
    for file in files.iter().rev() {
        let sub_path = file
            .strip_prefix(input_path)
            .expect("Original file path does not start with the original parent path");

        //dbg!(sub_path);
        let parent_path = format!("/{}", sub_path.parent().unwrap().to_string_lossy());
        let media_album_meta = media_album_metas.entry(parent_path.to_string());

        if file.is_dir() {
            //println!("Directory: {:?}", file);
            //media_root.albums.push(

            //if sub_path == Path::new("") {}
            let new_media_album = Rc::new(RefCell::new(model::MediaAlbumMeta {
                title: sub_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string()),
                ordinal: None,
                last_modified: Some(DateTime::<Utc>::from(file.metadata()?.modified()?)),
                sub_albums: HashMap::new(),
                media_files: HashMap::new(),
            }));

            media_album_metas.insert(
                //format!("/{}", sub_path.to_string_lossy()),
                format!("/{}", sub_path.to_string_lossy()),
                Rc::clone(&new_media_album),
            );

            //Insert into parent
            let parent_media_album_meta = media_album_metas.get_mut(&parent_path);
            dbg!(&parent_path);

            if let Some(parent_media_album_meta2) = parent_media_album_meta {
                let mut album_meta = parent_media_album_meta2.borrow_mut();
                album_meta.sub_albums.insert(
                    format!("/{}/", sub_path.to_string_lossy()),
                    Rc::clone(&new_media_album),
                );
            }

            // if let Some(parent_meta_mut) = parent_media_album_meta.borrow_mut() {
            //     parent_meta_mut.sub_albums.borrow_mut().insert(
            //         format!("/{}", sub_path.to_string_lossy()),
            //         Rc::clone(&new_media_album),
            //     );
            // } else {
            //     // Handle the case where parent_media_album_meta cannot be borrowed mutably, if necessary
            //     dbg!("Handle the case where parent_media_album_meta cannot be borrowed mutably, if necessary");
            // }

            // if let Some(parent_media_album_meta) = parent_media_album_meta {
            //     Rc::get_mut(parent_media_album_meta)
            //         .unwrap()
            //         .sub_albums
            //         .borrow_mut()
            //         .insert(
            //             format!("/{}", sub_path.to_string_lossy()),
            //             Rc::clone(&new_media_album),
            //         );
            // }

            // media_album_meta.and_modify(|album_meta| {
            //     Rc::get_mut(album_meta)
            //         .unwrap()
            //         .media_files
            //         .insert(file_path.clone(), Rc::clone(&media_file_meta));
            // });
            // parent_media_album_meta.sub_albums.insert(
            //     format!("/{}/", sub_path.to_string_lossy()),
            //     Rc::clone(&new_media_album),
            // );

            //let sub_albums = Rc::get_mut(&mut parent_media_album_meta.sub_albums).expect("WTF");

            // .expect("Failed to borrow Rc<MediaAlbumMeta> as mutable")
            // .insert(
            //     format!("/{}/", sub_path.to_string_lossy()),
            //     Rc::clone(&new_media_album),
            // );

            // .entry(format!("/{}/", sub_path.to_string_lossy()))
            // .or_insert(Rc::clone(&new_media_album));

            // parent_media_album_meta.sub_albums.insert(
            //     format!("/{}/", sub_path.to_string_lossy()),
            //     Rc::clone(&new_media_album),
            // );
            // .sub_albums
            // .entry(format!("/{}/", sub_path.to_string_lossy()))
            // .or_insert(Rc::clone(&new_media_album));
        } else {
            let file_path = format!("/{}", sub_path.to_string_lossy());

            //dbg!(file_path.clone());

            if file_path.ends_with("/album-meta.json") {
                // dbg!(input_path);
                // dbg!(&file_path);
                // dbg!(input_path.join(&file_path));
                let album_meta_overrides: model::MediaAlbumMeta =
                    read_object_from_json_file(Path::new(&format!(
                        "{}{}",
                        input_path.to_str().unwrap_or_default(),
                        file_path.as_str()
                    )))?;
                if album_meta_overrides.title.is_some() {
                    media_album_metas
                        .entry(parent_path)
                        .and_modify(|album_meta| {
                            album_meta.borrow_mut().title = album_meta_overrides.title.clone();
                        });
                }
            } else {
                let media_file_meta = Rc::new(RefCell::new(model::MediaFileMeta {
                    title: sub_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string(),
                    ordinal: 0,
                    last_modified: Utc::now(),
                }));

                media_file_metas.insert(file_path.clone(), Rc::clone(&media_file_meta));

                // if media_album_meta.is_none() {
                //     eprintln!("Parent album not found: {}", parent_path);
                // } else {

                //TODO: The get_mut here is not safe
                media_album_meta.and_modify(|media_album_meta| {
                    if let Some(album_meta) = Rc::get_mut(media_album_meta) {
                        (*album_meta)
                            .borrow_mut()
                            .media_files
                            .insert(file_path.clone(), Rc::clone(&media_file_meta));
                    }
                    // Rc::get_mut(media_album_meta)
                    //     .unwrap()
                    //     .media_files
                    //     .insert(file_path.clone(), Rc::clone(&media_file_meta));
                });
                // .and_modify(|album_meta| {
                //     album_meta
                //         .media_files
                //         .entry(file_path.clone())
                //         .or_insert(Rc::clone(&media_file_meta))
                // Rc::get_mut(album_meta)
                //     .unwrap()
                //     .media_files
                //     .insert(file_path.clone(), Rc::clone(&media_file_meta));
            }

            // media_album_metas
            //     .entry(parent_path.clone())
            //     .and_modify(|album_meta| {
            //         album_meta
            //             .media_files
            //             .insert(file_path.clone(), media_file_meta.clone());
            //     });
            // media_album_meta
            //     .unwrap()
            //     .media_files
            //     .insert(file_path.clone(), media_file_meta.clone());

            // if let Some(album_meta) = media_album_meta.as_mut() {
            //     album_meta
            //         .media_files
            //         .insert(file_path.clone(), media_file_meta.clone());
            // }
            // media_album_meta
            //     .unwrap()
            //     .media_files
            //     .insert(file_path.clone(), media_file_meta.clone());
            // dbg!(media_album_meta
            //     .as_ref()
            //     .unwrap()
            //     .title
            //     .as_ref()
            //     .unwrap_or(&String::from("Empty")));
            //}
        }
    }

    dbg!(media_album_metas.iter().count());

    let scan_result = ScanResult {
        // root_album_meta: model::MediaAlbumMeta {
        //     title: None,
        //     ordinal: None,
        //     last_modified: Utc::now(),
        //     sub_albums: HashMap::new(),
        //     media_files: HashMap::new(),
        // },
        media_album_metas: media_album_metas,
        media_file_metas: media_file_metas,
    };

    return Ok(Box::new(scan_result));
}
