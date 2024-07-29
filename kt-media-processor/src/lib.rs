use std::path::Path;

mod model;
pub mod processor;
pub mod scanner;
pub mod utils;

pub mod sync;

pub mod watcher;


pub fn scan_and_process_media(
    in_dir: &str,
    out_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let in_dir_path = Path::new(in_dir);

    // let timestamp = Utc::now().timestamp_millis().to_string();
    // let out_dir_path_tmp = format!("{}-{}", out_dir, timestamp);
    // let out_dir_path = Path::new(&out_dir_path_tmp);

    let out_dir_path = Path::new(out_dir);

    let scanned_result = scanner::scan_input_dir(in_dir_path)?;
    //let scanned_data = serde_json::to_string_pretty(&scanned_result.media_album_metas)?;
    //println!("Scanned: {}", scanned_data);

    let _processed_result =
        processor::process_input_meta(in_dir_path, out_dir_path, &scanned_result)?;
    //let processed_data = serde_json::to_string_pretty(&processed_result.media_albums)?;
    //println!("Processed: {}", processed_data);

    Ok(())
}


pub async fn do_s3_sync(
    local_dir: &str,
    aws_profile: Option<&str>,
    s3_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _local_dir_path = Path::new(local_dir);

    let result = sync::do_s3_sync(local_dir, aws_profile, s3_url);
    return result.await;
}