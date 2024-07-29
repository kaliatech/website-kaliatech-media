use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input directory
    #[arg(long = "inDir", short)]
    pub in_dir: String,

    /// Output directory
    #[arg(short, long = "outDir")]
    pub out_dir: String,

    /// AWS profile name (optional)
    #[arg(short, long = "awsProfile")]
    pub aws_profile: Option<String>,

    /// AWS S3 URL for automatic sync with delete (optional)
    #[arg(short, long = "s3Url")]
    pub s3_url: Option<String>,

}

pub fn parse_args() -> Args {
    let args = Args::parse();
    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }
    return args;
}
