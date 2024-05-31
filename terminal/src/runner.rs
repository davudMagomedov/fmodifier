use crate::output::ToOutput;
use crate::terminal::Terminal;
use crate::tokenizer::tokenize;

use core::parse_tokens;
use core::Core;

/// The `Runner` structure is iterator in which each iteration means following actions:
/// 1. Take user input.
/// 2. Parse user input.
/// 3. Execute user input.
/// 4. Output result.
///
/// It means that you can stop this runner whenever you want.
pub struct Runner {
    core: Core,
    terminal: Terminal,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            core: Core::new(),
            terminal: Terminal::new(),
        }
    }

    fn output<T: ToOutput>(&mut self, object: T) {
        self.terminal.write(&format!("{}\n", object.to_output()));
    }
}

impl Iterator for Runner {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let input = self.terminal.read_line();
        let tokens = match tokenize(&input) {
            Ok(tokens) => tokens,
            Err(e) => {
                self.output(e);
                return Some(());
            }
        };
        let command = match parse_tokens(&tokens) {
            Ok(command) => command,
            Err(e) => {
                self.output(e);
                return Some(());
            }
        };
        let core_output = match self.core.execute(command) {
            Ok(core_output) => core_output,
            Err(e) => {
                self.output(e);
                return Some(());
            }
        };

        self.output(core_output);

        Some(())
    }
}
