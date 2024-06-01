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

    /// The `place_bottom` function takes other string rectangle and places it in the bottom of the
    /// current one.
    pub fn place_bottom(mut self, other: StringRectangle) -> StringRectangle {
        other
            .lines
            .into_iter()
            .for_each(|line_of_other| self.push_bottom(line_of_other));
        self
    }

    /// The `place_top` function takes other string rectangle and places it in the top of the
    /// current one.
    pub fn place_top(self, other: StringRectangle) -> StringRectangle {
        other.place_bottom(self)
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
        if self.lines.is_empty() {
            return "".to_string();
        }

        let mut string_rect = "".to_string();

        self.lines
            .iter()
            .take(self.lines.len() - 1)
            .for_each(|line| {
                string_rect.push_str(line);
                string_rect.push('\n');
            });

        string_rect.push_str(&self.lines[self.lines.len() - 1]);

        string_rect
    }
}

fn rectangle_tab(line_count: usize) -> StringRectangle {
    StringRectangle::new_with_lines(&vec![" "; line_count])
}

fn rectangle_row_tab(line_count: usize) -> StringRectangle {
    StringRectangle::new_with_lines(&vec![" | "; line_count])
}

fn table_data_to_rectangle(table: &Table) -> StringRectangle {
    (0..table.column_names().len())
        .map(|column_index| {
            StringRectangle::new_with_lines(
                &(0..table.row_names().len())
                    .map(|row_index| table.get(row_index, column_index).unwrap().as_str())
                    .collect::<Vec<_>>(),
            )
            .place_right(rectangle_tab(table.row_names().len()))
        })
        .reduce(|acc, rect| acc.place_right(rect))
        .unwrap_or_else(|| StringRectangle::new())
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
    let data_rectangle = table_data_to_rectangle(table);

    // [ rows_rectangle  ] [ data_rectangle    ]
    let rectangle = rows_rectangle
        .place_right(rectangle_row_tab(table.row_names().len()))
        .place_right(data_rectangle);

    write_to.push_str(&rectangle.to_string());
}

#[test]
fn str_rect_test() {
    let sr_1 = StringRectangle::new_with_lines(&vec!["ABC", "AB", "ABCDE", "AAA"]);
    assert_eq!(sr_1.to_string(), "ABC  \nAB   \nABCDE\nAAA  ");

    let sr_2 = StringRectangle::new_with_lines(&vec!["H E", "TPAA PLEN", "OR", "P  T"]);
    assert_eq!(
        sr_2.to_string(),
        "H E      \nTPAA PLEN\nOR       \nP  T     "
    );

    let sr_3 = sr_1.place_right(sr_2);
    assert_eq!(
        sr_3.to_string(),
        "ABC  H E      \nAB   TPAA PLEN\nABCDEOR       \nAAA  P  T     "
    );
}
