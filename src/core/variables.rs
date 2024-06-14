use super::buffer::Buffer;
use super::file::File;

use std::collections::HashMap;

enum VariableValue {
    File(File),
    Buffer(Buffer),
    Integer(usize),
    String(String),
}

/// The `Variables` structure stores buffer and files.
///
/// There's following functions:
/// - `get_buffer` returns buffer with given name.
/// - `get_buffer_mut` returns mutable buffer with given name.
/// - `get_file` returns file with given name.
/// - `get_file_mut` returns mutable file with given name.
/// - `new_buffer` binds given buffer to given name.
/// - `new_file` binds given file to given name.
///
/// There can't be buffer and file with the same names at once.
pub struct Variables {
    vars: HashMap<String, VariableValue>,
}

impl Variables {
    /// The `new` function creates empty variable table.
    pub fn new() -> Self {
        Variables {
            vars: HashMap::new(),
        }
    }

    pub fn get_buffer(&self, buffer_name: &str) -> Option<&Buffer> {
        self.vars.get(buffer_name).map(|var| match var {
            VariableValue::Buffer(b) => Some(b),
            _ => None,
        })?
    }

    pub fn get_buffer_mut(&mut self, buffer_name: &str) -> Option<&mut Buffer> {
        self.vars.get_mut(buffer_name).map(|var| match var {
            VariableValue::Buffer(b) => Some(b),
            _ => None,
        })?
    }

    pub fn get_file(&self, file_name: &str) -> Option<&File> {
        self.vars.get(file_name).map(|var| match var {
            VariableValue::File(f) => Some(f),
            _ => None,
        })?
    }

    pub fn get_file_mut(&mut self, file_name: &str) -> Option<&mut File> {
        self.vars.get_mut(file_name).map(|var| match var {
            VariableValue::File(f) => Some(f),
            _ => None,
        })?
    }

    pub fn get_integer(&mut self, var_name: &str) -> Option<usize> {
        match self.vars.get(var_name)? {
            VariableValue::Integer(integer) => Some(*integer),
            _ => None,
        }
    }

    pub fn get_string(&mut self, var_name: &str) -> Option<&String> {
        match self.vars.get(var_name)? {
            VariableValue::String(string) => Some(string),
            _ => None,
        }
    }

    /// The `new_buffer` function replaces value in `buffer_name` to given buffer. Old value
    /// deletes.
    pub fn new_buffer(&mut self, buffer_name: String, buffer: Buffer) {
        self.vars.insert(buffer_name, VariableValue::Buffer(buffer));
    }

    /// The `new_file` function replaces value in `file_name` to given file. Old value deletes.
    pub fn new_file(&mut self, file_name: String, file: File) {
        self.vars.insert(file_name, VariableValue::File(file));
    }
}
