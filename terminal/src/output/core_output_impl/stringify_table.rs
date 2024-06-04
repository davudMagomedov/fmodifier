use super::string_rectangle::StringRectangle;

use core::output::*;

use std::collections::VecDeque;

fn rectangle_elements_vseparator(line_count: usize) -> StringRectangle {
    StringRectangle::new_with_lines(&vec![" "; line_count])
}

fn rectangle_row_element_separator(line_count: usize) -> StringRectangle {
    StringRectangle::new_with_lines(&vec![" │ "; line_count])
}

fn table_data_to_rectangle(table: &Table) -> StringRectangle {
    unimplemented!();
}

/// The `stringify_table` function writes to given string stringified given table. There's no extra
/// characters in the end and in the start.
pub fn stringify_table(table: &Table, write_to: &mut String) {
    unimplemented!();
}
