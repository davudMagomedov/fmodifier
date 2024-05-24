use std::error::Error as ErrorTrait;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, Clone)]
pub struct CoreError {}

impl CoreError {
    pub fn pass_new() -> Self {
        CoreError {}
    }
}

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "TODO ERROR HANDLING")
    }
}

impl ErrorTrait for CoreError {}

impl From<IoError> for CoreError {
    fn from(value: IoError) -> Self {
        unimplemented!();
    }
}
