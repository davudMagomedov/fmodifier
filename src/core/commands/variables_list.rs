use crate::core::variables::VariableValue;

use super::*;

use std::fmt::Display;

const BUFFER_VALUE: &str = "<buffer>";
const FILE_VALUE: &str = "<file>";

fn format_pair<T: Display>(varname: &str, value: T) -> String {
    format!("{varname} = {value}")
}

/// Teh `variables_list` function shows all variables keeped in core.
pub fn variables_list(core: &mut Core) -> CoreResult<CoreOutput> {
    let mut output = CoreOutput::new();

    core.variables
        .all()
        .for_each(|(varname, var_value)| match var_value {
            VariableValue::File(_) => output.push_info(format_pair(varname, FILE_VALUE)),
            VariableValue::Buffer(_) => output.push_info(format_pair(varname, BUFFER_VALUE)),
            VariableValue::Integer(integer) => output.push_info(format_pair(varname, integer)),
            VariableValue::String(string) => output.push_info(format_pair(varname, string)),
        });

    Ok(output)
}
