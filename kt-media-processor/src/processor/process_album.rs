use indexmap::IndexMap;

use std::cell::RefCell;

use std::rc::Rc;

use crate::model;
use crate::processor;

use std::error::Error;
use std::path::Path;

use std::io::{ErrorKind, Write};
use std::{fs, io};

// Recursive
pub fn process_album(
    media_albums: &mut IndexMap<String, Rc<RefCell<model::MediaAlbum>>>,
    media_files: &mut IndexMap<String, Rc<RefCell<model::MediaFile>>>,
    input_path: &Path,
    output_path: &Path,
    album_subpath_str: &str,
    media_album_meta: &Rc<RefCell<model::MediaAlbumSource>>,
) -> Result<Rc<RefCell<model::MediaAlbum>>, Box<dyn Error>> {
    let media_album_source = media_album_meta.borrow_mut();

    let media_album_rc: Rc<RefCell<model::MediaAlbum>>;

    // Read and start with existing album.json if it exists.
    let existing_album_json_path = output_path.join(album_subpath_str.trim_start_matches('/')).join("album.json");

    if existing_album_json_path.exists() {
        let existing_album_json_str = fs::read_to_string(&existing_album_json_path)?;
        media_album_rc = Rc::new(RefCell::new(
            serde_json::from_str(&existing_album_json_str).expect(
                format!(
                    "Failed to parse {} into MediaAlbum.",
                    existing_album_json_path.to_string_lossy()
                )
                    .as_str(),
            ),
        ));
        if !media_album_rc.borrow().path.eq(album_subpath_str) {
            return Err(Box::new(io::Error::new(
                ErrorKind::Other,
                format!(
                    "The path in {} does not match the directory structure.",
                    existing_album_json_path.to_string_lossy()
                ),
            )));
        }
    } else {
        media_album_rc = Rc::new(RefCell::new(model::MediaAlbum {
            path: album_subpath_str.to_string(),
            title: String::from("Unknown"),
            ordinal: -1,
            last_modified_dir: media_album_source.last_modified_dir,
            thumbnail: None,
            sub_albums: IndexMap::new(),
            media_files: IndexMap::new(),
        }));
    }


    let mut media_album = media_album_rc.borrow_mut();

    // Insert in to global media_albums map
    media_albums.insert(media_album.path.clone(), Rc::clone(&media_album_rc));

    // Update any fields from the album-source (which includes album-meta.json overrides) in case has changed
    media_album.title = media_album_source
        .title
        .clone()
        .unwrap_or_else(|| "Unknown".to_owned());
    media_album.ordinal = media_album_source.ordinal.unwrap_or(-1);
    media_album.thumbnail = media_album_source.thumbnail.clone();
    //media_album.last_modified_dir = media_album_source.last_modified_dir;

    // Vector of media files to be removed from album (due to errors) after iteration
    let mut media_files_to_remove = Vec::new();

    // Iterate all files of the album source
    for (media_file_meta_path, media_file_source) in &media_album_source.media_files {
        let media_file_source = media_file_source.borrow_mut();

        let media_file_rc = media_album
            .media_files
            .entry(media_file_meta_path.clone())
            .or_insert_with(|| {
                Rc::new(RefCell::new(model::MediaFile {
                    path: media_file_meta_path.clone(),
                    media_type: model::MediaFileType::UNKNOWN,
                    title: media_file_source.title.clone().unwrap_or(String::from("")),
                    ordinal: media_file_source.ordinal.unwrap_or(0),
                    last_modified: media_file_source.last_modified_file,
                    width: 0,
                    height: 0,
                    variants: IndexMap::new(),
                }))
            });

        let mut media_file = media_file_rc.borrow_mut();


        // Update any fields from the file-source (which includes any {file}-meta.json overrides) in case has changed
        media_file.title = media_file_source
            .title
            .clone()
            .unwrap_or_else(|| "Unknown".to_owned());
        media_file.ordinal = media_file_source.ordinal.unwrap_or(-1);

        // Build file source paths
        let file_src_path = &input_path.join(&media_file_meta_path.strip_prefix("/").unwrap());
        let src_filename_str = &file_src_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        // Create destination parent directory if needed
        let dst_subpath_noext_str = format!(
            "{}/{}",
            media_file_meta_path,
            src_filename_str.trim_end_matches(&format!(
                ".{}",
                file_src_path
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
            ))
        );

        let dst_subpath_noext_full_str = &format!("{}{}", output_path.to_string_lossy(), &dst_subpath_noext_str);
        let dst_subpath_noext = Path::new(&dst_subpath_noext_full_str);

        if let Some(dst_parent) = dst_subpath_noext.parent() {
            if !dst_parent.exists() {
                fs::create_dir_all(dst_parent)?;
            }
        }

        let _dst_parent = dst_subpath_noext
            .parent()
            .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Parent directory not found"))?;

        // Define a closure for processing files
        let process_file = |file_type: model::MediaFileType,
                            file_src_path: &Path,
                            output_path: &Path,
                            dst_subpath_noext_str: &str,
                            media_file_meta: &model::MediaFileSource,
                            media_file: &mut model::MediaFile| {
            media_file.media_type = file_type.clone();

            match file_type {
                model::MediaFileType::VIDEO => processor::process_file_video(
                    file_src_path,
                    output_path,
                    dst_subpath_noext_str,
                    media_file_meta,
                    media_file,
                ),
                model::MediaFileType::IMAGE => processor::process_file_image(
                    file_src_path,
                    output_path,
                    dst_subpath_noext_str,
                    media_file_meta,
                    media_file,
                ),
                _ => {
                    println!("Unhandled media type: {:?}", file_type);
                    Ok(())
                }
            }
        };

        // Determine file type and process the file
        let vid_exts = ["mp4", "m4v", "mkv", "mov", "h265", "hevc"];
        let file_type = if vid_exts
            .iter()
            .any(|&ext| media_file_meta_path.ends_with(ext))
        {
            model::MediaFileType::VIDEO
        } else {
            model::MediaFileType::IMAGE
        };

        // Process the file
        // Scope to borrow mutable
        {
            //let mut media_file = media_file_rc.borrow_mut();
            // Invoke the closure to process the file
            if let Err(e) = process_file(
                file_type,
                file_src_path,
                &output_path,
                &dst_subpath_noext_str,
                &media_file_source,
                &mut *media_file,
            ) {
                eprintln!(
                    "Error processing file {}. Msg: {}",
                    &file_src_path.to_string_lossy(),
                    e
                );
                // Remove this entry if errors occur while processing
                media_files_to_remove.push(media_file_meta_path.clone());
                continue;
            };

            // Clean up dst director and any left over variants from a previous run
            let dst_path_noext_str = &format!("{}{}", output_path.to_string_lossy(), &dst_subpath_noext_str);
            //let dst_subpath_parent_str = Path::new(&dst_subpath_noext_str).parent().unwrap().file_name().unwrap_or_default().to_string_lossy().to_string();
            let dst_path_noext = Path::new(dst_path_noext_str);
            let dst_parent_path = dst_path_noext.parent().unwrap();

            processor::clean_dst_dir(&output_path, &dst_parent_path, &mut *media_file)?;
        }
    }

    // Remove media files that do no longer exist in source
    media_album.media_files.iter().for_each(|(path, _)| {
        let src_path_str = format!("{}{}", output_path.to_string_lossy(), &path);
        let src_path = Path::new(&src_path_str);
        if !src_path.exists() {
            media_files_to_remove.push(path.clone());
        }
    });

    // Remove all media files with errors that occurred while processing
    media_files_to_remove
        .iter()
        .for_each(|media_file_meta_path| {
            media_album
                .media_files
                .shift_remove(&media_file_meta_path.to_string());
        });


    // Insert in to global media_files map
    for (path, media_file) in media_album.media_files.iter() {
        media_files.insert(path.clone(), Rc::clone(media_file));
    }


    // Recursively process source sub-albums
    for (sub_album_meta_path, sub_album_meta) in &media_album_source.sub_albums {
        let sub_media_album = process_album(
            media_albums,
            media_files,
            input_path,
            output_path,
            &sub_album_meta_path,
            sub_album_meta,
        );
        match sub_media_album {
            Ok(sub_media_album) => {
                media_album
                    .sub_albums
                    .insert(sub_album_meta_path.clone(), Rc::clone(&sub_media_album));
            }
            Err(e) => {
                eprintln!(
                    "Error processing media album {}. Skipping. Msg: {}",
                    &sub_album_meta_path, e
                );
                //return Err(e);
                continue;
            }
        }
    }

    // Remove any sub_albums that no longer exist
    let mut sub_albums_to_remove = Vec::new();
    media_album.sub_albums.iter().for_each(|(path, _)| {
        let src_path_str = format!("{}{}", output_path.to_string_lossy(), &path);
        let src_path = Path::new(&src_path_str);
        if !src_path.exists() {
            sub_albums_to_remove.push(path.clone());
        }
    });
    sub_albums_to_remove
        .iter()
        .for_each(|sub_album_meta_path| {
            media_album
                .sub_albums
                .shift_remove(&sub_album_meta_path.to_string());
        });

    media_album.sub_albums.sort_keys();
    media_album.media_files.sort_keys();

    // Write album.json
    let file_out_path = output_path
        .join(
            album_subpath_str
                .strip_prefix("/")
                .unwrap_or(album_subpath_str),
        )
        .join("album.json");

    // dbg!(&output_path);
    // dbg!(&file_out_path);

    if let Some(parent_dir) = file_out_path.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    }

    // Drop so that we can get the immutable ref back for serializing in the next stmt
    drop(media_album);

    let album_json_str =
        serde_json::to_string_pretty(&media_album_rc).expect("Failed to serialize MediaAlbum");

    // If album.json already exists and is the same, skip writing
    if file_out_path.exists() {
        let existing_album_json_str = fs::read_to_string(&file_out_path)?;
        if album_json_str == existing_album_json_str {
            return Ok(Rc::clone(&media_album_rc));
        }
    }

    let mut file_out = fs::File::create(&file_out_path).expect(&format!("Failed to create album.json file: {}", file_out_path.to_string_lossy()));
    file_out
        .write_all(album_json_str.as_bytes())
        .expect(&format!("Failed to write to album.json: {}", file_out_path.to_string_lossy()));

    return Ok(Rc::clone(&media_album_rc));
}
