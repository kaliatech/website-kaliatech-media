use kt_media_processor::{scan_and_process_media, sync};
use sync::do_s3_sync;

pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = utils::parse_args();

    println!("Starting kt-media-processor...");
    scan_and_process_media(&args.in_dir, &args.out_dir)?;
    println!("...kt-media-processor finished.");

    println!("Starting sync...");
    //do_s3_sync(&args.out_dir, &args.bucket_name, &args.bucket_prefix)?;
    do_s3_sync(&args.out_dir, "s3://kaliatech-media/kt-processor-test1/").await?;
    println!("...sync finished.");

    Ok(())
}
