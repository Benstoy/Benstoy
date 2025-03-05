use anyhow::Result;
use fscommon::StreamSlice;

use crate::{disk::Disk, partition::PartitionData};

pub use exfat::ExFatFormatter;
pub use fat32::Fat32Formatter;

mod exfat;
mod fat32;

pub fn format<T: Formatter>(
    disk: impl Disk,
    block_size: u64,
    partition_data: &PartitionData,
) -> Result<FSWrapperTODO> {
    T::format(
        StreamSlice::new(
            disk,
            dbg!(partition_data.start_byte),
            dbg!(partition_data.end_byte),
        )?,
        block_size,
        partition_data.label.to_owned(),
    )
}

pub trait Formatter {
    fn format(
        partition: StreamSlice<impl Disk>,
        block_size: u64,
        label: String,
    ) -> Result<FSWrapperTODO>;
}

// TODO
/// Generic abstraction over filesystems and accessing them
pub struct FSWrapperTODO;
