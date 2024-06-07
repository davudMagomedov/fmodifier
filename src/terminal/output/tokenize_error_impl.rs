use crate::terminal::tokenizer::TokenizeError;

use super::ToOutput;

impl<'a> ToOutput for TokenizeError<'a> {
    fn to_output(self) -> String {
        format!("Error: {}\n", self)
    }
}
