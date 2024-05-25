use std::error::Error as ErrorTrait;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

#[derive(Debug)]
enum CoreErrorEnum {
    UndefinedVariable { variable_name: String },
    IncorrectIndex { index: usize, top: usize },
    WritingToReadOnlyFile { file_name: String },
    Io { e: IoError },
}

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug)]
pub struct CoreError {
    enumer: CoreErrorEnum,
}

impl CoreError {
    pub fn pass_new() -> Self {
        unimplemented!()
    }
}

impl CoreError {
    pub fn undefined_variable(variable_name: String) -> Self {
        CoreError {
            enumer: CoreErrorEnum::UndefinedVariable { variable_name },
        }
    }

    pub fn incorrect_index(index: usize, top: usize) -> Self {
        CoreError {
            enumer: CoreErrorEnum::IncorrectIndex { index, top },
        }
    }

    pub fn writing_to_read_only_file(file_name: String) -> Self {
        CoreError {
            enumer: CoreErrorEnum::WritingToReadOnlyFile { file_name },
        }
    }
}

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "TODO ERROR HANDLING")
    }
}

impl ErrorTrait for CoreError {}

impl From<IoError> for CoreError {
    fn from(e: IoError) -> Self {
        CoreError {
            enumer: CoreErrorEnum::Io { e },
        }
    }
}
