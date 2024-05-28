use super::*;

fn correct_index_of_buffer(buffer: &Buffer, index: usize) -> bool {
    buffer.len() > index
}

fn buffer_set_byte_info(buffer_name: &str, index: usize, value: u8) -> String {
    format!(
        "Index {index} in buffer with name {name} was set to {value}.",
        name = buffer_name,
        index = index,
        value = value,
    )
}

/// The `buffer_set_byte` function sets the value in given index of buffer with given name to given
/// value.
///
/// Output's format:
/// - Info: Index <index> in buffer with name <name> was set to <value>.
pub fn buffer_set_byte(
    core: &mut Core,
    buffer_name: &str,
    index: usize,
    value: u8,
) -> CoreResult<CoreOutput> {
    let buffer = core
        .variables
        .get_buffer_mut(&buffer_name)
        .ok_or_else(|| CoreError::undefined_variable(buffer_name.to_string()))?;

    if !correct_index_of_buffer(buffer, index) {
        return Err(CoreError::incorrect_index(index, buffer.len()));
    }

    // There's guarantees that `index` isn't wrong, so there will be no panic.
    buffer.write_byte(value, index);

    let mut output = CoreOutput::new();
    output.push_info(buffer_set_byte_info(buffer_name, index, value));

    Ok(output)
}
