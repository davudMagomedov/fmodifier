use super::*;

macro_rules! TTTT {
    () => {
        .ok_or_else(|| CoreError::undefined_variable(buffer_name.to_string()))?
    };
}

fn info(buffer_name: &str, new_buffer_name: &str, start: usize, end: usize) -> String {
    format!("Slice {buffer_name}[{start}..{end}] is pulled out and named '{new_buffer_name}'")
}

/// The `pull_out_slice` copies bytes from given buffer starting and ending by given indexes to new
/// buffer.
///
/// The function creates new buffer which size exactly equals to `end - start`.
pub fn pull_out_slice(
    core: &mut Core,
    buffer_name: &str,
    new_buffer_name: String,
    start: usize,
    end: usize,
) -> CoreResult<CoreOutput> {
    let buffer = core
        .variables
        .get_buffer(buffer_name)
        .ok_or_else(|| CoreError::undefined_variable(buffer_name.to_string()))?;

    let slice = buffer
        .read_bytes(start, end)
        .ok_or_else(|| CoreError::undefined_variable(buffer_name.to_string()))?;

    if slice.len() != end - start {
        return Err(CoreError::incorrect_index(end, buffer.len()));
    }

    let mut new_buffer = Buffer::new(end - start);
    new_buffer.write_bytes(slice, 0);

    let mut output = CoreOutput::new();
    output.push_info(info(buffer_name, &new_buffer_name, start, end));

    core.variables.new_buffer(new_buffer_name, new_buffer);

    Ok(output)
}
