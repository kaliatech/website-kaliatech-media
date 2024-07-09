use crate::model::{Encoding, MediaEncodingRequest};

pub fn generate_filename(
    file_img_subpath_str: &str,
    dest_req: &MediaEncodingRequest,
    src_dim_w: u32,
    src_dim_h: u32,
) -> (String, u32, u32) {
    let req_dim_w = dest_req.width;
    let req_dim_h = dest_req.height;

    let mut actual_dim_w;
    let mut actual_dim_h;

    if dest_req.keep_aspect {
        // source is landscape, so size by width
        if src_dim_w > src_dim_h {
            actual_dim_w = req_dim_w;
            actual_dim_h = (src_dim_h as f64 * (req_dim_w as f64 / src_dim_w as f64)) as u32;
        }
        // source is portrait or square, so size by height
        else {
            actual_dim_h = req_dim_h;
            actual_dim_w = (src_dim_w as f64 * (req_dim_h as f64 / src_dim_h as f64)) as u32;
        }
    } else {
        // let req_ar = req_dim_w as f64 / req_dim_h as f64;

        // //let src_ar = src_dim_h as f64 / src_dim_w as f64;

        // let (crop_width, crop_height) = if req_ar < 1.0 {
        //     // crop horizontally
        //     let crop_height = req_dim_h;
        //     let crop_width = (crop_height as f64 * (1.0 / req_ar)) as u32;
        //     (crop_width, crop_height)
        // } else {
        //     // crop vertically
        //     let crop_width = req_dim_w;
        //     let crop_height = (crop_width as f64 * (1.0 / req_ar)) as u32;
        //     (crop_width, crop_height)
        // };

        // dbg!(req_ar);
        // dbg!(crop_width);
        // dbg!(crop_height);

        // // Calculate top-left corner for crop to center it
        // // let x = (src_dim_w - crop_width) / 2;
        // // let y = (src_dim_h - crop_height) / 2;
        // crop_dim_w = crop_width;
        // crop_dim_h = crop_height
        actual_dim_w = req_dim_w;
        actual_dim_h = req_dim_h;
    }

    // If req results in image larger than source, return source. We don't scale up.
    if actual_dim_w > src_dim_w || actual_dim_h > src_dim_h {
        actual_dim_w = src_dim_w;
        actual_dim_h = src_dim_h;
    }

    let mut expected_subpath_ext_str = file_img_subpath_str.to_string();
    expected_subpath_ext_str = expected_subpath_ext_str + ".";

    expected_subpath_ext_str = expected_subpath_ext_str
        + &actual_dim_w.to_string()
        + "x"
        + &actual_dim_h.to_string()
        + ".";

    match dest_req.encoding {
        Encoding::JPEG => {
            expected_subpath_ext_str = expected_subpath_ext_str + "jpg";
        }
        Encoding::WEBP => {
            expected_subpath_ext_str = expected_subpath_ext_str + "webp";
        }
        Encoding::AVIF => {
            expected_subpath_ext_str = expected_subpath_ext_str + "avif";
        } // _ => {
          //     file_img_subpath_ext_str = file_img_subpath_ext_str + "unknown";
          //     eprintln!("Unsupported encoding");
          // }
    };

    return (
        expected_subpath_ext_str.to_string(),
        actual_dim_w,
        actual_dim_h,
    );
}
