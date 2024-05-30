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
