use anyhow::Result;
use gpt::{DiskDevice, GptConfig, GptDisk, partition_types};

use crate::BRANDING;

/// There aren't any useful flags for our usecase
const NO_FLAGS: u64 = 0;
const ALIGNMENT_SECTORS: u64 = 2048;

pub struct PartitionData {
    pub label: String,
    pub start_byte: u64,
    pub end_byte: u64,
}

pub struct PartitionTable {
    pub block_size: u64,
    pub partition_data: [PartitionData; 2],
}

pub fn partition_file(mut disk_file: impl DiskDevice) -> Result<PartitionTable> {
    // Write protectiveMBR to make legacy programms avoid overwriting our data.
    gpt::mbr::ProtectiveMBR::new().overwrite_lba0(&mut disk_file)?;

    let mut disk = GptConfig::new()
        .writable(true)
        .create_from_device(disk_file, None)?;
    let block_size = disk.logical_block_size().as_u64();

    let efi_part_id = disk.add_partition(
        &format!("{BRANDING} EFI"),
        required_efi_size(block_size),
        partition_types::EFI,
        NO_FLAGS,
        Some(ALIGNMENT_SECTORS),
    )?;
    let efi_part = &disk.partitions()[&efi_part_id].clone();

    let iso_part_id = disk.add_partition(
        BRANDING,
        get_remaining_bytes(&disk),
        partition_types::BASIC, // Microsoft Basic Data (exFat)
        NO_FLAGS,
        Some(ALIGNMENT_SECTORS),
    )?;
    let iso_part = &disk.partitions()[&iso_part_id].clone();

    disk.write()?;

    Ok(PartitionTable {
        block_size,
        partition_data: [efi_part, iso_part].map(|part| PartitionData {
            label: part.name.to_owned(),
            start_byte: part.first_lba * block_size,
            end_byte: part.last_lba * block_size,
        }),
    })
}

/// Get bytes usable for the iso partition
fn get_remaining_bytes(disk: &GptDisk<impl DiskDevice>) -> u64 {
    // Because of the alignment of the first partition sectors
    // 34 - 2048 are empty, making the first free sector useless.
    //
    // This means we need to select the last free sector.
    // .1 returns the size instead of starting address
    //
    // Returned size is given in sectors. To return bytes we need
    disk.find_free_sectors().last().unwrap().1 * disk.logical_block_size().as_u64()
}

fn required_efi_size(block_size: u64) -> u64 {
    round_up_to_alignment(100 * (1024 ^ 2), block_size)
}

fn round_up_to_alignment(size: u64, block_size: u64) -> u64 {
    size.next_multiple_of(ALIGNMENT_SECTORS * block_size)
}
