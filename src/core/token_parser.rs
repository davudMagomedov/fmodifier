use super::command::CoreCommand;
use super::token::Token;

use std::error::Error as ErrorTrait;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError {}

impl ParseError {
    pub fn pass_new() -> Self {
        ParseError {}
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ParseError: ")
    }
}

impl ErrorTrait for ParseError {}

/// The `parse_tokens` function takes sequence of tokens and makes on them core's command.
pub fn parse_tokens(tokens: &[Token]) -> ParseResult<CoreCommand> {
    let Some(Token::Word(command_name)) = tokens.get(0) else { return Err(ParseError::pass_new()) };

    match command_name.as_str() {
        "make_buffer" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(buffer_size)) = tokens.get(2) else { return Err(ParseError::pass_new()) };

            Ok(CoreCommand::MakeBuffer {
                buffer_name: buffer_name.clone(),
                buffer_size: *buffer_size,
            })
        }
        "fill_buffer" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(value)) = tokens.get(2) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(start)) = tokens.get(3) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(end)) = tokens.get(4) else { return Err(ParseError::pass_new()) };

            if *value > 255 {
                return Err(ParseError::pass_new());
            }

            Ok(CoreCommand::FillBuffer {
                buffer_name,
                value: *value as u8,
                start: *start,
                end: *end,
            })
        }
        "show_buffer" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(start)) = tokens.get(2) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(end)) = tokens.get(3) else { return Err(ParseError::pass_new()) };

            Ok(CoreCommand::ShowBuffer {
                buffer_name,
                start: *start,
                end: *end,
            })
        }
        "buffer_info" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::pass_new()) };

            Ok(CoreCommand::BufferInfo { buffer_name })
        }
        "buffer_set_byte" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(index)) = tokens.get(2) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(value)) = tokens.get(3) else { return Err(ParseError::pass_new()) };

            if *value > 255 {
                return Err(ParseError::pass_new());
            }

            Ok(CoreCommand::BufferSetByte {
                buffer_name,
                index: *index,
                value: *value as u8,
            })
        }
        "create_file" => {
            let Some(Token::Word(file_name)) = tokens.get(1) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(file_size)) = tokens.get(2) else { return Err(ParseError::pass_new()) };

            Ok(CoreCommand::CreateFile {
                file_name: file_name.clone(),
                file_size: *file_size,
            })
        }
        "from_file_to_buffer" => {
            let Some(Token::Word(file_name)) = tokens.get(1) else { return Err(ParseError::pass_new()) };
            let Some(Token::Word(buffer_name)) = tokens.get(2) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(bytes_count)) = tokens.get(3) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(file_start)) = tokens.get(4) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(buffer_start)) = tokens.get(5) else { return Err(ParseError::pass_new()) };

            Ok(CoreCommand::FromFileToBuffer {
                file_name,
                buffer_name,
                bytes_count: *bytes_count,
                file_start: *file_start,
                buffer_start: *buffer_start,
            })
        }
        "from_buffer_to_file" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::pass_new()) };
            let Some(Token::Word(file_name)) = tokens.get(2) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(bytes_count)) = tokens.get(3) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(buffer_start)) = tokens.get(4) else { return Err(ParseError::pass_new()) };
            let Some(Token::UInt(file_start)) = tokens.get(5) else { return Err(ParseError::pass_new()) };

            Ok(CoreCommand::FromBufferToFile {
                buffer_name,
                file_name,
                bytes_count: *bytes_count,
                buffer_start: *buffer_start,
                file_start: *file_start,
            })
        }
        _ => Err(ParseError::pass_new()),
    }
}
