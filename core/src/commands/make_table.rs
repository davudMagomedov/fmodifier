use super::*;

const COLUMNS_COUNT: usize = 16;

/// The `make_table` creates table with appropriate names for columns and rows. It's take two
/// arguments: bytes and index those bytes start with.
pub fn make_table(bytes: &[u8], start: usize) -> OtherInfo {
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
