use super::command::CoreCommand;
use super::token::Token;

use std::error::Error as ErrorTrait;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    UnknownCommandTemplate,
}

impl ParseError {
    pub fn unknown_command_template() -> Self {
        ParseError::UnknownCommandTemplate
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ParseError: {:?}", self)
    }
}

impl ErrorTrait for ParseError {}

/// The `parse_tokens` function takes sequence of tokens and makes on them core's command.
pub fn parse_tokens(tokens: &[Token]) -> ParseResult<CoreCommand> {
    let Some(Token::Word(command_name)) = tokens.get(0) else { return Err(ParseError::unknown_command_template()) };

    match command_name.as_str() {
        "make_buffer" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(buffer_size)) = tokens.get(2) else { return Err(ParseError::unknown_command_template()) };

            Ok(CoreCommand::MakeBuffer {
                buffer_name: buffer_name.clone(),
                buffer_size: *buffer_size,
            })
        }
        "fill_buffer" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(value)) = tokens.get(2) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(start)) = tokens.get(3) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(end)) = tokens.get(4) else { return Err(ParseError::unknown_command_template()) };

            if *value > 255 {
                return Err(ParseError::unknown_command_template());
            }

            Ok(CoreCommand::FillBuffer {
                buffer_name,
                value: *value as u8,
                start: *start,
                end: *end,
            })
        }
        "show_buffer" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(start)) = tokens.get(2) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(end)) = tokens.get(3) else { return Err(ParseError::unknown_command_template()) };

            Ok(CoreCommand::ShowBuffer {
                buffer_name,
                start: *start,
                end: *end,
            })
        }
        "buffer_info" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::unknown_command_template()) };

            Ok(CoreCommand::BufferInfo { buffer_name })
        }
        "buffer_set_byte" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(index)) = tokens.get(2) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(value)) = tokens.get(3) else { return Err(ParseError::unknown_command_template()) };

            if *value > 255 {
                return Err(ParseError::unknown_command_template());
            }

            Ok(CoreCommand::BufferSetByte {
                buffer_name,
                index: *index,
                value: *value as u8,
            })
        }
        "create_file" => {
            let Some(Token::Word(file_name)) = tokens.get(1) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(file_size)) = tokens.get(2) else { return Err(ParseError::unknown_command_template()) };

            Ok(CoreCommand::CreateFile {
                file_name: file_name.clone(),
                file_size: *file_size,
            })
        }
        "from_file_to_buffer" => {
            let Some(Token::Word(file_name)) = tokens.get(1) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::Word(buffer_name)) = tokens.get(2) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(bytes_count)) = tokens.get(3) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(file_start)) = tokens.get(4) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(buffer_start)) = tokens.get(5) else { return Err(ParseError::unknown_command_template()) };

            Ok(CoreCommand::FromFileToBuffer {
                file_name,
                buffer_name,
                bytes_count: *bytes_count,
                file_start: *file_start,
                buffer_start: *buffer_start,
            })
        }
        "from_buffer_to_file" => {
            let Some(Token::Word(buffer_name)) = tokens.get(1) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::Word(file_name)) = tokens.get(2) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(bytes_count)) = tokens.get(3) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(buffer_start)) = tokens.get(4) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(file_start)) = tokens.get(5) else { return Err(ParseError::unknown_command_template()) };

            Ok(CoreCommand::FromBufferToFile {
                buffer_name,
                file_name,
                bytes_count: *bytes_count,
                buffer_start: *buffer_start,
                file_start: *file_start,
            })
        }
        "open_file" => {
            let Some(Token::Word(file_name)) = tokens.get(1) else { return Err(ParseError::unknown_command_template()) };

            Ok(CoreCommand::OpenFile {
                file_name: file_name.clone(),
            })
        }
        "show_file" => {
            let Some(Token::Word(file_name)) = tokens.get(1) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(start)) = tokens.get(2) else { return Err(ParseError::unknown_command_template()) };
            let Some(Token::UInt(end)) = tokens.get(3) else { return Err(ParseError::unknown_command_template()) };

            Ok(CoreCommand::ShowFile {
                file_name,
                start: *start,
                end: *end,
            })
        }
        _ => Err(ParseError::unknown_command_template()),
    }
}
