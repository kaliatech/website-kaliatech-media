use std::error::Error;
use std::path::Path;

use std::fs;
use std::fs::File;
use std::io::BufWriter;

use crate::model;

use image::codecs::avif::AvifEncoder;
use image::GenericImageView;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use super::ffmpeg;
use super::ffmpeg::ffprobe_meta::FfprobeMeta;

pub fn process_file_video(
    src_path: &Path,
    dst_root_path: &Path,
    dst_subpath_noext_str: &str,
    _media_file_meta: &model::MediaFileSource,
    media_file: &mut model::MediaFile,
) -> Result<(), Box<dyn Error>> {
    let thumbnail_subpath_str = format!("{}.tn.avif", &dst_subpath_noext_str);
    let thumbnail_path_str = &format!("{}{}", dst_root_path.to_string_lossy(), &thumbnail_subpath_str);
    let thumbnail_path = &Path::new(thumbnail_path_str);

    let tn_parent_path = thumbnail_path.parent().unwrap();
    if !tn_parent_path.exists() {
        fs::create_dir_all(tn_parent_path)?;
    }
    if !tn_parent_path.is_dir() {
        return Err("Destination parent path is a file instead of a directory".into());
    }

    let video_dst_path_str = format!(
        "{}/{}",
        tn_parent_path.to_str().unwrap(),
        src_path.file_name().unwrap().to_str().unwrap()
    );
    let video_dst_path = Path::new(&video_dst_path_str);

    // println!(
    //     "Considering: {}",
    //     thumbnail_subpath_str
    // );

    if thumbnail_path.exists() {
        let tn_last_modified = fs::metadata(&thumbnail_path).unwrap().modified()?;
        if tn_last_modified >= src_path.metadata()?.modified()?
            && tn_last_modified >= <DateTime<Utc> as Into<SystemTime>>::into(media_file.last_modified)
        {
            // Also check if dst_path exists in case thumbnail was created, but video was not output/copied due to errors
            if video_dst_path.exists() {
                return Ok(());
            }
        }
    }

    println!(
        "  Processing: {}",
        thumbnail_subpath_str
    );

    let tn_image = ffmpeg::extract_thumbnail(src_path)?;

    // Write thumbnail
    let file_out = &File::create(&thumbnail_path)?;
    let mut file_out_writer = BufWriter::new(file_out);
    let encoder = AvifEncoder::new_with_speed_quality(&mut file_out_writer, 4, 80);
    tn_image.write_with_encoder(encoder)?;

    let (tn_image_w, tn_image_h) = tn_image.dimensions();

    //TODO: generate 1280x720 thumbnail if default thumbnail is larger

    // Add thumbnails as variants to media_file
    media_file.variants.insert(
        thumbnail_subpath_str.to_string(),
        Rc::new(RefCell::new(model::MediaFileVariant {
            path: thumbnail_subpath_str.to_string(),
            mime_type: "image/avif".to_string(),
            width: tn_image_w,
            height: tn_image_h,
            duration: None,
            bytes: u32::try_from(file_out.metadata().unwrap().len()).unwrap_or(0),
            is_thumbnail: true,
        })),
    );

    // Copy source file (no transcoding, for now)
    // ...eventually `ffmpeg -i input.flv -vcodec libx264 -acodec aac output.mp4` ...or similar
    fs::copy(src_path, video_dst_path)?;

    let ffprobe_meta = ffmpeg::extract_meta(src_path)?;

    // Add source file as a variant
    let source_subpath_str = dst_subpath_noext_str
        .rsplit_once('/')
        .map_or("", |(left, _)| left);
    let source_subpath_str = format!("{}/{}", source_subpath_str, video_dst_path.file_name().unwrap().to_str().unwrap());

    media_file.variants.insert(
        source_subpath_str.clone(),
        Rc::new(RefCell::new(model::MediaFileVariant {
            path: media_file.path.clone(),
            mime_type: get_mime_type(&ffprobe_meta),
            width: ffprobe_meta.streams[0].width,
            height: ffprobe_meta.streams[0].height,
            duration: ffprobe_meta.streams[0].get_duration().ok(),
            bytes: u32::try_from(src_path.metadata().unwrap().len()).unwrap_or(0),
            is_thumbnail: false,
        })),
    );

    return Ok(());
}

fn get_mime_type(ffprobe_meta: &FfprobeMeta) -> String {
    if ffprobe_meta.streams[0].codec_name == "h264" {
        return "video/mp4".to_string();
    } else {
        return "video".to_string();
    }
}
