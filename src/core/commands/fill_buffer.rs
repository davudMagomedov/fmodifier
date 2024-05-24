use super::*;

fn fill_buffer_info(buffer_name: &str, value: u8, written_bytes: usize) -> InfoLine {
    format!(
        "Bytes in the amount of {count} pieces were filled by {val} value in buffer '{name}'.",
        count = written_bytes,
        name = buffer_name,
        val = value
    )
}

/// The `fill_buffer` function fills bytes in buffer with given name from given start to given end
/// by given value.
///
/// If given start index is wrong in any way, the function just will write no byte to buffer.
///
/// Output's format:
/// - Info: Bytes in the amount of <written_bytes> pieces were filled by <value> value in buffer
/// '<buffer_name>'.
pub fn fill_buffer(
    core: &mut Core,
    buffer_name: String,
    value: u8,
    start: usize,
    end: usize,
) -> CoreResult<CoreOutput> {
    // - Get buffer. If there's no buffer with the name, return Err.
    // - File bytes in buffer and get count of written bytes. If the start index is wrong, make
    // written bytes' count to 0.
    // - Create output and write appropriate info to it.
    // - Return output.

    let buffer = core
        .variables
        .get_buffer_mut(&buffer_name)
        .ok_or_else(|| CoreError::pass_new())?;

    let written_bytes = buffer.fill_bytes(value, start, end).unwrap_or(0);

    let mut output = CoreOutput::new();
    output.push_info(fill_buffer_info(&buffer_name, value, written_bytes));

    Ok(output)
}
