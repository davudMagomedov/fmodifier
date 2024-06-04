use std::collections::VecDeque;

const ANY_INDEX: usize = 0;
const ANY_USIZE_VALUE: usize = 0;

const EMPTY_CHAR: char = ' ';

const UTF_8_ERROR: &str = "UTF-8 error";

pub struct StringRectangle {
    // INVARIANT:
    // - The sizes of all lines in `lines` are the same.

    //
    lines: VecDeque<String>,
}

impl StringRectangle {
    pub const fn new() -> Self {
        StringRectangle {
            lines: VecDeque::new(),
        }
    }

    /// The `length` function returns lenght of the string rectangle. It can return 0 that means
    /// the rectangle is empty.
    pub fn length(&self) -> usize {
        self.lines.len()
    }

    /// The `width` function returns width of the string rectangle. It can return 0 that means the
    /// rectangle is empty.
    pub fn width(&self) -> usize {
        self.lines
            .get(ANY_INDEX)
            .map(|string| string.len())
            .unwrap_or(0)
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

    /// The `place_right_top` function takes a string rectangle and places it to the right of the
    /// current one. If the current string rectangle has size greater than given one, the function
    /// will place given string rectangle in top.
    ///
    /// If the given string rectangle is empty, the function just returns the same rectangle as it
    /// was.
    ///
    /// #### Example
    /// ```text
    /// s1 is 4x5 rectangle = [
    ///     [DAVUD]
    ///     [LEON ]
    ///     [HI   ]
    ///     [BAN  ]
    /// ]
    /// s2 is 3x7 rectangle = [
    ///     [HELLO  ]
    ///     [CONV   ]
    ///     [FMODIF ]
    /// ]
    /// s1.place_right_top(s2) is 4x12 rectangle = [
    ///     [DAVUDHELLO  ]
    ///     [LEON CONV   ]
    ///     [HI   FMODIF ]
    ///     [BAN         ]
    /// ]
    /// ```
    pub fn place_right_top(self, other: StringRectangle) -> StringRectangle {
        if self.length() >= other.length() {
            // Then, `self.length()` is greater than `other.length()` on `larger`.
            let empty_rectangle_length = self.length() - other.length();
            let empty_rectangle_width = other.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            self.place_right(
                // SAFETY: `other.width == empty_rectangle.width`.
                unsafe { other.place_bottom_unchecked(empty_rectangle) },
            )
        } else {
            // Then, `self.length()` is less than `other.length()` on `larger`.
            let empty_rectangle_length = other.length() - self.length();
            let empty_rectangle_width = self.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            (
                // SAFETY: `self.width == empty_rectangle.width`.
                unsafe { self.place_bottom_unchecked(empty_rectangle) }
            )
            .place_right(other)
        }
    }

    /// The `place_right_bottom` function takes a string rectangle and places it to the right of
    /// the currrent one. If the current string rectangle has size greater than given one, the
    /// function will place given string rectangle in bottom.
    ///
    /// If the given string rectangle is empty, the function just returns the same rectangle as it
    /// was.
    ///
    /// #### Example
    /// ```text
    /// s1 is 5x5 rectangle = [
    ///     [DAVUD]
    ///     [LEON ]
    ///     [HI   ]
    ///     [BAN  ]
    ///     [SOME ]
    /// ]
    /// s2 is 3x7 rectangle = [
    ///     [HELLO  ]
    ///     [CONV   ]
    ///     [FMODIF ]
    /// ]
    /// s1.place_right_bottom(s2) is 5x12 rectangle = [
    ///     [DAVUD       ]
    ///     [LEON        ]
    ///     [HI   HELLO  ]
    ///     [BAN  CONV   ]
    ///     [SOME FMODIF ]
    /// ]
    /// ```
    pub fn place_right_bottom(self, other: StringRectangle) -> StringRectangle {
        if self.length() >= other.length() {
            // Then, `self.length()` is greater than `other.length()` on `larger`.
            let empty_rectangle_length = self.length() - other.length();
            let empty_rectangle_width = other.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            self.place_right(
                // SAFETY: `other.width == empty_rectangle.width`.
                unsafe { other.place_top_unchecked(empty_rectangle) },
            )
        } else {
            // Then, `self.length()` is less than `other.length()` on `larger`.
            let empty_rectangle_length = other.length() - self.length();
            let empty_rectangle_width = self.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            (
                // SAFETY: `self.width == empty_rectangle.width`.
                unsafe { self.place_bottom_unchecked(empty_rectangle) }
            )
            .place_right(other)
        }
    }

    /// The `place_right` function places a string rectangle in right of the current one. If number
    /// of the lengths of two rectangles are different, the function cuts one of two to another
    /// one.
    fn place_right(self, other: StringRectangle) -> StringRectangle {
        let lines = self
            .lines
            .into_iter()
            .zip(other.lines.into_iter())
            .map(|(self_line, other_line)| self_line + &other_line)
            .collect::<Vec<_>>();

        StringRectangle::new_with_lines(
            &lines
                .iter()
                .map(|string| string.as_str())
                .collect::<Vec<_>>(),
        )
    }

    /// The `place_left` function places given string rectangle in the left of the current one. If
    /// lengths of the two rectangles are different, the function cuts one of two to length of
    /// another one.
    pub fn place_left(self, other: StringRectangle) -> StringRectangle {
        let lines = self
            .lines
            .into_iter()
            .zip(other.lines.into_iter())
            .map(|(self_line, other_line)| other_line + &self_line)
            .collect::<Vec<_>>();

        StringRectangle::new_with_lines(
            &lines
                .iter()
                .map(|string| string.as_str())
                .collect::<Vec<_>>(),
        )
    }

    /// The `place_bottom` function takes other string rectangle and places it in the bottom of the
    /// current one.
    pub fn place_bottom(mut self, other: StringRectangle) -> StringRectangle {
        unimplemented!();
    }

    /// The `place_top` function takes other string rectangle and places it in the top of the
    /// current one.
    pub fn place_top(self, other: StringRectangle) -> StringRectangle {
        unimplemented!();
    }

    /// The `place_bottom_unchecked` places a string rectange in the bottom of the current one
    /// without adjusting.
    ///
    /// #### SAFETY
    /// - `self.width == other.width` or either `self` or `other` is empty.
    unsafe fn place_bottom_unchecked(mut self, other: StringRectangle) -> StringRectangle {
        self.lines.extend(other.lines);
        self
    }

    /// The `place_top_unchecked` places a string rectangle in the top of the current one without
    /// adjusting.
    ///
    /// #### SAFETY:
    /// - `self.width == other.width` or either `self` or `other` is empty.
    unsafe fn place_top_unchecked(self, mut other: StringRectangle) -> StringRectangle {
        other.lines.extend(self.lines);
        other
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
