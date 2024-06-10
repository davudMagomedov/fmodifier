use super::*;

fn info(left_buffer_name: &str, right_buffer_name: &str, new_buffer_name: &str) -> String {
    format!("Buffers {left_buffer_name} and {right_buffer_name} were merged into one buffer {new_buffer_name}.")
}

/// The `merge_buffers` function creates a buffer whose size is equal to sum of the another two,
/// and writes bytes from the first buffer to first part of new buffer, and bytes from second
/// buffer to the second part.
pub fn merge_buffers(
    core: &mut Core,
    left_buffer_name: &str,
    right_buffer_name: &str,
    new_buffer_name: String,
) -> CoreResult<CoreOutput> {
    let left_buffer = core
        .variables
        .get_buffer(left_buffer_name)
        .ok_or_else(|| CoreError::undefined_variable(left_buffer_name.to_string()))?;
    let right_buffer = core
        .variables
        .get_buffer(right_buffer_name)
        .ok_or_else(|| CoreError::undefined_variable(right_buffer_name.to_string()))?;

    let mut new_buffer = Buffer::new(left_buffer.len() + right_buffer.len());

    new_buffer.write_bytes(left_buffer.read_bytes(0, left_buffer.len()).unwrap(), 0);
    new_buffer.write_bytes(
        right_buffer.read_bytes(0, right_buffer.len()).unwrap(),
        left_buffer.len(),
    );

    let mut output = CoreOutput::new();
    output.push_info(info(left_buffer_name, right_buffer_name, &new_buffer_name));

    core.variables.new_buffer(new_buffer_name, new_buffer);

    Ok(output)
}
