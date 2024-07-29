use kt_media_processor::{scan_and_process_media, sync};
use sync::do_s3_sync;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use log;

pub mod utils; // pub only to avoid serde unused import warnings
mod watcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args = utils::parse_args();


    let running = Arc::new(AtomicBool::new(true));
    // let r = running.clone();
    // ctrlc::set_handler(move || {
    //     r.store(false, Ordering::SeqCst);
    //     //TODO: Need to restructure to allow drop of tx/rx channels
    //     //TODO: A ctrl-c should also be able to stop the recursive album processing, etc
    //     //watcher::interrupt_watcher();
    // })?;

    println!("Starting kt-media-processor...");

    while running.load(Ordering::SeqCst) {
        println!("Scanning and processing...");
        scan_and_process_media(&args.in_dir, &args.out_dir)?;

        println!("Syncing to S3...");
        //do_s3_sync(&args.out_dir, &args.bucket_name)?;
        //do_s3_sync(&args.out_dir, Some("kt-media-processor"), "s3://kaliatech-media/kt-processor-test1/").await?;
        if let (Some(aws_profile), Some(s3_url)) = (&args.aws_profile, &args.s3_url) {
            do_s3_sync(&args.out_dir, Some(aws_profile.as_str()), &s3_url).await?;
        }

        log::info!("Watching for changes...");
        watcher::start_watcher(&args.in_dir)?;
    }

    log::info!("...kt-media-processor finished.");
    Ok(())
}
