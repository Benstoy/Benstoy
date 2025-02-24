pub mod compile_source;
mod partition;

use std::path::PathBuf;

use clap::Parser;
use compile_source::compile;
use partition::partition_file;

const BRANDING: &str = "Benstoy";

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Device/File to write the benstoy image to
    file: PathBuf,

    /// Whether to compile in release mode
    #[arg(long, default_value_t = false)]
    release: bool,
}

fn main() {
    let args = Args::parse();

    dbg!(compile(args.release).unwrap());
    partition_file(&args.file).unwrap()
}
