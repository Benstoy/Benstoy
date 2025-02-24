use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use anyhow::{Context, Result};
use gpt::{GptConfig, GptDisk, partition_types};

use crate::BRANDING;

/// There aren't any useful flags for our usecase
const NO_FLAGS: u64 = 0;
const ALIGNMENT_SECTORS: u64 = 2048;

pub fn partition_file(path: &Path) -> Result<()> {
    // This file can be a disk device.
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .with_context(|| format!("Couldn't find the disk/file {:?}", path))?;

    // Write protectiveMBR to make legacy programms avoid overwriting our data.
    gpt::mbr::ProtectiveMBR::new().overwrite_lba0(&mut file)?;

    let mut disk = GptConfig::new()
        .writable(true)
        .create_from_device(file, None)?;
    let block_size = disk.logical_block_size().as_u64();

    disk.add_partition(
        &format!("{BRANDING} EFI"),
        find_required_efi_size(block_size),
        partition_types::EFI,
        NO_FLAGS,
        Some(ALIGNMENT_SECTORS),
    )?;

    disk.add_partition(
        BRANDING,
        get_remaining_bytes(&disk),
        partition_types::BASIC, // Microsoft Basic Data (exFat)
        NO_FLAGS,
        Some(ALIGNMENT_SECTORS),
    )?;

    disk.write()?;
    Ok(())
}

/// Get bytes usable for the iso partition
fn get_remaining_bytes(disk: &GptDisk<File>) -> u64 {
    // Because of the alignment of the first partition sectors
    // 34 - 2048 are empty, making the first free sector useless.
    //
    // This means we need to select the last free sector.
    // .1 returns the size instead of starting address
    //
    // Returned size is given in sectors. To return bytes we need
    disk.find_free_sectors().last().unwrap().1 * disk.logical_block_size().as_u64()
}

fn find_required_efi_size(block_size: u64) -> u64 {
    round_up_to_alignment(100 * (1024 ^ 2), block_size)
}

fn round_up_to_alignment(size: u64, block_size: u64) -> u64 {
    size.next_multiple_of(ALIGNMENT_SECTORS * block_size)
}
