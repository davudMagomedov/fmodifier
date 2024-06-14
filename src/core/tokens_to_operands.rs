use super::operand::Operand;
use super::Core;
use super::Token;

use std::error::Error;
use std::fmt::{Display, Result as FmtResult};

#[derive(Debug, Clone)]
pub enum TokensTranslationError {
    UnknownVariable { varname: String },
}

impl Display for TokensTranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "TokensTranslationError: ")?;
        match self {
            Self::UnknownVariable { varname } => {
                write!(f, "UnknownVariable: {}", varname)
            }
        }
    }
}

impl Error for TokensTranslationError {}

/// The `tokens_to_operands` variables takes tokens and converts them to operands according to
/// given core.
pub fn tokens_to_operands(
    core: &mut Core,
    tokens: &[Token],
) -> Result<Vec<Operand>, TokensTranslationError> {
    Ok(tokens
        .into_iter()
        .map(|token| match token {
            Token::UInt(integer) => Operand::UInt(*integer),
            Token::Word(word) => Operand::Name(word.clone()),
        })
        .collect())
}
