use std::io::{stderr, stdin, stdout, Result as IoResult, Stderr, Stdin, Stdout, Write};

const PROMPT: &str = "fmod> ";

/// The `Terminal` structure provides function for reading and writing to terminal. It provides
/// such basic functionality as prompt to enter and so on.
///
/// #### Example
/// ```ignore
/// let mut terminal = Terminal::new();
///
/// let line = terminal.read_line()?;
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
    pub fn read_line(&mut self) -> IoResult<String> {
        self.out_stream.write(PROMPT.as_bytes())?;
        self.out_stream.flush()?;

        let mut buffer = String::new();
        self.in_stream.read_line(&mut buffer)?;

        Ok(buffer.trim().to_string())
    }

    /// The `write` function obviously writes given string to terminal.
    pub fn write(&mut self, string: &str) -> IoResult<()> {
        self.out_stream.write(string.as_bytes())?;
        self.out_stream.flush()?;

        Ok(())
    }
}
