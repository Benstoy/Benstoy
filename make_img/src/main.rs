pub mod compile_source;
mod format_image;

use std::path::PathBuf;

use clap::Parser;
use compile_source::compile;
use format_image::format_file;

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
    format_file(&args.file).unwrap()
}
