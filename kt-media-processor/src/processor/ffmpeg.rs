use std::process::{Command, Stdio};

use ffprobe_meta::FfprobeMeta;
use image::DynamicImage;

use std::path::Path;

use tempdir::TempDir;

use image::io::Reader as ImageReader;

pub mod ffprobe_meta;

#[cfg(test)]
mod tests;

pub fn extract_thumbnail(video_path: &Path) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let tmp_dir = TempDir::new("kt-media-processor")?;
    let file_stem = video_path.file_stem().expect("Video path has no file stem");
    let file_path = tmp_dir.path().join(file_stem).with_extension(".jpg");
    // let mut tmp_file = File::create(file_path)?;

    let duration = extract_duration(video_path)?;

    // Extract thumbnail with ffmpeg
    let _cmd = Command::new("ffmpeg")
        .args([
            "-i",
            video_path.to_str().unwrap(),
            "-vf",
            "thumbnail",
            "-ss",
            (duration / 2).to_string().as_str(),
            "-frames:v",
            "1",
            file_path.to_str().unwrap(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute ffmpeg");

    let img = ImageReader::open(file_path)?.decode()?;

    return Ok(img);
}

fn extract_duration(video_path: &Path) -> Result<u32, Box<dyn std::error::Error>> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            video_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute ffprobe");

    let duration_str = String::from_utf8(output.stdout).unwrap();
    let duration = duration_str.trim().parse::<f32>().unwrap() as u32;

    return Ok(duration);
}

pub fn extract_meta(video_path: &Path) -> Result<FfprobeMeta, Box<dyn std::error::Error>> {
    //ffprobe -i kt-media-root/test-vid-17s.mp4 -v error -print_format json -show_streams -select_streams v:0
    let output = Command::new("ffprobe")
        .args([
            "-i",
            video_path.to_str().unwrap(),
            "-v",
            "error",
            "-print_format",
            "json",
            "-show_streams",
            "-select_streams",
            "v:0",
        ])
        .output()
        .expect("Failed to execute ffprobe");

    let output_str = String::from_utf8(output.stdout).unwrap();
    //dbg!(&output_str);
    let ffprobe_meta = serde_json::from_str(&output_str).map_err(Into::into);
    return ffprobe_meta;
}
