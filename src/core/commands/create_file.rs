use super::*;

fn create_file_info(file_name: &str, file_size: usize) -> String {
    format!(
        "File with name {name} and size {size} was created",
        name = file_name,
        size = file_size
    )
}

/// The `create_file` obviously creates new file with given name and size. If there's another
/// variable with the given name, the function just replace old value to file.
///
/// Output's format:
/// - Info: File with name <file_name> and size <file_size> was created.
pub fn create_file(core: &mut Core, file_name: String, file_size: usize) -> CoreResult<CoreOutput> {
    let file = File::New(NewFile::new(&file_name).map_err(|e| CoreError::from(e))?);

    core.variables.new_file(file_name.clone(), file);

    let mut output = CoreOutput::new();
    output.push_info(create_file_info(&file_name, file_size));

    Ok(output)
}
