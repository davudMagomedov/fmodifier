const ANY_INDEX: usize = 0;
const ANY_USIZE_VALUE: usize = 0;

const EMPTY_CHAR: char = ' ';

const UTF_8_ERROR: &str = "UTF-8 error";

#[derive(Debug, Clone)]
pub struct StringRectangle {
    // INVARIANT:
    // - The sizes of all lines in `lines` are the same.

    //
    lines: Vec<Vec<char>>,
}

impl StringRectangle {
    pub const fn new() -> Self {
        StringRectangle { lines: Vec::new() }
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

    /// The `fill` function creates string rectangle with given shape and fills it by the given
    /// value.
    pub fn fill(width: usize, height: usize, fill_by: char) -> StringRectangle {
        debug_assert!(fill_by.is_ascii());
        debug_assert_ne!(fill_by, '\n');

        let line = vec![fill_by; width];
        let lines = (0..height)
            .map(|_| line.clone())
            .collect::<Vec<Vec<char>>>();

        StringRectangle { lines }
    }

    /// The `place_right_top` function takes a string rectangle and places it to the right of the
    /// current one. If the current string rectangle has size greater than given one, the function
    /// will place given string rectangle in top.
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

            other.place_left(
                // SAFETY: `self.width == empty_rectangle.width`.
                unsafe { self.place_bottom_unchecked(empty_rectangle) },
            )
        }
    }

    /// The `place_right_bottom` function takes a string rectangle and places it to the right of
    /// the currrent one. If the current string rectangle has size greater than given one, the
    /// function will place given string rectangle in bottom.
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

            other.place_left(
                // SAFETY: `self.width == empty_rectangle.width`.
                unsafe { self.place_bottom_unchecked(empty_rectangle) },
            )
        }
    }

    /// The `place_left_top` function takes a string rectangle and places it to the left of the
    /// current one. If the current string rectangle has size greater than given one, the function
    /// will place given string rectangle in top.
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
    /// s1.place_left_top(s2) is 5x12 rectangle = [
    ///     [HELLO  DAVUD]
    ///     [CONV   LEON ]
    ///     [FMODIF HI   ]
    ///     [       BAN  ]
    ///     [       SOME ]
    /// ]
    /// ```
    pub fn place_left_top(self, other: StringRectangle) -> StringRectangle {
        if self.length() >= other.length() {
            // Then, `self.length()` is greater than `other.length()` on `larger`.
            let empty_rectangle_length = self.length() - other.length();
            let empty_rectangle_width = other.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            self.place_left(
                // SAFETY: `other.width == empty_rectangle.width`.
                unsafe { other.place_bottom_unchecked(empty_rectangle) },
            )
        } else {
            // Then, `self.length()` is less than `other.length()` on `larger`.
            let empty_rectangle_length = other.length() - self.length();
            let empty_rectangle_width = self.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            other.place_right(
                // SAFETY: `self.width == empty_rectangle.width`.
                unsafe { self.place_bottom_unchecked(empty_rectangle) },
            )
        }
    }

    /// The `place_left_bottom` function takes a string rectangle and places it to hte left of the
    /// current one. If the current string rectangle has size greater than give one, the function
    /// will place given string rectangle in bottom.
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
    /// s1.place_left_bottom(s2) is 5x12 rectangle = [
    ///     [       DAVUD]
    ///     [       LEON ]
    ///     [HELLO  HI   ]
    ///     [CONV   BAN  ]
    ///     [FMODIF SOME ]
    /// ]
    /// ```
    pub fn place_left_bottom(self, other: StringRectangle) -> StringRectangle {
        if self.length() >= other.length() {
            // Then, `self.length()` is greater than `other.length()` on `larger`.
            let empty_rectangle_length = self.length() - other.length();
            let empty_rectangle_width = other.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            self.place_left(
                // SAFETY: `other.width == empty_rectangle.width`.
                unsafe { other.place_top_unchecked(empty_rectangle) },
            )
        } else {
            // Then, `self.length()` is less than `other.length()` on `larger`.
            let empty_rectangle_length = other.length() - self.length();
            let empty_rectangle_width = self.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            other.place_right(
                // SAFETY: `self.width == empty_rectangle.width`.
                unsafe { self.place_bottom_unchecked(empty_rectangle) },
            )
        }
    }

    /// The `place_bottom_left` function takes a string rectangle and places it to the bottom of
    /// the current one. If the current string rectangle has width greater than given one, the
    /// function will place given string rectangle in left.
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
    /// s2.place_bottom_left(s1) is 5x12 rectangle = [
    ///     [HELLO  ]
    ///     [CONV   ]
    ///     [FMODIF ]
    ///     [DAVUD  ]
    ///     [LEON   ]
    ///     [HI     ]
    ///     [BAN    ]
    ///     [SOME   ]
    /// ]
    /// ```
    pub fn place_bottom_left(self, other: StringRectangle) -> StringRectangle {
        if self.width() >= other.width() {
            let empty_rectangle_length = other.length();
            let empty_rectangle_width = self.width() - other.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            // SAFETY: `other.place_right(empty_rectangle)` is `[other.length X self.width]`
            // rectangle and `self` is `[self.length X self.width]` rectangle.
            unsafe { self.place_bottom_unchecked(other.place_right(empty_rectangle)) }
        } else {
            let empty_rectangle_length = self.length();
            let empty_rectangle_width = other.width() - self.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            // SAFETY: `self.place_right(empty_rectangle)` is `[self.length X other.width]` and
            // `other` is `[other.length X other.width]` rectangle.
            unsafe { other.place_top_unchecked(self.place_right(empty_rectangle)) }
        }
    }

    /// The `place_bottom_right` fucntion takes a string rectangle and places it to the bottom of
    /// the current one. If the current string rectangle has width greater than given one, the
    /// function will palce given string in right.
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
    /// s2.place_bottom_right(s1) is 5x12 rectangle = [
    ///     [HELLO  ]
    ///     [CONV   ]
    ///     [FMODIF ]
    ///     [  DAVUD]
    ///     [  LEON ]
    ///     [  HI   ]
    ///     [  BAN  ]
    ///     [  SOME ]
    /// ]
    /// ```
    pub fn place_bottom_right(self, other: StringRectangle) -> StringRectangle {
        if self.width() >= other.width() {
            let empty_rectangle_length = other.length();
            let empty_rectangle_width = self.width() - other.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            // SAFETY: `other.place_left(empty_rectangle)` is `[other.length X self.width]`
            // rectangle and `self` is `[self.length X self.width]` rectangle.
            unsafe { self.place_bottom_unchecked(other.place_left(empty_rectangle)) }
        } else {
            let empty_rectangle_length = self.length();
            let empty_rectangle_width = other.width() - self.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            // SAFETY: `self.place_right(empty_rectangle)` is `[self.length X other.width]` and
            // `other` is `[other.length X other.width]` rectangle.
            unsafe { other.place_top_unchecked(self.place_right(empty_rectangle)) }
        }
    }

    /// The `place_top_left` function takes a string rectangle and places it to the top of the
    /// current one. If the current string rectangle has width greater than given one, the function
    /// will will place given string in left.
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
    /// s2.place_top_left(s1) is 5x12 rectangle = [
    ///     [DAVUD  ]
    ///     [LEON   ]
    ///     [HI     ]
    ///     [BAN    ]
    ///     [SOME   ]
    ///     [HELLO  ]
    ///     [CONV   ]
    ///     [FMODIF ]
    /// ]
    /// ```
    pub fn place_top_left(self, other: StringRectangle) -> StringRectangle {
        if self.width() >= other.width() {
            let empty_rectangle_length = other.length();
            let empty_rectangle_width = self.width() - other.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            // SAFETY: `other.place_right(empty_rectangle)` is `[other.length X self.width]`
            // rectangle and `self` is `[self.length X self.width]` rectangle.
            unsafe { self.place_top_unchecked(other.place_right(empty_rectangle)) }
        } else {
            let empty_rectangle_length = self.length();
            let empty_rectangle_width = other.width() - self.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            // SAFETY: `self.place_right(empty_rectangle)` is `[self.length X other.width]` and
            // `other` is `[other.length X other.width]`.
            unsafe { other.place_bottom_unchecked(self.place_right(empty_rectangle)) }
        }
    }

    /// The `place_top_right` function takes a string rectangle and places in to the top of the
    /// current one. If the current string rectangle has width greater than given one, the function
    /// will place given string in right.
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
    /// s2.place_top_right(s1) is 5x12 rectangle = [
    ///     [  DAVUD]
    ///     [  LEON ]
    ///     [  HI   ]
    ///     [  BAN  ]
    ///     [  SOME ]
    ///     [HELLO  ]
    ///     [CONV   ]
    ///     [FMODIF ]
    /// ]
    /// ```
    pub fn place_top_right(self, other: StringRectangle) -> StringRectangle {
        if self.width() >= other.width() {
            let empty_rectangle_length = other.length();
            let empty_rectangle_width = self.width() - other.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            // SAFETY: `other.place_left(empty_rectangle)` is `[other.length X self.width]`
            // rectangle and `self` is `[self.length X self.width]` rectangle.
            unsafe { self.place_top_unchecked(other.place_left(empty_rectangle)) }
        } else {
            let empty_rectangle_length = self.length();
            let empty_rectangle_width = other.width() - self.width();

            let empty_rectangle =
                StringRectangle::fill(empty_rectangle_width, empty_rectangle_length, EMPTY_CHAR);

            // SAFETY: `self.place_right(empty_rectangle)` is `[self.length X other.width]` and
            // `other` is `[other.length X other.width]`.
            unsafe { other.place_bottom_unchecked(self.place_right(empty_rectangle)) }
        }
    }

    /// The `place_right` function places a string rectangle in right of the current one. If number
    /// of the lengths of two rectangles are different, the function cuts one of two to another
    /// one.
    fn place_right(self, other: StringRectangle) -> StringRectangle {
        StringRectangle {
            lines: self
                .lines
                .into_iter()
                .zip(other.lines.into_iter())
                .map(|(mut self_line, other_line)| {
                    self_line.extend(other_line);
                    self_line
                })
                .collect(),
        }
    }

    /// The `place_left` function places given string rectangle in the left of the current one. If
    /// lengths of the two rectangles are different, the function cuts one of two to length of
    /// another one.
    pub fn place_left(self, other: StringRectangle) -> StringRectangle {
        StringRectangle {
            lines: self
                .lines
                .into_iter()
                .zip(other.lines.into_iter())
                .map(|(self_line, mut other_line)| {
                    other_line.extend(self_line);
                    other_line
                })
                .collect(),
        }
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
}

impl From<&str> for StringRectangle {
    fn from(value: &str) -> Self {
        let lines: Vec<&str> = value.lines().collect();

        let max_len = lines
            .iter()
            .max_by_key(|&&string| string.len())
            .unwrap_or(&"")
            .len();

        let lines: Vec<_> = lines
            .into_iter()
            .map(|line| {
                line.chars()
                    .chain(vec![' '; max_len - line.len()])
                    .collect::<Vec<_>>()
            })
            .collect();

        StringRectangle { lines }
    }
}

impl ToString for StringRectangle {
    fn to_string(&self) -> String {
        if self.lines.is_empty() {
            return "".to_string();
        }

        let mut string = String::new();

        for line in &self.lines[..self.lines.len() - 1] {
            for &ch in line {
                string.push(ch);
            }
            string.push('\n');
        }

        for &ch in &self.lines[self.lines.len() - 1] {
            string.push(ch);
        }

        string
    }
}

