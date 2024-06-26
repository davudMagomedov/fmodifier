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
///
/// The function returns count of written bytes. If can be 0 if start is 0 or write_from is 0.
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
    /// The `new` function creates new file in directory and opens in in read-write mode. File will
    /// have bytes in the amount of given pieces filled by zero.
    ///
    /// If file with the given path already exists, the function returns `Err`.
    pub fn new<T: AsRef<Path>>(path: T, size: usize) -> IoResult<Self> {
        let mut raw_file = OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .open(path)?;

        raw_file.write_all(&vec![0; size])?;

        Ok(NewFile { raw: raw_file })
    }

    /// The `len` function returns length of the file.
    pub fn len(&self) -> IoResult<usize> {
        self.raw.metadata().map(|meta| meta.len() as usize)
    }

    // Write {

    /// The `write_byte` function writes byte in first argument into index in second argument.
    ///
    /// If the index goes beyound the bounder, the function panics.
    pub fn write_byte(&mut self, byte: u8, index: usize) -> IoResult<()> {
        let buffer = std::array::from_ref(&byte);

        match write(&mut self.raw, index, buffer)? {
            0 => panic!("The index goes beyond the bounder!"),
            1 => { /* Good */ }
            _ => unreachable!(),
        };

        Ok(())
    }

    /// The `write_bytes` function writes bytes from bytes in first argument starting with index in
    /// second argument. The function returns count of written bytes.
    ///
    /// If second argument goes beyound the bounder, the function will return `Ok(0)` but not
    /// error.
    pub fn write_bytes(&mut self, bytes: &[u8], start_with: usize) -> IoResult<usize> {
        write(&mut self.raw, start_with, bytes)
    }

    // }

    // Read {

    /// The `read_byte` returns a byte from a given index. If index is wrong anyway, the function
    /// causes panic.
    pub fn read_byte(&mut self, index: usize) -> IoResult<u8> {
        let mut buf = 0_u8;

        match read(&mut self.raw, index, std::array::from_mut(&mut buf))? {
            0 => panic!("The index goes beyond the bounder!"),
            1 => { /* Good */ }
            _ => unreachable!(),
        };

        Ok(buf)
    }

    /// The `read_bytes` returns a slice of file's bytes starting from the first argument and
    /// ending by the second argument. If `start` goes beyond the boundaries, the function returns
    /// `None`.
    pub fn read_bytes(&mut self, start: usize, end: usize) -> IoResult<Option<Box<[u8]>>> {
        if start >= self.len()? {
            return Ok(None);
        }

        let mut buffer = Box::from_iter((start..end.min(self.len()?)).map(|_| 0_u8));

        read(&mut self.raw, start, &mut buffer[..])?;

        Ok(Some(buffer))
    }

    // }
}

/// The `ReadFile` structure lets to operations for reading only. This structure just openes
/// existing file and reads it.
pub struct ReadFile {
    // This file is opened with read flag only.
    raw: RawFile,
}

impl ReadFile {
    /// The `new` function opens file in directory (if it doesn't exist, the function return `Err`)
    /// and creates this structures.
    pub fn new<T: AsRef<Path>>(path: T) -> IoResult<Self> {
        Ok(ReadFile {
            raw: OpenOptions::new().read(true).open(path)?,
        })
    }

    /// The `len` function returns length of the file.
    pub fn len(&self) -> IoResult<usize> {
        self.raw.metadata().map(|meta| meta.len() as usize)
    }

    // Read {

    /// The `read_byte` returns a byte from a given index. If index is wrong anyway, the function
    /// causes panic.
    pub fn read_byte(&mut self, index: usize) -> IoResult<u8> {
        let mut buf = 0_u8;

        match read(&mut self.raw, index, std::array::from_mut(&mut buf))? {
            0 => panic!("The index goes beyond the bounder!"),
            1 => { /* Good */ }
            _ => unreachable!(),
        };

        Ok(buf)
    }

    /// The `read_bytes` returns a slice of file's bytes starting from the first argument and
    /// ending by the second argument. If `start` goes beyond the boundaries, the function returns
    /// `None`.
    pub fn read_bytes(&mut self, start: usize, end: usize) -> IoResult<Option<Box<[u8]>>> {
        if start >= self.len()? {
            return Ok(None);
        }

        let mut buffer = Box::from_iter((start..end.min(self.len()?)).map(|_| 0_u8));

        read(&mut self.raw, start, &mut buffer[..])?;

        Ok(Some(buffer))
    }

    // }
}

/// The `File` enumeration contains two variants:
/// - New file for writing.
/// - Opened existing file for reading.
pub enum File {
    New(NewFile),
    ToRead(ReadFile),
}
