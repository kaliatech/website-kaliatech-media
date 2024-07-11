use std::error::Error;
use std::path::Path;

use std::fs;

use crate::model;
use crate::utils::glob_files;

pub fn clean_dst_dir(dst_root_dir: &Path, media_file_dst_dir: &Path, media_file: &mut model::MediaFile) -> Result<(), Box<dyn Error>> {
    let files = glob_files(media_file_dst_dir);
    for file in files {
        if file.is_dir() {
            continue;
        }
        let file_name = file.file_name();
        let file_name_str = file_name.unwrap().to_string_lossy();

        let media_parent_path = media_file_dst_dir.strip_prefix(dst_root_dir)
            .expect("Failed to strip dst_root_dir prefix from media_file_dst_dir")
            .to_path_buf();
        let media_parent_path_str = &media_parent_path.to_string_lossy();


        // Delete any files in dst directory that are not in the variants list
        let media_file_variant_path_str = format!("/{}/{}", &media_parent_path_str, &file_name_str);

        if media_file.variants.contains_key(&media_file_variant_path_str) {
            continue;
        }

        println!("Remove file: {}", &file.to_string_lossy());
        fs::remove_file(file)?;
    }

    return Ok(());
}
