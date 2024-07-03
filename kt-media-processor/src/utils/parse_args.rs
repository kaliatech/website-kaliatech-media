use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(long = "inDir", short)]
    pub in_dir: String,

    /// Number of times to greet
    #[arg(short, long = "outDir")]
    pub out_dir: String,
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }
    return args;
}
