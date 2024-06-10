use super::*;

fn info(file_name: &str, new_buffer_name: &str) -> String {
    format!(
        "File with name {} were turned into buffer with name {}",
        file_name, new_buffer_name
    )
}

/// The `turn_file_to_buffer` function turns file with given name to buffer with given name. The
/// file keeps being in variables.
pub fn turn_file_to_buffer(
    core: &mut Core,
    file_name: &str,
    new_buffer_name: String,
) -> CoreResult<CoreOutput> {
    let file = core
        .variables
        .get_file_mut(file_name)
        .ok_or_else(|| CoreError::undefined_variable(file_name.to_string()))?;

    let file_size = match file {
        File::New(f) => f.len()?,
        File::ToRead(f) => f.len()?,
    };
    let file_bytes = match file {
        File::New(f) => f.read_bytes(0, f.len()?)?.unwrap(),
        File::ToRead(f) => f.read_bytes(0, f.len()?)?.unwrap(),
    };

    let mut new_buffer = Buffer::new(file_size);

    new_buffer.write_bytes(&file_bytes, 0);

    let mut output = CoreOutput::new();
    output.push_info(info(file_name, &new_buffer_name));

    core.variables.new_buffer(new_buffer_name, new_buffer);

    Ok(output)
}
