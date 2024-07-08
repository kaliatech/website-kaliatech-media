use std::borrow::BorrowMut;
use std::cell::RefMut;
use std::error::Error;
use std::path::Path;

use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::model;
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

use image::io::Reader as ImageReader;
use image::GenericImageView;

use image::codecs::avif::AvifEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::webp::WebPEncoder;
use image::ImageEncoder;

use std::time::Instant;

pub fn process_file(
    file_src_path: &Path,
    output_path: &Path,
    media_file_meta: Ref<model::MediaFileMeta>,
    mut media_file: RefMut<model::MediaFile>,
) -> Result<(), Box<dyn Error>> {
    //todo!();

    let file_img_subpath_str = format!(
        "{}",
        media_file
            .path
            .strip_prefix("/")
            .unwrap_or(&media_file.path)
            .trim_end_matches(&format!(
                ".{}",
                file_src_path
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
            ))
    );
    let file_img_base_path = output_path.join(&file_img_subpath_str);

    if let Some(parent) = file_img_base_path.parent() {
        fs::create_dir_all(parent)?;
    }

    //500w
    //1980w
    //3840w

    dbg!(file_src_path.to_str());

    let g = ImageReader::open(file_src_path)?.with_guessed_format()?;
    println!("Format: {}", g.format().unwrap().to_mime_type());

    let src_img = ImageReader::open(file_src_path)?.decode()?;

    // dbg!(file_img_1280x720_path.to_str());
    // //let img_500x500 = img.resize(500, 500, image::imageops::FilterType::CatmullRom);
    // let img_1280x720 = src_img.thumbnail_exact(1280, 720);

    // let file = File::create(&file_img_1280x720_path)?;
    // let mut buffered_file_writer = BufWriter::new(file);

    // Avif Defaults:
    //  speed: 4
    //  quality: 80
    // let encoder = AvifEncoder::new_with_speed_quality(&mut buffered_file_writer, 4, 80);
    // img_1280x720.write_with_encoder(encoder)?;

    let output_formats = [
        model::MediaEncodingRequest {
            width: 1280,
            height: 720,
            encoding: model::Encoding::JPEG,
            keep_aspect: false,
        },
        // model::MediaEncodingRequest {
        //     width: 1920,
        //     height: 1920,
        //     encoding: model::Encoding::JPEG,
        //     keep_aspect: true,
        // },
        // model::MediaEncodingRequest {
        //     width: 3840,
        //     height: 3840,
        //     encoding: model::Encoding::JPEG,
        //     keep_aspect: true,
        // },
        // model::MediaEncodingRequest {
        //     width: 1280,
        //     height: 720,
        //     encoding: model::Encoding::WEBP,
        //     keep_aspect: false,
        // },
        // model::MediaEncodingRequest {
        //     width: 1920,
        //     height: 1920,
        //     encoding: model::Encoding::WEBP,
        //     keep_aspect: true,
        // },
        // model::MediaEncodingRequest {
        //     width: 3840,
        //     height: 3840,
        //     encoding: model::Encoding::WEBP,
        //     keep_aspect: true,
        // },
        // model::MediaEncodingRequest {
        //     width: 1280,
        //     height: 720,
        //     encoding: model::Encoding::AVIF,
        //     keep_aspect: false,
        // },
        model::MediaEncodingRequest {
            width: 1920,
            height: 1920,
            encoding: model::Encoding::AVIF,
            keep_aspect: true,
        },
        // model::MediaEncodingRequest {
        //     width: 3840,
        //     height: 3840,
        //     encoding: model::Encoding::AVIF,
        //     keep_aspect: true,
        // },
    ];

    for output_format in &output_formats {
        let (dim_w, dim_h) = src_img.dimensions();

        // Resize and crop as needed
        let img_out = match output_format.keep_aspect {
            true => {
                //src_img.thumbnail_exact(output_format.width, output_format.height),
                src_img.resize(
                    output_format.width,
                    output_format.height,
                    image::imageops::FilterType::Lanczos3,
                )
            }

            false => {
                let target_aspect_ratio = output_format.width as f64 / output_format.height as f64;
                let src_aspect_ratio = dim_w as f64 / dim_h as f64;

                let (crop_width, crop_height) = if src_aspect_ratio > target_aspect_ratio {
                    // Source is wider than target, crop horizontally
                    let crop_height = dim_h;
                    let crop_width = (crop_height as f64 * target_aspect_ratio) as u32;
                    (crop_width, crop_height)
                } else {
                    // Source is taller than target, crop vertically
                    let crop_width = dim_w;
                    let crop_height = (crop_width as f64 / target_aspect_ratio) as u32;
                    (crop_width, crop_height)
                };

                // Calculate top-left corner for crop to center it
                let x = (dim_w - crop_width) / 2;
                let y = (dim_h - crop_height) / 2;

                let cropped_img = &src_img.crop_imm(x, y, crop_width, crop_height);
                cropped_img.resize(
                    output_format.width,
                    output_format.height,
                    image::imageops::FilterType::Lanczos3,
                )
            }
        };

        let (dim_resized_w, dim_resized_h) = img_out.dimensions();

        //img_out = src_img.thumbnail_exact(1280, 720);
        let mut file_img_subpath_ext_str = file_img_subpath_str.clone();
        file_img_subpath_ext_str = file_img_subpath_ext_str + ".";
        if output_format.keep_aspect {
            if dim_w > dim_h {
                file_img_subpath_ext_str =
                    file_img_subpath_ext_str + &output_format.width.to_string();
                file_img_subpath_ext_str = file_img_subpath_ext_str + "w.";
            } else {
                file_img_subpath_ext_str =
                    file_img_subpath_ext_str + &output_format.height.to_string();
                file_img_subpath_ext_str = file_img_subpath_ext_str + "h.";
            }
        } else {
            file_img_subpath_ext_str = file_img_subpath_ext_str
                + &output_format.width.to_string()
                + "x"
                + &output_format.height.to_string()
                + ".";
        }

        match output_format.encoding {
            model::Encoding::JPEG => {
                file_img_subpath_ext_str = file_img_subpath_ext_str + "jpg";
            }
            model::Encoding::WEBP => {
                file_img_subpath_ext_str = file_img_subpath_ext_str + "webp";
            }
            model::Encoding::AVIF => {
                file_img_subpath_ext_str = file_img_subpath_ext_str + "avif";
            }
            _ => {
                file_img_subpath_ext_str = file_img_subpath_ext_str + "unknown";
                eprintln!("Unsupported encoding");
            }
        };

        // .with_file_name(
        //     file_img_1280x720_path
        //         .file_stem()
        //         .expect("File should have a stem"),
        // )
        // .with_extension("1920w.avif");

        let file_out_path = output_path.join(&file_img_subpath_ext_str); //Path::new(&file_img_subpath_ext_str);
        let file_out = &File::create(&file_out_path)?;
        let mut file_out_writer = BufWriter::new(file_out);

        let now = Instant::now();
        match output_format.encoding {
            model::Encoding::JPEG => {
                let encoder = JpegEncoder::new_with_quality(&mut file_out_writer, 80);

                img_out.to_rgb8().write_with_encoder(encoder)?;
            }
            model::Encoding::WEBP => {
                let encoder = WebPEncoder::new_lossless(&mut file_out_writer);
                img_out.write_with_encoder(encoder)?;
            }
            model::Encoding::AVIF => {
                // Avif Defaults:
                //  speed: 4 ...10 is fastest
                //  quality: 80 ...100 is best
                let encoder = AvifEncoder::new_with_speed_quality(&mut file_out_writer, 4, 80);
                //encoder = encoder.with_num_threads(Some(8));
                img_out.write_with_encoder(encoder)?;
            } // _ => {
              //     eprintln!("Unsupported encoding");
              // }
        };
        let elapsed = now.elapsed();
        println!("{}, Time: {:.2?}", &file_img_subpath_ext_str, elapsed);

        media_file.variants.borrow_mut().insert(
            file_img_subpath_ext_str.clone(),
            Rc::new(RefCell::new(model::MediaFileVariant {
                path: file_img_subpath_ext_str.clone(),
                mime_type: match output_format.encoding {
                    model::Encoding::JPEG => "image/jpeg".to_string(),
                    model::Encoding::WEBP => "image/webp".to_string(),
                    model::Encoding::AVIF => "image/avif".to_string(),
                    _ => "image/unknown".to_string(),
                },
                width: dim_resized_w,
                height: dim_resized_h,
                bytes: u32::try_from(file_out.metadata().unwrap().len()).unwrap_or(0),
                is_thumbnail: !output_format.keep_aspect,
            })),
        );

        media_file.last_modified = chrono::Utc::now();
    }

    //img_1280x720.save(&file_img_1280x720_path)?;

    // let (dim_w, dim_h) = img.dimensions();
    // if dim_w > dim_h {
    //     let file_img_1920w_path = file_img_1280x720_path
    //         .with_file_name(
    //             file_img_1280x720_path
    //                 .file_stem()
    //                 .expect("File should have a stem"),
    //         )
    //         .with_extension("1920w.avif");
    //     let img_1920w = img.resize(1920, 1920, image::imageops::FilterType::Lanczos3);

    //     img_1920w.save(&file_img_1920w_path)?;
    // } else {
    //     let file_img_1920w_path = file_img_1280x720_path
    //         .with_file_name(
    //             file_img_1280x720_path
    //                 .file_stem()
    //                 .expect("File should have a stem"),
    //         )
    //         .with_extension("1920h.avif");

    //     let img_1920h = img.resize(1920, 1920, image::imageops::FilterType::Lanczos3);
    //     img_1920h.save(&file_img_1920w_path)?;
    // }

    return Ok(());
}
