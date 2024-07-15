use crate::model;

pub fn get_output_formats() -> Vec<model::MediaEncodingRequest> {
    vec![
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
            width: 640,
            height: 360,
            encoding: model::Encoding::AVIF,
            keep_aspect: false,
        },
        // model::MediaEncodingRequest {
        //     width: 1280,
        //     height: 720,
        //     encoding: model::Encoding::AVIF,
        //     keep_aspect: false,
        // },
        model::MediaEncodingRequest {
            width: 1920,
            height: 1080,
            encoding: model::Encoding::AVIF,
            keep_aspect: true,
        },
        // model::MediaEncodingRequest {
        //     width: 3840,
        //     height: 2160,
        //     encoding: model::Encoding::AVIF,
        //     keep_aspect: true,
        // },
    ]
}
