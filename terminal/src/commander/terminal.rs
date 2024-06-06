use super::Commander;

use std::io::{
    stderr, stdin, stdout, IsTerminal, Result as IoResult, Stderr, Stdin, Stdout, Write,
};
use std::process::exit;

const PROMPT: &str = "fmod> ";

const EXIT_ERROR_CODE: i32 = 1;

const IO_ERROR_MESSAGE: &str = "The I/O system has failed";
const STDIN_NOT_TERMINAL_ERROR_MESSAGE: &str = "Stdin must be terminal, sir.";

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
            out_stream: Self::make_out_stream(),
            in_stream: Self::make_in_stream(),
            error_stream: Self::make_error_stream(),
        }
    }

    fn make_in_stream() -> Stdin {
        let stdin = stdin();

        if stdin.is_terminal() {
            println!("{}", STDIN_NOT_TERMINAL_ERROR_MESSAGE);
            exit(EXIT_ERROR_CODE);
        }

        stdin
    }

    fn make_out_stream() -> Stdout {
        stdout()
    }

    fn make_error_stream() -> Stderr {
        stderr()
    }

    /// The `read_line_raw` function returns result of getting string from terminal. The function
    /// can be blocked.
    fn read_line_raw(&mut self) -> IoResult<String> {
        self.out_stream.write(PROMPT.as_bytes())?;
        self.out_stream.flush()?;

        let mut buffer = String::new();
        self.in_stream.read_line(&mut buffer)?;

        Ok(buffer.trim().to_string())
    }

    /// The `write_raw` function returns result of writing string to terminal.
    fn write_raw(&mut self, string: &str) -> IoResult<()> {
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

impl Commander for Terminal {
    fn read_command(&mut self) -> String {
        let io_result = self.read_line_raw();
        self.parse_io_result(io_result)
    }

    fn write_result(&mut self, result: String) {
        let io_result = self.write_raw(&result);
        self.parse_io_result(io_result);
    }
}
