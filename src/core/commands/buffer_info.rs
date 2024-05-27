use super::*;

fn format_name(buffer_name: &str) -> String {
    format!("Name: {}.", buffer_name)
}

fn format_size(len: usize) -> String {
    format!("Size: {} bytes.", len)
}

/// The `present_info` function writes information about given buffer to given output.
fn present_info(buffer: &Buffer, buffer_name: &str, output: &mut CoreOutput) {
    output.push_info(format_name(buffer_name));
    output.push_info(format_size(buffer.len()));
}

/// The `buffer_info` writes information about buffer with given name to output.
///
/// Output's format:
/// - Name: <buffer_name>.
/// - Size: <buffer_size>
pub fn buffer_info(core: &Core, buffer_name: &str) -> CoreResult<CoreOutput> {
    let buffer = core
        .variables
        .get_buffer(buffer_name)
        .ok_or_else(|| CoreError::undefined_variable(buffer_name.to_string()))?;

    let mut output = CoreOutput::new();
    present_info(buffer, buffer_name, &mut output);

    Ok(output)
}
