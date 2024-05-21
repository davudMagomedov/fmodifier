use std::fs::File as RawFile;
use std::fs::OpenOptions;
use std::io::{Read, Result as IoResult, Seek, SeekFrom, Write};
use std::path::Path;

/// The `read` function writes bytes from the file in first argument starting from index in second
/// argument to buffer in third argument. It means that if you call `read(file, 3, &mut write_to)`,
/// the `file[3]` will be in `write_to[0]`, the `file[4]` will be in `wrote_to[1]`, etc.
///
/// If `start` goes beyond the bounder, the function returns `Ok(0)`. It signs that no byte is
/// read.
///
/// Accepted guarantees:
/// 1. Given file is opened in read mode.
fn read(raw: &mut RawFile, start: usize, write_to: &mut [u8]) -> IoResult<usize> {
    raw.seek(SeekFrom::Start(start as u64))?;
    raw.read(write_to)
}

/// The `write` funtion reads bytes from third argument and writes them to file in first argument
/// starting from index in second argument.
fn write(raw: &mut RawFile, start: usize, write_from: &[u8]) -> IoResult<usize> {
    raw.seek(SeekFrom::Start(start as u64))?;
    raw.write(write_from)
}

/// The `NewFile` structure lets do operations for writing and reading. Also, making `NewFile`
/// you create new file in directory.
pub struct NewFile {
    // INVARIANTS:
    // - `NewFile::raw` is always opened in write-read mode.
    // - `NewFile::raw` is new file in directory.

    // This file is new, created with write and create_new flags.
    raw: RawFile,
}

impl NewFile {
    /// The `new` function creates new file in directory and opens it in read-write mode. If the
    /// file already exists, the function returns `Err`.
    pub fn new<T: AsRef<Path>>(path: T) -> IoResult<Self> {
        Ok(NewFile {
            raw: OpenOptions::new()
                .write(true)
                .read(true)
                .create_new(true)
                .open(path)?,
        })
    }

    /// The `len` function returns length of the file.
    pub fn len(&self) -> IoResult<usize> {
        self.raw.metadata().map(|meta| meta.len() as usize)
    }

    // Write {

    /// The `write_byte` function writes byte in first argument into index in second argument.
    ///
    /// If the index goes beyond the bounder, the function doesn't returns error.
    pub fn write_byte(&mut self, byte: u8, index: usize) -> IoResult<()> {
        let buffer = std::array::from_ref(&byte);
        write(&mut self.raw, index, buffer)?;

        Ok(())
    }

    // }
}

/// The `ReadFile` structure lets to operations for reading only. This structure just openes
/// existing file and reads it.
pub struct ReadFile {
    // This file is opened with read flag only.
    raw: RawFile,
}
