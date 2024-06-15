use super::*;

use std::fmt::Display;

fn info<T: Display>(variable_name: &str, value: T) -> String {
    format!(
        "Value in {} variable was set to '{}' value.",
        variable_name, value
    )
}

/// The `set_variable_integer` function places given integer value into the variable with given
/// name.
pub fn set_variable_integer(
    core: &mut Core,
    variable_name: String,
    value: usize,
) -> CoreResult<CoreOutput> {
    let mut output = CoreOutput::new();
    output.push_info(info(&variable_name, value));

    core.variables.new_integer(variable_name, value);

    Ok(output)
}

/// The `set_variable_string` function places given string value into the variable with given name.
pub fn set_variable_string(
    core: &mut Core,
    variable_name: String,
    value: String,
) -> CoreResult<CoreOutput> {
    let mut output = CoreOutput::new();
    output.push_info(info(&variable_name, &value));

    core.variables.new_string(variable_name, value);

    Ok(output)
}
