use anyhow::Result;
use fatfs::{FormatVolumeOptions, FsOptions};
use fscommon::StreamSlice;

use crate::disk::Disk;

use super::{FSWrapperTODO, Formatter};

pub struct Fat32Formatter;

impl Formatter for Fat32Formatter {
    fn format(mut partition: StreamSlice<impl Disk>, _: u64, _: String) -> Result<FSWrapperTODO> {
        fatfs::format_volume(&mut partition, FormatVolumeOptions::new())?;
        let _ = fatfs::FileSystem::new(&mut partition, FsOptions::new())?;

        Ok(FSWrapperTODO)
    }
}
