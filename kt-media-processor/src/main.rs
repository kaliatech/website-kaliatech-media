use kt_media_processor::scan_and_process_media;

pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting kt-media-processor...");

    let args = utils::parse_args();

    scan_and_process_media(&args.in_dir, &args.out_dir)?;

    println!("...kt-media-processor finished.");
    Ok(())
}
