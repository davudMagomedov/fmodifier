use crate::core::command::CoreCommand;

pub enum Command<'a> {
    Core(CoreCommand<'a>),
    BufferWriteBytes {
        buffer_name: &'a str,
        start: usize,
        bytes: &'a [u8],
    },
    PullOutSlice {
        buffer_name: &'a str,
        new_buffer_name: &'a str,
    },
    MergeBuffers {
        left_buffer_name: &'a str,
        right_buffer_name: &'a str,
        new_buffer_name: &'a str,
    },
}
