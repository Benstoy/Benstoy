use std::path::PathBuf;

use benstoy_make_img::{
    compile,
    disk::disk_from_file,
    format::{ExFatFormatter, Fat32Formatter, format},
    partition_file,
};
use clap::Parser;

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
    let mut disk = disk_from_file(&args.file).unwrap();

    let _binaries = compile(args.release).unwrap();
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
