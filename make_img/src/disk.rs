use anyhow::{Context, Result};

use core::fmt;
use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

pub trait Disk: Read + Write + Seek + fmt::Debug {}
impl<T: Read + Write + Seek + fmt::Debug> Disk for T {}

pub(crate) trait Seekable: Seek {
    /// Copied from `std::io::Seek::stream_len`
    fn stream_len(&mut self) -> Result<u64> {
        let old_pos = self.stream_position()?;
        let len = self.seek(SeekFrom::End(0))?;

        // Avoid seeking a third time when we were already at the end of the
        // stream. The branch is usually way cheaper than a seek operation.
        if old_pos != len {
            self.seek(SeekFrom::Start(old_pos))?;
        }

        Ok(len)
    }
}
impl<T: Seek> Seekable for T {}

pub fn from_file(path: &Path) -> Result<impl Disk> {
    OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .with_context(|| format!("Couldn't find the disk/file {:?}", path))
}
