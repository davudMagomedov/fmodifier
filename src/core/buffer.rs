/// The `Buffer` structure is just a sequence of bytes in given size.
#[derive(Clone)]
pub struct Buffer {
    data: Box<[u8]>,
}

impl Buffer {
    /// The `new` function creates a buffer in given size. All bytes in new buffer is 0.
    pub fn new(size: usize) -> Self {
        Buffer {
            data: Box::from_iter((0..size).map(|_| 0_u8)),
        }
    }

    /// The `len` function returns lenght of buffer. Are you really didn't realize?
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // Write Methods {

    /// The `write_byte` function sets byte in a given index to given value. If a given index is
    /// wrong anyway, the function causes panic.
    pub fn write_byte(&mut self, byte: u8, index: usize) {
        self.data[index] = byte;
    }

    /// The `write_bytes` function writes bytes from bytes in first argument starting with index in
    /// second argument. The function returns count of written bytes.
    ///
    /// It means that if you call `buffer.write_bytes(bytes, 3)`, in `buffer.data[3]` there'll be
    /// `bytes[0]`, in `buffer.data[4]` there'll be `bytes[1]`, etc.
    ///
    /// If an end was reached early than all bytes in first argument were written, the function
    /// doesn't raise any error; just returning count will be less that length of given bytes.
    pub fn write_bytes(&mut self, bytes: &[u8], start_with: usize) -> usize {
        bytes
            .into_iter()
            .zip(self.data.iter_mut().skip(start_with))
            .map(|(&byte, buf_cell)| *buf_cell = byte)
            .count()
    }

    /// The `fill_bytes` function fill bytes starting from `start` and ending at `end` indexes. If
    /// `start` go beyond the boundaries, the function returns `None`. The function returns count
    /// of written bytes.
    pub fn fill_bytes(&mut self, fill_by: u8, start: usize, end: usize) -> Option<usize> {
        Some(
            self.data
                .get_mut(start..end.min(self.len()))?
                .into_iter()
                .map(|t| *t = fill_by)
                .count(),
        )
    }

    // }

    // Read Methods {

    /// The `read_byte` returns a byte from a given index. If index is wrong anyway, the function
    /// causes panic.
    pub fn read_byte(&self, index: usize) -> u8 {
        self.data[index]
    }

    /// The `read_bytes` returns a slice of buffer's bytes starting from the first argument and
    /// ending by the second argument. If `start` goes beyond the boundaries, the function returns
    /// `None`.
    pub fn read_bytes(&self, start: usize, end: usize) -> Option<&[u8]> {
        self.data.get(start..end.min(self.data.len()))
    }

    // }
}
