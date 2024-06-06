use super::string_rectangle::StringRectangle;

use core::output::*;

const NEW_LINE: char = '\n';

fn make_column(lines: &[String]) -> StringRectangle {
    if lines.is_empty() {
        return StringRectangle::new();
    }

    let content = lines[1..]
        .into_iter()
        .fold(lines[0].clone(), |acc, string| {
            format!("{acc}{NEW_LINE}{string}")
        });

    StringRectangle::from(content.as_str())
}

/// The `rectangle_elements_vseparator` function is string rectangle that need to be inserted
/// between two columns of elements of table.
fn rectangle_elements_vseparator(line_count: usize) -> StringRectangle {
    make_column(&vec![" ".to_string(); line_count])
}

/// The `rectangle_row_element_separator` is string rectangle that need to be inserted between
/// column of names of table and first column of elements of table.
fn rectangle_row_element_separator(line_count: usize) -> StringRectangle {
    make_column(&vec![" ┃ ".to_string(); line_count])
}

/// The `table_data_to_rectangle` function turns elements of given table into matrix as string
/// rectangle.
fn table_data_to_rectangle(table: &Table) -> StringRectangle {
    let mut matrix_rectangle = StringRectangle::new();

    for column_index in 0..table.column_count() {
        let column = make_column(
            &(0..table.row_count())
                .map(|row_index| table.get(row_index, column_index).unwrap().clone())
                .collect::<Vec<_>>(),
        );

        matrix_rectangle = matrix_rectangle
            .place_right_top(column)
            .place_right_top(rectangle_elements_vseparator(table.row_count()));
    }

    matrix_rectangle
}

/// The `stringify_table` function writes to given string stringified given table. There's no extra
/// characters in the end and in the start.
pub fn stringify_table(table: &Table, write_to: &mut String) {
    let rows_rectangle = make_column(table.row_names());
    let data_matrix = table_data_to_rectangle(table);

    let full_table_rectangle = rows_rectangle
        .place_right_top(rectangle_row_element_separator(table.row_count()))
        .place_right_top(data_matrix);

    write_to.push_str(full_table_rectangle.to_string().as_str());
}
