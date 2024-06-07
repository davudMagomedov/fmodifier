use std::io::{stderr, stdin, stdout, Result as IoResult, Stderr, Stdin, Stdout, Write};

const PROMPT: &str = "fmod> ";

const IO_ERROR_MESSAGE: &str = "The I/O system has failed";

/// The `Terminal` structure provides function for reading and writing to terminal. It provides
/// such basic functionality as prompt to enter and so on.
///
/// The structure monitors I/O error and, if necessary, terminate the program.
///
/// #### Example
/// ```ignore
/// let mut terminal = Terminal::new();
///
/// let line = terminal.read_line();
///
/// let tokens = tokenize(line)?;
/// let command = parse_tokens(&tokens)?;
///
/// core.execute(command)?;
/// ```
pub struct Terminal {
    out_stream: Stdout,
    in_stream: Stdin,
    error_stream: Stderr,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            out_stream: stdout(),
            in_stream: stdin(),
            error_stream: stderr(),
        }
    }

    /// The `read_line` function returns string an user wrote to terminal. If the user didn't write
    /// anything, the function is blocked.
    ///
    /// When IO error happens, the function parses it automatically.
    pub fn read_line(&mut self) -> String {
        let io_result = self._read_line();
        self.parse_io_result(io_result)
    }

    /// The `write` function obviously writes given string to terminal.
    ///
    /// When IO error happens, the function parses it automatically.
    pub fn write(&mut self, string: &str) {
        let io_result = self._write(string);
        self.parse_io_result(io_result)
    }

    /// The `_read_line` function returns result of getting string from terminal. The function can
    /// be blocked.
    fn _read_line(&mut self) -> IoResult<String> {
        self.out_stream.write(PROMPT.as_bytes())?;
        self.out_stream.flush()?;

        let mut buffer = String::new();
        self.in_stream.read_line(&mut buffer)?;

        Ok(buffer.trim().to_string())
    }

    /// The `_write` function returns result of writing string to terminal.
    fn _write(&mut self, string: &str) -> IoResult<()> {
        self.out_stream.write(string.as_bytes())?;
        self.out_stream.flush()?;

        Ok(())
    }

    /// The `parse_io_result` takes IO result and returns value in Ok variant. If result is Err
    /// variant, the function processes this case: maybe by program termination, maybe by warning,
    /// etc.
    fn parse_io_result<T>(&self, result: IoResult<T>) -> T {
        result.expect(IO_ERROR_MESSAGE)
    }
}
