use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;

use chrono::Utc;

use serde_json::Result as SerdeResult;

pub mod model;
pub mod utils;

//fn read_media_root_from_file(path: &Path) -> SerdeResult<model::MediaRoot> {
fn read_media_root_from_file<P: AsRef<Path>>(path: P) -> SerdeResult<model::MediaAlbum> {
    // // Open the file in read-only mode.
    if !path.as_ref().exists() {
        File::create(path.as_ref()).expect("Failed to create root.json file");

        let data = r#"{
    "path": "/",
    "ordinal": 0,
    "sub_albums": {}
    "media_files": {}
}"#;

        let mut file_out = File::create(path.as_ref()).expect("Failed to create root.json file");
        file_out
            .write_all(data.as_bytes())
            .expect("Failed to write to root.json");
    }
    // let mut file = File::open(path.as_ref()).expect("Failed to open file");

    // Read the contents of the file into a string.
    let mut file_in = File::open(path.as_ref()).expect("Failed to open file");
    let mut contents = String::new();
    file_in
        .read_to_string(&mut contents)
        .expect("Failed to read file");

    // Deserialize the JSON string into MediaRoot struct.
    serde_json::from_str(&contents)
}

pub fn scan_and_process_media(
    in_dir: &str,
    out_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = Utc::now().timestamp_millis().to_string();

    let in_dir_path = Path::new(in_dir);
    let formatted_path = format!("{}-{}", out_dir, timestamp);
    let out_dir_path = Path::new(&formatted_path);

    if !out_dir_path.exists() {
        println!("Creating output directory: {:?}", out_dir_path);
        std::fs::create_dir_all(out_dir_path)?;
    }

    let root_media_json_path = out_dir_path.join("root.json");
    println!("Root media JSON path: {:?}", root_media_json_path);

    let mut media_root = match read_media_root_from_file(&root_media_json_path) {
        Ok(media_root) => {
            println!("Successfully read media root.");
            media_root
        }
        Err(e) => {
            println!("Error reading media root: {}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Unable to read root.json file.",
            )));
        }
    };

    let files = utils::collect_files(in_dir_path);

    // let root_media_json_path = in_dir_path.join("root.json");
    // if !root_media_json_path.exists() {
    //     if File::create(&root_media_json_path).is_err() {
    //         eprintln!("Failed to create root.json file in input directory.");
    //         return Err(Box::new(std::io::Error::new(
    //             std::io::ErrorKind::NotFound,
    //             "root.json file not found in input directory.",
    //         )));
    //     }
    // }

    // let root_media_json_file = File::open(&root_media_json_path);
    // if let Err(e) = root_media_json_file {
    //     eprintln!("Failed to open file: {:?}", e);
    //     // Perform additional error handling or return an error from the function
    //     return Err(Box::new(std::io::Error::new(
    //         std::io::ErrorKind::Other,
    //         "Unable to open root.json file",
    //     )));
    // }

    // let mut root_media_file = File::open(in_dir_path.join("root.json"));
    // let mut data = String::new();
    // root_media_file.read_to_string(&mut data).unwrap();

    // let mut data = String::new();
    // root_media_json_file
    //     .unwrap()
    //     .read_to_string(&mut data)
    //     .unwrap();

    for file in files {
        if file.is_dir() {
            println!("Directory: {:?}", file);
            //media_root.albums.push(
            let sub_path = file
                .strip_prefix(in_dir_path)
                .expect("Original file path does not start with the original parent path");

            let album_title = sub_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            let album_path_str = format!("/{}", album_title);

            let media_album_new = model::MediaAlbum {
                title: album_title,
                path: album_path_str.clone(),
                ordinal: 0,
                last_modified: Utc::now(),
                sub_albums: HashMap::new(),
                media_files: HashMap::new(),
            };
            let new_dir_path = out_dir_path.join(sub_path);
            if let Err(err) = std::fs::create_dir_all(new_dir_path) {
                eprintln!("Failed to create directory: {:?}", err);
                return Err(Box::new(err));
            }

            media_root
                .sub_albums
                .insert(album_path_str, media_album_new);
            continue;
        }
        //println!("{:?}", file);
        //println!("{:?}", file.extension().unwrap());
    }

    let data = serde_json::to_string_pretty(&media_root)?;

    println!("Data: {}", data);

    let mut file_out =
        File::create(&root_media_json_path).expect("Failed to create root.json file");
    file_out
        .write_all(data.as_bytes())
        .expect("Failed to write to root.json");

    println!("...kt-media-processor finished.");
    Ok(())
}
