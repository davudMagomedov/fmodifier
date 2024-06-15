use super::*;

use std::fmt::Display;

const BUFFER_VALUE: &str = "<buffer>";
const FILE_VALUE: &str = "<file>";

fn info<T: Display>(variable_name: &str, value: T) -> String {
    format!("{variable_name} = {value}")
}

// The `get_variable` function write to output the variable's value. If there's no such variable
// with given name, the function returns `Err`.
pub fn get_variable(core: &mut Core, variable_name: &str) -> CoreResult<CoreOutput> {
    let mut output = CoreOutput::new();

    if let Some(string_value) = core.variables.get_string(variable_name) {
        output.push_info(info(variable_name, string_value));
    } else if let Some(integer_value) = core.variables.get_integer(variable_name) {
        output.push_info(info(variable_name, integer_value));
    } else if let Some(_buffer) = core.variables.get_buffer(variable_name) {
        output.push_info(info(variable_name, BUFFER_VALUE));
    } else if let Some(_file) = core.variables.get_file(variable_name) {
        output.push_info(info(variable_name, FILE_VALUE));
    } else {
        return Err(CoreError::undefined_variable(variable_name.to_string()));
    }

    Ok(output)
}
