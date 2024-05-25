use super::*;

fn from_file_to_buffer_info(
    file_name: &str,
    buffer_name: &str,
    written_bytes_count: usize,
) -> String {
    format!(
        "Bytes of file {file_name} in the amount of {count} pieces were written to buffer {buffer_name}.",
        file_name = file_name,
        count = written_bytes_count,
        buffer_name = buffer_name
    )
}

/// The `from_file_to_buffer` function writes bytes in the given amount from file with given name
/// to buffer with given name. Reading from file, the function starts from given file's start
/// index. Writing to buffer, the function starts from given buffer's index.
///
/// Output's format:
/// - Info: Bytes of file <file_name> in the amount of <written_bytes_count> pieces were written to
/// buffer <buffer_name>.
pub fn from_file_to_buffer(
    core: &mut Core,
    file_name: String,
    buffer_name: String,
    bytes_count: usize,
    file_start: usize,
    buffer_start: usize,
) -> CoreResult<CoreOutput> {
    let file = core
        .variables
        .get_file_mut(&file_name)
        .ok_or_else(|| CoreError::undefined_variable(file_name.clone()))?;
    let file_len = match file {
        File::New(f) => f.len().map_err(|e| CoreError::from(e))?,
        File::ToRead(f) => f.len().map_err(|e| CoreError::from(e))?,
    };

    let bytes = match file {
        File::New(f) => f.read_bytes(file_start, file_start + bytes_count),
        File::ToRead(f) => f.read_bytes(file_start, file_start + bytes_count),
    }
    .map_err(|e| CoreError::from(e))?
    .ok_or_else(|| CoreError::incorrect_index(file_start, file_len))?; // Incorrect index

    let buffer = core
        .variables
        .get_buffer_mut(&buffer_name)
        .ok_or_else(|| CoreError::undefined_variable(buffer_name.clone()))?;

    let written_bytes_count = buffer.write_bytes(&bytes, buffer_start);

    let mut output = CoreOutput::new();
    output.push_info(from_file_to_buffer_info(
        &file_name,
        &buffer_name,
        written_bytes_count,
    ));

    Ok(output)
}
