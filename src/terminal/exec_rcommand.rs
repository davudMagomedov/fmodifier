use crate::terminal::commander::Commander;
use crate::terminal::output::ToOutput;
use crate::terminal::rcommand::RunCommand;
use crate::terminal::runner::Runner;

use std::error::Error as ErrorTrait;
use std::fmt::{Display, Result as FmtResult};

#[derive(Debug)]
pub enum ExecuteRunCommandError<'a> {
    CommandNotFound { command_name: &'a str },
}

impl<'a> Display for ExecuteRunCommandError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "CommandExecution: ")?;
        match self {
            Self::CommandNotFound { command_name } => {
                write!(f, "CommandNotFound: {}", command_name)
            }
        }
    }
}

impl<'a> ErrorTrait for ExecuteRunCommandError<'a> {}

impl<'a> ToOutput for ExecuteRunCommandError<'a> {
    fn to_output(self) -> String {
        format!("Error: {}", self)
    }
}

pub struct RCOutput(String);

impl ToOutput for RCOutput {
    fn to_output(self) -> String {
        self.0
    }
}

/// The `execute_run_command`, surprisingly, executes `RunCommand`. If the command succesfully
/// executed, the function returns `Ok(obj)` where `obj`'s type implements `ToOtput` trait.
/// Otherwise, the function returns error that implements `ToOutput` trait.
pub fn execute_run_command<'a, C: Commander>(
    runner: &mut Runner<C>,
    rcommand: &'a RunCommand,
) -> Result<RCOutput, ExecuteRunCommandError<'a>> {
    match rcommand {
        RunCommand::Exit => {
            runner.finish();
            Ok(RCOutput("".to_string()))
        }
        RunCommand::Help => {
            unimplemented!();
        }
        RunCommand::HelpAbout { .. } => {
            unimplemented!();
        }
    }
}
