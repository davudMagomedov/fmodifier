use super::*;

fn from_buffer_to_file_info(
    buffer_name: &str,
    file_name: &str,
    written_bytes_count: usize,
) -> String {
    format!(
        "Bytes of buffer {buffer_name} in the amount of {count} pieces were written to file {file_name}.",
        buffer_name = buffer_name,
        file_name = file_name,
        count = written_bytes_count,
    )
}

/// The `from_buffer_to_file` function writes bytes in the given amount from buffer with given name
/// to file with given name. Reading from buffer, the function starts from given buffer's start
/// index. Writing to file, the function starts from given file's index.
///
/// Output's format:
/// - Info: Bytes of buffer <buffer_name> in the amount of <written_bytes_count> pieces were written to
/// file <file_name>.
pub fn from_buffer_to_file(
    core: &mut Core,
    buffer_name: &str,
    file_name: &str,
    bytes_count: usize,
    buffer_start: usize,
    file_start: usize,
) -> CoreResult<CoreOutput> {
    let buffer = core
        .variables
        .get_buffer_mut(buffer_name)
        .ok_or_else(|| CoreError::undefined_variable(buffer_name.to_string()))?;
    let bytes = Box::from(
        buffer
            .read_bytes(buffer_start, buffer_start + bytes_count)
            .ok_or_else(|| CoreError::incorrect_index(buffer_start, buffer.len()))?,
    );

    let file = core
        .variables
        .get_file_mut(file_name)
        .ok_or_else(|| CoreError::undefined_variable(file_name.to_string()))?;

    let written_bytes_count = match file {
        File::New(f) => f.write_bytes(&bytes, file_start),
        File::ToRead(_) => return Err(CoreError::writing_to_read_only_file(file_name.to_string())),
    }
    .map_err(|e| CoreError::from(e))?;

    let mut output = CoreOutput::new();
    output.push_info(from_buffer_to_file_info(
        buffer_name,
        file_name,
        written_bytes_count,
    ));

    Ok(output)
}
