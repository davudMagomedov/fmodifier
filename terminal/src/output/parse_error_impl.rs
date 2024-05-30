use super::ToOutput;

use core::token_parser::ParseError;

impl ToOutput for ParseError {
    fn to_output(self) -> String {
        format!("Error: {}", self)
    }
}
