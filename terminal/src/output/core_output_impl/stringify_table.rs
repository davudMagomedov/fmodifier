use super::string_rectangle::StringRectangle;

use core::output::*;

/// The `rectangle_elements_vseparator` function is string rectangle that need to be inserted
/// between two columns of elements of table.
fn rectangle_elements_vseparator(line_count: usize) -> StringRectangle {
    StringRectangle::new_with_lines(&vec![" "; line_count])
}

/// The `rectangle_row_element_separator` is string rectangle that need to be inserted between
/// column of names of table and first column of elements of table.
fn rectangle_row_element_separator(line_count: usize) -> StringRectangle {
    StringRectangle::new_with_lines(&vec![" │ "; line_count])
}

/// The `table_data_to_rectangle` function turns elements of given table into matrix as string
/// rectangle.
fn table_data_to_rectangle(table: &Table) -> StringRectangle {
    let mut matrix_rectangle = StringRectangle::new();

    for column_index in 0..table.column_count() {
        let mut column_values: Vec<&str> = Vec::new();

        for row_index in 0..table.row_count() {
            column_values.push(table.get(row_index, column_index).unwrap());
        }

        matrix_rectangle = matrix_rectangle
            .place_right_top(StringRectangle::new_with_lines(&column_values))
            .place_right_top(rectangle_elements_vseparator(table.row_count()));
    }

    matrix_rectangle
}

/// The `stringify_table` function writes to given string stringified given table. There's no extra
/// characters in the end and in the start.
pub fn stringify_table(table: &Table, write_to: &mut String) {
    let rows_rectangle = StringRectangle::new_with_lines(
        &table
            .row_names()
            .into_iter()
            .map(|string| string.as_str())
            .collect::<Vec<_>>(),
    );
    let data_matrix = table_data_to_rectangle(table);

    let full_table_rectangle = rows_rectangle
        .place_right_top(rectangle_row_element_separator(table.row_count()))
        .place_right_top(data_matrix);

    write_to.push_str(full_table_rectangle.to_string().as_str());
}
