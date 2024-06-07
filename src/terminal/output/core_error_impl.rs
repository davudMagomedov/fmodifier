use super::ToOutput;

use crate::core::core_e::CoreError;

impl ToOutput for CoreError {
    fn to_output(self) -> String {
        format!("Error: {}\n", self)
    }
}
