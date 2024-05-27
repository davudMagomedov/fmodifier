use super::*;

const COLUMNS_COUNT: usize = 8;

/// The `make_table` creates table with appropriate names for columns and rows. It's take two
/// arguments: bytes and index those bytes start with.
fn make_table(bytes: &[u8], start: usize) -> OtherInfo {
    let start_index = start % COLUMNS_COUNT;
    let rows_count = ((start_index + bytes.len() - 1) / COLUMNS_COUNT) * COLUMNS_COUNT;
    let row_start_in = (start / COLUMNS_COUNT) * COLUMNS_COUNT;

    let rows_names = (row_start_in..row_start_in + rows_count)
        .map(|i| ((i * COLUMNS_COUNT).to_string()))
        .collect::<Vec<_>>();
    let columns_names = (0..COLUMNS_COUNT)
        .map(|i| ShortString::new(i.to_string()).unwrap())
        .collect::<Vec<_>>();

    let mut table = Table::new(rows_names, columns_names);

    (start_index..start_index + bytes.len()).for_each(|index| {
        table.write(
            ShortString::new(bytes[index].to_string()).unwrap(),
            index / COLUMNS_COUNT,
            index % COLUMNS_COUNT,
        )
    });

    OtherInfo::BigTable { table }
}

fn show_buffer_wrong_index(buffer_name: &str, index: usize) -> Warning {
    format!(
        "Buffer '{name}' doesn't have index {index}.",
        name = buffer_name,
        index = index
    )
}

/// The `show_buffer` writes content of the buffer with given name to output.
///
/// Output's format: if the indexes are wrong in any way:
/// - Warning: Buffer '<buffer_name>' doesn't have index <some_wrong_index>.
///
/// Output's format: if the indexes are correct:
/// - Other info: Table of elements.
pub fn show_buffer(
    core: &Core,
    buffer_name: &str,
    start: usize,
    end: usize,
) -> CoreResult<CoreOutput> {
    // - Get buffer with the name.
    // - If there's no buffer with the name, return Err.
    // - Get bytes from start to end indexes.
    // - If it can't to get the bytes, return output with warning.
    // - Make table.
    // - Make output and write to it the table.
    // - Ok(output)

    let buffer = core
        .variables
        .get_buffer(buffer_name)
        .ok_or_else(|| CoreError::undefined_variable(buffer_name.to_string()))?;
    let bytes = match buffer.read_bytes(start, end) {
        Some(bytes) => bytes,
        None => {
            let mut output = CoreOutput::new();
            output.push_warning(show_buffer_wrong_index(buffer_name, start));

            return Ok(output);
        }
    };

    let table = make_table(bytes, start);

    let mut output = CoreOutput::new();
    output.push_other_info(table);

    Ok(output)
}
