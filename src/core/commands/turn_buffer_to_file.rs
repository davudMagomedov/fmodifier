use super::*;

fn info(buffer_name: &str, new_file_name: &str) -> String {
    format!(
        "File with name {} was created from buffer {}",
        new_file_name, buffer_name
    )
}

/// The `turn_buffer_to_file` function creates file with given name and data the same as in buffer
/// with given name.
pub fn turn_buffer_to_file(
    core: &mut Core,
    buffer_name: &str,
    new_file_name: String,
) -> CoreResult<CoreOutput> {
    let buffer = core
        .variables
        .get_buffer(buffer_name)
        .ok_or_else(|| CoreError::undefined_variable(buffer_name.to_string()))?;
    let mut new_file = NewFile::new(&new_file_name, buffer.len())?;

    new_file.write_bytes(buffer.read_bytes(0, buffer.len()).unwrap(), 0)?;

    let mut output = CoreOutput::new();
    output.push_info(info(buffer_name, &new_file_name));

    core.variables.new_file(new_file_name, File::New(new_file));

    Ok(output)
}
