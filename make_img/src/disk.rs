use anyhow::{Context, Result};

use std::{fs::OpenOptions, path::Path};

pub(crate) use sealed::*;

pub(crate) mod sealed {
    use anyhow::Result;

    use core::fmt;
    use std::io::{Read, Seek, SeekFrom, Write};

    pub trait Disk: Read + Write + Seek + fmt::Debug {}
    impl<T: Read + Write + Seek + fmt::Debug> Disk for T {}

    pub trait Seekable: Seek {
        /// Copied from `std::io::Seek::stream_len` to avoid using nightly
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
}

pub fn disk_from_file(path: &Path) -> Result<impl Disk> {
    OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .with_context(|| format!("Couldn't find the disk/file {:?}", path))
}
