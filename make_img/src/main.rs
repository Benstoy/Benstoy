pub mod compile_source;
mod disk;
mod format;
mod partition;

use std::path::PathBuf;

use clap::Parser;
use compile_source::compile;
use disk::from_file;
use format::{ExFatFormatter, Fat32Formatter, format};
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
    let mut disk = from_file(&args.file).unwrap();

    dbg!(compile(args.release).unwrap());
    let partition_table = partition_file(&mut disk).unwrap();

    format::<Fat32Formatter>(
        &mut disk,
        partition_table.block_size,
        &partition_table.partition_data[0],
    )
    .unwrap();

    format::<ExFatFormatter>(
        &mut disk,
        partition_table.block_size,
        &partition_table.partition_data[1],
    )
    .unwrap();
}
