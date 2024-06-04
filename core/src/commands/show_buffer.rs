use super::*;

fn show_buffer_wrong_index(buffer_name: &str, index: usize) -> Warning {
    format!(
        "Buffer '{name}' doesn't have index {index}.",
        name = buffer_name,
        index = index
    )
}

/// The `show_buffer` writes content of the buffer with given name to output.
///
/// Output's format: if the indexes are wrong in any way:
/// - Warning: Buffer '<buffer_name>' doesn't have index <some_wrong_index>.
///
/// Output's format: if the indexes are correct:
/// - Other info: Table of elements.
pub fn show_buffer(
    core: &Core,
    buffer_name: &str,
    start: usize,
    end: usize,
) -> CoreResult<CoreOutput> {
    // - Get buffer with the name.
    // - If there's no buffer with the name, return Err.
    // - Get bytes from start to end indexes.
    // - If it can't to get the bytes, return output with warning.
    // - Make table.
    // - Make output and write to it the table.
    // - Ok(output)

    let buffer = core
        .variables
        .get_buffer(buffer_name)
        .ok_or_else(|| CoreError::undefined_variable(buffer_name.to_string()))?;
    let bytes = match buffer.read_bytes(start, end) {
        Some(bytes) => bytes,
        None => {
            let mut output = CoreOutput::new();
            output.push_warning(show_buffer_wrong_index(buffer_name, start));

            return Ok(output);
        }
    };

    let table = make_table(bytes, start);

    let mut output = CoreOutput::new();
    output.push_other_info(table);

    Ok(output)
}
