use super::*;

fn make_buffer_info(buffer_name: &str, buffer_size: usize) -> InfoLine {
    format!(
        "Buffer with name {name} and size {size} is created.",
        name = buffer_name,
        size = buffer_size
    )
}

/// The `make_buffer` function creates new buffer with given size and writes it to variable with
/// given name.
///
/// Output's format:
/// - Info: Buffer with name <buffer_name> and size <size> is created.
pub fn make_buffer(
    core: &mut Core,
    buffer_name: String,
    buffer_size: usize,
) -> CoreResult<CoreOutput> {
    let buffer = Buffer::new(buffer_size);

    let mut output = CoreOutput::new();
    output.push_info(make_buffer_info(&buffer_name, buffer_size));

    core.variables.new_buffer(buffer_name, buffer);

    Ok(output)
}
