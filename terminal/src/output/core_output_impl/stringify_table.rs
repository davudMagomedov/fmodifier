use core::output::*;

use std::collections::VecDeque;

const ANY_INDEX: usize = 0;
const ANY_USIZE_VALUE: usize = 0;

const UTF_8_ERROR: &str = "UTF-8 error";

struct StringRectangle {
    // INVARIANT:
    // - All strings in `lines` are the same.

    //
    lines: VecDeque<String>,
}

impl StringRectangle {
    pub const fn new() -> Self {
        StringRectangle {
            lines: VecDeque::new(),
        }
    }

    /// The `new_with_lines` function returns string rectangle by the given lines.
    ///
    /// Any line in given list of lines must not have `\n` character.
    pub fn new_with_lines(lines: &[&str]) -> Self {
        debug_assert!(lines.into_iter().all(|line| !line.contains('\n')));

        let mut lines: VecDeque<String> =
            lines.into_iter().map(|string| string.to_string()).collect();

        let max_len = lines
            .iter()
            .max_by_key(|string| string.len())
            .map(|string| string.len())
            .unwrap_or(ANY_USIZE_VALUE);

        lines.iter_mut().for_each(|string| {
            let missing_spaces =
                String::from_utf8(vec![b' '; max_len - string.len()]).expect(UTF_8_ERROR);
            string.push_str(&missing_spaces);
        });

        StringRectangle { lines }
    }

    /// The `fill` function creates string rectangle with given shape and fills it by the given
    /// value.
    pub fn fill(width: usize, height: usize, fill_by: char) -> StringRectangle {
        debug_assert!(fill_by.is_ascii());
        debug_assert_ne!(fill_by, '\n');

        let line = String::from_utf8(vec![fill_by as u8; width]).expect(UTF_8_ERROR);
        let lines = (0..height)
            .map(|_| line.clone())
            .collect::<VecDeque<String>>();

        StringRectangle { lines }
    }

    /// The `push_bottom` function inserts the given line to bottom of string rectangle saving
    /// proportion.
    ///
    /// The given line must not contain new line symbol.
    pub fn push_bottom(&mut self, line_to_push: String) {
        debug_assert!(!line_to_push.contains("\n"));

        if self.lines.is_empty() {
            self.lines.push_back(line_to_push);
            return;
        }

        let mut inserting_line = line_to_push;
        self.adjust(&mut inserting_line);

        self.lines.push_back(inserting_line);
    }

    /// The `push_top` function inserts the given line to top of string rectangle saving
    /// proportion.
    ///
    /// The given line must not contain new line symbol.
    pub fn push_top(&mut self, line_to_push: String) {
        debug_assert!(!line_to_push.contains("\n"));

        if self.lines.is_empty() {
            self.lines.push_back(line_to_push);
            return;
        }

        let mut inserting_line = line_to_push;
        self.adjust(&mut inserting_line);

        self.lines.push_front(inserting_line);
    }

    /// The `place_right` takes other string rectangle and places it in right of the current one.
    ///
    /// Count of lines of other string rectangle must be the same as in the current one. Otherwise,
    /// the function will panic.
    pub fn place_right(mut self, other: StringRectangle) -> StringRectangle {
        debug_assert_eq!(self.lines.len(), other.lines.len());

        if self.lines.len() != other.lines.len() {
            panic!(
                "Two rectangles has a different count of lines: {} and {}",
                self.lines.len(),
                other.lines.len()
            );
        }

        for (line_index, line) in self.lines.iter_mut().enumerate() {
            // Each line in `self.lines` has the same size A and each line in `other.lines` has the
            // same size B. So, new string rectangle will have lines with the same size A + B.
            line.push_str(&other.lines[line_index]);
        }

        self
    }

    /// The `place_left` takes other string rectangle and places it in left of the current one.
    ///
    /// Count of lines of other string rectangle must be the same as in the current one.
    pub fn place_left(self, other: StringRectangle) -> StringRectangle {
        other.place_right(self)
    }

    /// The `adjust` function does either expand given string to make its len the same like in
    /// lines or expand each line to make their len the same like in the string.
    ///
    /// A string expands filling a space char.
    fn adjust(&mut self, line_to_push: &mut String) {
        let line_len = line_to_push.len();
        let rectangle_len = self.lines[ANY_INDEX].len();

        if rectangle_len >= line_len {
            let missing_spaces =
                unsafe { String::from_utf8_unchecked(vec![b' '; rectangle_len - line_len]) };
            line_to_push.push_str(&missing_spaces);
        } else {
            let missing_spaces =
                unsafe { String::from_utf8_unchecked(vec![b' '; line_len - rectangle_len]) };
            for line in &mut self.lines {
                line.push_str(&missing_spaces);
            }
        }
    }
}

impl ToString for StringRectangle {
    fn to_string(&self) -> String {
        let mut string_rect = "".to_string();

        self.lines
            .iter()
            .for_each(|line| string_rect.push_str(line));

        string_rect
    }
}

struct Matrix {
    // INVARIANT: `data.len() % row_count == 0`, because `data.len() % row_count` is column count.

    // data: [...COLUMNS, ...COLUMNS, ...COLUMNS, ...]
    data: Vec<String>,
    row_count: usize,
}

impl Matrix {
    pub fn get(&self, row: usize, column: usize) -> Option<&String> {
        let column_count = self.data.len() / self.row_count;
        self.data.get(row * column_count + column)
    }
}

/// The `from<Table>` copies all data in fields to new matrix, columns names and row names. For
/// example, if you had table 3x4 (3 rows and 4 columns), you'll get matrix 4x5: column names and
/// row names are added to data.
impl From<&Table> for Matrix {
    fn from(value: &Table) -> Self {
        let column_names = value.column_names();
        let row_names = value.row_names();

        let mut matrix_data: Vec<String> = Vec::new();

        matrix_data.extend(
            column_names
                .into_iter()
                .map(|short_string| short_string.get().to_string()),
        );

        let table_column_count = column_names.len();
        let table_row_count = row_names.len();

        (0..table_row_count).for_each(|row_index| {
            matrix_data.push(row_names[row_index].clone());
            matrix_data.extend((0..table_column_count).map(|column_index| {
                value
                    .get(row_index, column_index)
                    .unwrap() // `row_index` and `column_index` are right in any way.
                    .get()
                    .to_string()
            }));
        });

        Matrix {
            data: matrix_data,
            row_count: table_row_count + 1,
        }
    }
}

impl ToString for Matrix {
    fn to_string(&self) -> String {
        let column_count = self.data.len() / self.row_count;

        let maybe_rectangle = (0..column_count)
            .map(|column_index| {
                let lines = &(0..self.row_count)
                    .map(|row_index| self.get(row_index, column_index).unwrap().as_str())
                    .collect::<Vec<&str>>();

                StringRectangle::new_with_lines(lines)
            })
            .reduce(|acc, str_rect| {
                let tab = StringRectangle::new_with_lines(&vec![" "; self.row_count]);
                acc.place_right(tab).place_right(str_rect)
            });

        let rectangle = match maybe_rectangle {
            Some(rect) => rect,
            None => return "".to_string(),
        };

        rectangle.to_string()
    }
}

/// The `stringify_table` function writes to given string stringified given table. There's no extra
/// characters in the end and in the start.
pub fn stringify_table(table: &Table, write_to: &mut String) {
    let matrix = Matrix::from(table);
    write_to.push_str(&matrix.to_string());
}
