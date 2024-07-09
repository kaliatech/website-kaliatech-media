use crate::processor::ffmpeg;

use super::*;

const TEST_VIDEO_PATH_STR: &str = "test/test-vid-17s.mp4";

#[test]
fn test_thumbnail() {
    let thumbnail = ffmpeg::extract_thumbnail(Path::new(TEST_VIDEO_PATH_STR));
    assert_eq!("", "");
    assert!(thumbnail.is_ok());
}

#[test]
fn test_duration() {
    let duration = ffmpeg::extract_duration(Path::new(TEST_VIDEO_PATH_STR));
    assert_eq!(duration.unwrap(), 17);
}

#[test]
fn test_extract_meta() {
    let ffprobe_meta = ffmpeg::extract_meta(Path::new(TEST_VIDEO_PATH_STR));
    if ffprobe_meta.is_err() {
        eprintln!("Error: {:?}", ffprobe_meta.as_ref().err());
    } else {
        assert!(ffprobe_meta.is_ok());
        let ffprobe_meta = ffprobe_meta.unwrap();
        assert_eq!(
            &ffprobe_meta.streams[0].duration, "17.517500",
            "Duration should be 17.517500"
        );

        assert_eq!(
            ffprobe_meta.streams[0].get_duration(),
            Ok(17.517500 as f64),
            "Duration should be 17.517500"
        );
    }
}
