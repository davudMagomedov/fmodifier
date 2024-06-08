use super::Commander;

use std::fs::File;
use std::io::{stderr, stdout, Read, Result as IoResult, Stderr, Stdout, Write};

const NEW_LINE_CHAR: char = '\n';

const READ_METADATA_ERROR: &str = "Couldn't read metadata.";
const IO_ERROR_MESSAGE: &str = "The I/O system has failed";

struct Liner {
    lines: std::vec::IntoIter<String>,
}

impl Liner {
    pub fn new(content: String) -> Liner {
        Liner {
            lines: content
                .lines()
                .map(|string_slice| string_slice.to_string())
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }
}

impl Iterator for Liner {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next()
    }
}

pub struct FileReader {
    // This is instead of `in_stream`.
    liner: Liner,
    out_stream: Stdout,
    error_stream: Stderr,
}

impl FileReader {
    pub fn new(path: &str) -> IoResult<Self> {
        let mut file = Self::open_file(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        Ok(FileReader {
            liner: Liner::new(buf),
            out_stream: Self::make_out_stream(),
            error_stream: Self::make_error_stream(),
        })
    }

    /// The `write_raw` function returns result of writing string to terminal.
    fn write_raw(&mut self, string: &str) -> IoResult<()> {
        self.out_stream.write(string.as_bytes())?;
        self.out_stream.flush()?;

        Ok(())
    }

    fn open_file(path: &str) -> IoResult<File> {
        File::open(path)
    }

    fn make_out_stream() -> Stdout {
        stdout()
    }

    fn make_error_stream() -> Stderr {
        stderr()
    }

    /// The `parse_io_result` takes IO result and returns value in Ok variant. If result is Err
    /// variant, the function processes this case: maybe by program termination, maybe by warning,
    /// etc.
    fn parse_io_result<T>(&self, result: IoResult<T>) -> T {
        result.expect(IO_ERROR_MESSAGE)
    }
}

impl Commander for FileReader {
    fn is_terminal() -> bool {
        false
    }

    fn read_command(&mut self) -> Option<String> {
        self.liner.next()
    }

    fn write_result(&mut self, result: String) {
        let io_result = self.write_raw(&result);
        self.parse_io_result(io_result);
    }
}
