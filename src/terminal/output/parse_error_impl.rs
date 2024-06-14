use super::ToOutput;

use crate::core::parse_operands::ParseError;

impl ToOutput for ParseError {
    fn to_output(self) -> String {
        format!("Error: {}\n", self)
    }
}
