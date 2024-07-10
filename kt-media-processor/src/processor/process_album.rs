use indexmap::IndexMap;

use std::cell::RefCell;

use std::rc::Rc;

use crate::model;
use crate::processor;

use std::error::Error;
use std::path::Path;

use std::{fs, io};
use std::io::{ErrorKind, Write};


// Recursive
pub fn process_album(
    input_path: &Path,
    output_path: &Path,
    album_subpath_str: &str,
    media_album_meta: &Rc<RefCell<model::MediaAlbumMeta>>,
) -> Result<Rc<RefCell<model::MediaAlbum>>, Box<dyn Error>> {
    let media_album_meta = media_album_meta.borrow_mut();

    let media_album_rc: Rc<RefCell<model::MediaAlbum>>;

    // Read and start with existing album.json if it exists.
    let existing_album_json_path = output_path.join(album_subpath_str).join("album.json");
    if existing_album_json_path.exists() {
        let existing_album_json_str = fs::read_to_string(&existing_album_json_path)?;
        media_album_rc = Rc::new(RefCell::new(
            serde_json::from_str(&existing_album_json_str)
                .expect("Failed to parse JSON into MediaAlbum"),
        ));
    } else {
        media_album_rc = Rc::new(RefCell::new(model::MediaAlbum {
            path: album_subpath_str.to_string(),
            title: String::from("Unknown"),
            ordinal: -1,
            last_modified_dir: media_album_meta.last_modified_dir,
            sub_albums: IndexMap::new(),
            media_files: IndexMap::new(),
        }));
    }

    let mut media_album = media_album_rc.borrow_mut();

    // Update any fields from the album meta.json in case has changed
    media_album.title = media_album_meta
        .title
        .clone()
        .unwrap_or_else(|| "Unknown".to_owned());
    media_album.ordinal = media_album_meta.ordinal.unwrap_or(-1);
    media_album.last_modified_dir = media_album_meta.last_modified_dir;

    // Iterate all files of the album
    let mut media_files_to_remove = Vec::new();
    for (media_file_meta_path, media_file_meta) in &media_album_meta.media_files {
        let media_file_meta = media_file_meta.borrow_mut();

        let media_file_rc = media_album.media_files.entry(media_file_meta_path.clone()).or_insert_with(|| {
            Rc::new(RefCell::new(model::MediaFile {
                path: media_file_meta_path.clone(),
                media_type: model::MediaFileType::UNKNOWN,
                title: media_file_meta.title.clone().unwrap_or(String::from("")),
                ordinal: media_file_meta.ordinal.unwrap_or(0),
                last_modified: media_file_meta.last_modified_file,
                width: 0,
                height: 0,
                variants: IndexMap::new(),
            }))
        });

        // resize, generate thumbnails, etc
        let file_src_path = &input_path.join(
            &media_file_meta_path
                .strip_prefix("/")
                .unwrap_or(album_subpath_str),
        );
        {
            let mut media_file = media_file_rc.borrow_mut();

            let src_filename_str = &file_src_path.file_name().unwrap_or_default().to_string_lossy();

            let dst_subpath_noext_str = format!(
                "{}/{}",
                src_filename_str,
                media_file
                    .path
                    .strip_prefix("/")
                    .unwrap_or(&media_file.path)
                    .trim_end_matches(&format!(
                        ".{}",
                        file_src_path.extension().unwrap_or_default().to_string_lossy()
                    ))
            );
            let dst_subpath_noext = output_path.join(&dst_subpath_noext_str);
            // let dst_subpath_noext_str = dst_subpath_noext.to_string_lossy();

            if let Some(dst_parent) = dst_subpath_noext.parent() {
                if !dst_parent.exists() {
                    fs::create_dir_all(dst_parent)?;
                }
            }
            let dst_parent = dst_subpath_noext.parent().ok_or_else(|| {
                io::Error::new(ErrorKind::NotFound, "Parent directory not found")
            })?;

            let media_file_path_str = media_file.path.to_string();


            // Define a closure for processing files
            let process_file = |file_type: model::MediaFileType, file_src_path: &Path, output_path: &Path, dst_subpath_noext_str: &str, media_file_meta: &model::MediaFileMeta, media_file: &mut model::MediaFile| {
                match file_type {
                    model::MediaFileType::VIDEO => {
                        processor::process_file_video(
                            file_src_path,
                            output_path,
                            dst_subpath_noext_str,
                            media_file_meta,
                            media_file,
                        )
                    }
                    model::MediaFileType::IMAGE => {
                        processor::process_file_image(
                            file_src_path,
                            output_path,
                            dst_subpath_noext_str,
                            media_file_meta,
                            media_file,
                        )
                    }
                    _ => {
                        Ok(())
                    }
                }
            };

            // Determine file type and process the file
            let vid_exts = ["mp4", "m4v", "mkv", "mov", "h265", "hevc"];
            let file_type = if vid_exts.iter().any(|&ext| media_file_path_str.ends_with(ext)) {
                model::MediaFileType::VIDEO
            } else {
                model::MediaFileType::IMAGE
            };

            // Invoke the closure to process the file
            if let Err(e) = process_file(file_type, file_src_path, &output_path, &dst_subpath_noext_str, &media_file_meta, &mut *media_file) {
                eprintln!("Error processing file {}. Msg: {}", &file_src_path.to_string_lossy(), e);
                // Remove this entry if errors occur while processing
                media_files_to_remove.push(media_file_meta_path);
                continue;
            };
        }

        // insert to the album
        // we previously did this before switching the entry api to capture existing variants
        // TODO: Will need to manually remove this entry if errors occur while processing
        // media_album
        //     .media_files
        //     .insert(media_file_meta_path.clone(), Rc::clone(&media_file_rc));
    }

    // Remove all media files with errors that occurred while processing
    media_files_to_remove.iter().for_each(|media_file_meta_path| {
        media_album.media_files.shift_remove(&media_file_meta_path.to_string());
    });

    for (sub_album_meta_path, sub_album_meta) in &media_album_meta.sub_albums {
        // Recursively process sub-albums
        let sub_media_album = process_album(
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
            },
            Err(e) => {
                eprintln!("Error processing media album {}. Skipping. Msg: {}", &sub_album_meta_path, e);
                //return Err(e);
                continue;
            }
        }
    }

    //TODO: Only write a new album.json if the last_modified_dir is newer than existing album.json (if any) last modified

    media_album.sub_albums.sort_keys();
    media_album.media_files.sort_keys();

    // Drop so that we can get the immutable ref back for serializing
    drop(media_album);

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

    let album_json_str =
        serde_json::to_string_pretty(&media_album_rc).expect("Failed to serialize MediaAlbum");

    // If album.json already exists and is the same, skip writing
    if file_out_path.exists() {
        let existing_album_json_str = fs::read_to_string(&file_out_path)?;
        if album_json_str == existing_album_json_str {
            return Ok(media_album_rc);
        }
    }

    let mut file_out = fs::File::create(&file_out_path).expect("Failed to create album.json file");
    file_out
        .write_all(album_json_str.as_bytes())
        .expect("Failed to write to album.json");

    return Ok(media_album_rc);
}
