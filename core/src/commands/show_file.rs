use super::*;

const COLUMNS_COUNT: usize = 16;

/// The `make_table` creates table with appropriate names for columns and rows. It's take two
/// arguments: bytes and index those bytes start with.
fn make_table(bytes: &[u8], start: usize) -> OtherInfo {
    let column_names: Vec<String> = (0..COLUMNS_COUNT).map(|c| c.to_string()).collect();
    let row_names = (start / COLUMNS_COUNT..(start + bytes.len()) / COLUMNS_COUNT + 1)
        .map(|row_index| (row_index * COLUMNS_COUNT).to_string())
        .collect();

    let mut table = Table::new(row_names, column_names);

    let offset = start % COLUMNS_COUNT;
    for (byte_index, &byte) in bytes.into_iter().enumerate() {
        table.write(
            // byte.to_string(),
            format!("{:02x}", byte),
            (offset + byte_index) / COLUMNS_COUNT,
            (offset + byte_index) % COLUMNS_COUNT,
        )
    }

    OtherInfo::BigTable { table }
}

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
