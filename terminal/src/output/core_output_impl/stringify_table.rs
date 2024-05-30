use core::output::*;

use std::collections::VecDeque;

const ANY_INDEX: usize = 0;

struct StringRectangle {
    // INVARIANT:
    // - All strings in `lines` are the same.

    //
    lines: VecDeque<String>,
}

impl StringRectangle {
    pub fn new() -> Self {
        StringRectangle {
            lines: VecDeque::new(),
        }
    }

    /// The `fill` function creates string rectangle with given shape and fills it by the given
    /// value.
    pub fn fill(width: usize, height: usize, fill_by: char) -> StringRectangle {
        let line = String::from_utf8(vec![fill_by as u8; width]).expect("UTF-8 error");
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
