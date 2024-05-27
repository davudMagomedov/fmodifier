use crate::core::command::CoreCommand;

pub enum Command {
    Core(CoreCommand),
    BufferWriteBytes {
        buffer_name: String,
        start: usize,
        bytes: Vec<u8>,
    },
    PullOutSlice {
        buffer_name: String,
        new_buffer_name: String,
    },
    MergeBuffers {
        left_buffer_name: String,
        right_buffer_name: String,
        new_buffer_name: String,
    },
}
