use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::io::{self, Read, Seek, Write};
use wasmer_wasi::{WasiFile, WasiFsError};

/// For capturing stdout/stderr. Stores all output in a string.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pipe {
    pub buffer: Vec<u8>,
}

impl Pipe {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }
    pub fn clear(&mut self) {
        self.buffer = Vec::new();
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.buffer[..]).unwrap())
    }
}

#[typetag::serde]
impl WasiFile for Pipe {
    fn last_accessed(&self) -> u64 {
        0
    }
    fn last_modified(&self) -> u64 {
        0
    }
    fn created_time(&self) -> u64 {
        0
    }
    fn size(&self) -> u64 {
        0
    }
    fn set_len(&mut self, _len: u64) -> Result<(), WasiFsError> {
        Ok(())
    }
    fn unlink(&mut self) -> Result<(), WasiFsError> {
        Ok(())
    }
    fn bytes_available(&self) -> Result<usize, WasiFsError> {
        // return an arbitrary amount
        Ok(1024)
    }
}

// fail when reading or Seeking
impl Read for Pipe {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&self.buffer[..]).read(buf)
    }
}

impl Seek for Pipe {
    fn seek(&mut self, _pos: io::SeekFrom) -> io::Result<u64> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "can not seek capturing stdout",
        ))
    }
}

impl Write for Pipe {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.extend(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
