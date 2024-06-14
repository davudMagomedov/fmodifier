use crate::core::tokens_to_operands::TokensTranslationError;

use super::ToOutput;

impl ToOutput for TokensTranslationError {
    fn to_output(self) -> String {
        format!("Error: {}\n", self)
    }
}
