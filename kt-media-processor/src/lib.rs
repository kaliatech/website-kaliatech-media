use std::path::Path;

use chrono::Utc;

pub mod model;
pub mod processor;
pub mod utils;

pub fn scan_and_process_media(
    in_dir: &str,
    out_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = Utc::now().timestamp_millis().to_string();

    let in_dir_path = Path::new(in_dir);

    let out_dir_path_tmp = format!("{}-{}", out_dir, timestamp);
    let out_dir_path = Path::new(&out_dir_path_tmp);

    let scan_result = processor::scan_input_dir(in_dir_path)?;
    let scan_data = serde_json::to_string_pretty(&scan_result.media_album_metas)?;
    println!("Scan: {}", scan_data);

    let output_result = processor::process_input_meta(in_dir_path, out_dir_path, &scan_result)?;
    let output_data = serde_json::to_string_pretty(&output_result.media_albums)?;
    println!("Output: {}", output_data);

    Ok(())
}
