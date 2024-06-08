use super::commander::Commander;
use super::output::ToOutput;
use super::tokenizer::tokenize;

use crate::core::parse_tokens;
use crate::core::Core;

/// The `Runner` structure is iterator in which each iteration means following actions:
/// 1. Take commander's command.
/// 2. Parse the command.
/// 3. Execute the command.
/// 4. Output result.
///
/// Iterations ends when ends commands.
pub struct Runner<C: Commander> {
    core: Core,
    commander: C,
    completed: bool,
}

impl<C: Commander> Runner<C> {
    pub fn new(commander: C) -> Self {
        Runner {
            core: Core::new(),
            commander,
            completed: false,
        }
    }

    pub fn finish(&mut self) {
        self.completed = true;
    }

    fn output<T: ToOutput>(&mut self, object: T) {
        self.commander
            .write_result(format!("{}", object.to_output()));
    }
}

impl<C: Commander> Iterator for Runner<C> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }

        let input = self.commander.read_command()?;
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
