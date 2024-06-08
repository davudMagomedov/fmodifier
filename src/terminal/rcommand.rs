use crate::core::Token;

const EXIT_WORD: &str = "exit";
const HELP_WORD: &str = "help";

pub enum RunCommand {
    Exit,
    Help,
    HelpAbout { about: String },
}

/// The `parse_run_command` function takes a sequence of tokens and tries parse them into
/// `RunCommand`. If there's no way to make given tokens into `RunCommand`, the function is gonna
/// return `None`.
pub fn parse_run_command(tokens: &[Token]) -> Option<RunCommand> {
    match tokens.get(0)? {
        Token::Word(exit) if exit == EXIT_WORD => Some(RunCommand::Exit),
        Token::Word(help) if help == HELP_WORD => match tokens.get(1) {
            Some(Token::Word(about)) => Some(RunCommand::HelpAbout {
                about: about.clone(),
            }),
            None => Some(RunCommand::Help),
            _ => None,
        },
        _ => None,
    }
}
