use std::borrow::BorrowMut;
use std::cell::RefMut;
use std::error::Error;
use std::path::Path;

use std::fs;
use std::fs::File;
use std::io::BufWriter;

use crate::model;
use crate::processor::generate_filename;
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

use image::io::Reader as ImageReader;
use image::GenericImageView;

use image::codecs::avif::AvifEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::webp::WebPEncoder;

use std::time::Instant;

use std::cmp;

pub fn process_file(
    file_src_path: &Path,
    output_path: &Path,
    _media_file_meta: Ref<model::MediaFileMeta>,
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

    // let g = ImageReader::open(file_src_path)?.with_guessed_format()?;
    // println!("Format: {}", g.format().unwrap().to_mime_type());

    // This might be problematic is only being done to here determine if file has been modified
    let src_img = ImageReader::open(file_src_path)?.decode()?;
    let (src_dim_w, src_dim_h) = src_img.dimensions();

    //TODO: Maybe check with a known file first to determine modified instead of opening/decoding every file

    media_file.width = src_dim_w;
    media_file.height = src_dim_h;

    // dbg!(file_img_1280x720_path.to_str());
    // //let img_500x500 = img.resize(500, 500, image::imageops::FilterType::CatmullRom);
    // let img_1280x720 = src_img.thumbnail_exact(1280, 720);

    // let file = File::create(&file_img_1280x720_path)?;
    // let mut buffered_file_writer = BufWriter::new(file);

    // Avif Defaults:
    //  speed: 4
    //  quality: 80
    // let encoder = AvifEncoder::new_with_speed_quality(&mut buffered_file_writer, 4, 80);

    let output_formats = [
        // model::MediaEncodingRequest {
        //     width: 1280,
        //     height: 720,
        //     encoding: model::Encoding::JPEG,
        //     keep_aspect: false,
        // },
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
        model::MediaEncodingRequest {
            width: 1280,
            height: 720,
            encoding: model::Encoding::AVIF,
            keep_aspect: false,
        },
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
        // Generated expected output name
        let (expected_subpath_ext_str, actual_dim_w, actual_dim_h) =
            generate_filename(&file_img_subpath_str, output_format, src_dim_w, src_dim_h);

        println!("Considering: {}", expected_subpath_ext_str);

        // If file exists and last_modified is after the last_modified of the source file
        // and the given meta, then skip actual final resizing and generation
        let expected_path = &output_path.join(&expected_subpath_ext_str);

        if expected_path.exists()
            && fs::metadata(expected_path).unwrap().modified()?
                > file_src_path.metadata()?.modified()?
        //&& (File::new(expected_path).metadata()?.modified()? > media_file.last_modified)
        {
            println!("Skipping: {}", expected_subpath_ext_str);
            continue;
        }

        println!("Processing: {}", expected_subpath_ext_str);
        // Resize and crop as needed
        let img_out = match output_format.keep_aspect {
            true => {
                dbg!("keep_aspect");
                dbg!(actual_dim_w);
                dbg!(actual_dim_h);
                dbg!(output_format);

                if actual_dim_w < output_format.width || actual_dim_h < output_format.height {
                    dbg!("clone");
                    src_img.clone()
                } else {
                    //src_img.thumbnail_exact(output_format.width, output_format.height),
                    //we calculated actuals, so we use exact
                    dbg!("resize");
                    let now = Instant::now();
                    let resized_img = src_img.resize_exact(
                        actual_dim_w,
                        actual_dim_h,
                        image::imageops::FilterType::Lanczos3,
                    );
                    let elapsed = now.elapsed();
                    println!(
                        "{}, Resize Time: {:.2?}",
                        &expected_subpath_ext_str, elapsed
                    );
                    resized_img
                }
            }

            false => {
                if actual_dim_w < output_format.width || actual_dim_h < output_format.height {
                    //src_img.clone()
                    dbg!("too small");
                    dbg!(actual_dim_w);
                    dbg!(actual_dim_h);
                    dbg!(output_format);
                    src_img.resize_exact(
                        actual_dim_w,
                        actual_dim_h,
                        image::imageops::FilterType::Lanczos3,
                    )
                    //TODO: debug here I guess
                } else {
                    // Calculate top-left corner for crop to center it
                    let src_ar = src_dim_w as f64 / src_dim_h as f64;
                    let req_ar = actual_dim_w as f64 / actual_dim_h as f64;
                    dbg!(src_ar);
                    dbg!(req_ar);

                    // let mut precrop_size = cmp::min(src_dim_w, actual_dim_w);
                    // if src_ar < 1.0 && req_ar < 1.0 {
                    //     precrop_size = cmp::min(src_dim_w, actual_dim_w);
                    // } else if src_ar > 1.0 && req_ar > 1.0 {
                    //     precrop_size = cmp::min(src_dim_h, actual_dim_h);
                    // } else if src_ar > 1.0 && req_ar < 1.0 {
                    //     precrop_size = cmp::min(src_dim_h, actual_dim_h);
                    // } else if src_ar < 1.0 && req_ar > 1.0 {
                    //     precrop_size = cmp::min(src_dim_w, actual_dim_w);
                    // }

                    let (scale_width, scale_height) = if src_ar > req_ar {
                        // Scale based on height
                        let scale_height = actual_dim_h;
                        let scale_width =
                            (src_dim_w as f64 * (scale_height as f64 / src_dim_h as f64)) as u32;
                        (scale_width, scale_height)
                    } else {
                        // Scale based on width
                        let scale_width = actual_dim_w;
                        let scale_height =
                            (src_dim_h as f64 * (scale_width as f64 / src_dim_w as f64)) as u32;
                        (scale_width, scale_height)
                    };

                    let resized_img = src_img.resize_exact(
                        scale_width,
                        scale_height,
                        image::imageops::FilterType::Lanczos3,
                    );

                    let (resized_dim_w, resized_dim_h) = resized_img.dimensions();

                    dbg!(src_dim_w);
                    dbg!(src_dim_h);

                    dbg!(actual_dim_w);
                    dbg!(actual_dim_h);

                    dbg!(resized_dim_w);
                    dbg!(resized_dim_h);

                    // Calculate crop start points
                    let crop_x = if scale_width > actual_dim_w {
                        (scale_width - actual_dim_w) / 2
                    } else {
                        0
                    };
                    let crop_y = if scale_height > actual_dim_h {
                        (scale_height - actual_dim_h) / 2
                    } else {
                        0
                    };

                    resized_img.crop_imm(crop_x, crop_y, actual_dim_w, actual_dim_h)
                }
            }
        };

        let (dim_resized_w, dim_resized_h) = img_out.dimensions();

        let file_out_path = expected_path; //Path::new(&file_img_subpath_ext_str);
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
        println!("{}, Write Time: {:.2?}", &expected_subpath_ext_str, elapsed);

        media_file.variants.borrow_mut().insert(
            expected_subpath_ext_str.clone(),
            Rc::new(RefCell::new(model::MediaFileVariant {
                path: expected_subpath_ext_str.clone(),
                mime_type: match output_format.encoding {
                    model::Encoding::JPEG => "image/jpeg".to_string(),
                    model::Encoding::WEBP => "image/webp".to_string(),
                    model::Encoding::AVIF => "image/avif".to_string(),
                    // _ => "image/unknown".to_string(),
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
