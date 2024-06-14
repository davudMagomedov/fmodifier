use super::commander::Commander;
use super::exec_rcommand::execute_run_command;
use super::output::ToOutput;
use super::rcommand::parse_run_command;
use super::tokenizer::tokenize;

use crate::core::parse_operands;
use crate::core::tokens_to_operands;
use crate::core::Core;
use crate::core::Token;

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

    pub fn print(&mut self, msg: String) {
        self.commander.write_result(msg);
    }

    pub fn output<T: ToOutput>(&mut self, object: T) {
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

        let operands = match tokens_to_operands(&mut self.core, &tokens) {
            Ok(operands) => operands,
            Err(e) => {
                self.output(e);
                return Some(());
            }
        };

        if let Some(run_command) = parse_run_command(&operands) {
            if let Err(e) = execute_run_command(self, &run_command) {
                self.output(e);
            };

            return Some(());
        }

        let command = match parse_operands(&operands) {
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
