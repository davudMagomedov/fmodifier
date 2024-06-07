use super::Commander;

use std::io::{
    stderr, stdin, stdout, IsTerminal, Lines, Result as IoResult, Stderr, StdinLock, Stdout, Write,
};

const PROMPT: &str = "fmod> ";

const EXIT_ERROR_CODE: i32 = 1;

const IO_ERROR_MESSAGE: &str = "The I/O system has failed";
const STDIN_NOT_TERMINAL_ERROR_MESSAGE: &str = "Stdin must be terminal, sir.";

type Liner = Lines<StdinLock<'static>>;

/// The `Terminal` structure provides function for reading and writing to terminal. It provides
/// such basic functionality as prompt to enter and so on.
///
/// #### `Commander` implemention
/// The `Terminal` returns `None` on `Terminal::read_command` if there's an IO error or an `stdin`
/// stream came to EOF.
///
/// #### Example
/// ```ignore
/// let mut terminal = Terminal::new();
///
/// let line = terminal.read_line().unwrap();
///
/// let tokens = tokenize(line)?;
/// let command = parse_tokens(&tokens).unwrap();
///
/// core.execute(command)?;
/// ```
pub struct Terminal {
    liner: Liner,
    out_stream: Stdout,
    error_stream: Stderr,

    interacitve_mode: bool,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            liner: Self::make_in_lines(),
            out_stream: Self::make_out_stream(),
            error_stream: Self::make_error_stream(),

            interacitve_mode: Self::is_interactive_mode(),
        }
    }

    fn is_interactive_mode() -> bool {
        stdin().is_terminal()
    }

    fn make_in_lines() -> Liner {
        stdin().lines()
    }

    fn make_out_stream() -> Stdout {
        stdout()
    }

    fn make_error_stream() -> Stderr {
        stderr()
    }

    fn print_prompt(&mut self) -> IoResult<()> {
        self.out_stream.write(PROMPT.as_bytes())?;
        self.out_stream.flush()?;

        Ok(())
    }

    /// The `read_line_raw` function returns result of getting string from terminal. The function
    /// can be blocked.
    fn read_line_raw(&mut self) -> Option<String> {
        if self.interacitve_mode {
            self.print_prompt().ok()?;
        }

        self.liner.next()?.ok()
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
    fn read_command(&mut self) -> Option<String> {
        self.read_line_raw()
    }

    fn write_result(&mut self, result: String) {
        let io_result = self.write_raw(&result);
        self.parse_io_result(io_result);
    }
}
