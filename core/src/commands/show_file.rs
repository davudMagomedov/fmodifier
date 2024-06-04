use super::*;

/// The `show_file` function reads bytes from the file with given name and writes to output the
/// table with those bytes.
pub fn show_file(
    core: &mut Core,
    file_name: &str,
    start: usize,
    end: usize,
) -> CoreResult<CoreOutput> {
    let file = core
        .variables
        .get_file_mut(file_name)
        .ok_or_else(|| CoreError::undefined_variable(file_name.to_string()))?;

    let bytes = match file {
        File::ToRead(file) => {
            let file_size = file.len().map_err(|io_err| CoreError::from(io_err))?;
            file.read_bytes(start, end)
                .map_err(|io_err| CoreError::from(io_err))?
                .ok_or_else(|| CoreError::incorrect_index(start, file_size))?
        }
        File::New(file) => {
            let file_size = file.len().map_err(|io_err| CoreError::from(io_err))?;
            file.read_bytes(start, end)
                .map_err(|io_err| CoreError::from(io_err))?
                .ok_or_else(|| CoreError::incorrect_index(start, file_size))?
        }
    };

    let table = make_table(&bytes, start);

    let mut output = CoreOutput::new();
    output.push_other_info(table);

    Ok(output)
}
