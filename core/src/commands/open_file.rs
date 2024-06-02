use super::*;

fn open_file_info(file_name: &str) -> String {
    format!("The file {file_name} is opened.")
}

/// The `open_file` function opens file with given name and saves it to variables. Name of file in
/// variables table is the same as in the directory this file is keeped in.
pub fn open_file(core: &mut Core, file_name: String) -> CoreResult<CoreOutput> {
    let file_to_read = ReadFile::new(&file_name).map_err(|io_error| CoreError::from(io_error))?;
    let file = File::ToRead(file_to_read);

    let mut output = CoreOutput::new();
    output.push_info(open_file_info(&file_name));

    core.variables.new_file(file_name, file);

    Ok(output)
}
