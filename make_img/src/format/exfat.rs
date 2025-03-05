use anyhow::{Result, anyhow};
use exfat::format::{FormatOptions, Label};
use fscommon::StreamSlice;

use crate::disk::{Disk, Seekable};

use super::{FSWrapperTODO, Formatter};

pub struct ExFatFormatter;

impl Formatter for ExFatFormatter {
    fn format(
        mut partition: StreamSlice<impl Disk>,
        block_size: u64,
        label: String,
    ) -> Result<FSWrapperTODO> {
        let size = partition.stream_len()?;

        exfat::format::Formatter::try_new(
            0,
            block_size as u16,
            size,
            1,
            FormatOptions::new(
                false,
                false,
                size,
                Label::new(label).ok_or(anyhow!("Failed creating EFI Label"))?,
            ),
        )?;

        Ok(FSWrapperTODO)
    }
}
