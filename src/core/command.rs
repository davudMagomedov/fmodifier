/// The `CoreCommand` enumeration contains all variants of commands.
pub enum CoreCommand<'a> {
    // Command structure convention:
    // - There's following variants of naming:
    //     + <Action><Object>. For example: `MakeBuffer` (Action - `Make` and Object - `Buffer`).
    //     + <Subject><Action>. For example: `BufferSetByte` (Subject - `Buffer`, Method - `SetByte`).
    //     + Logically justified construction. For example: `FromFileToBuffer` or `FromBufferToFile`.
    // - First arguments are objects/subobjects.
    // - Second argumetns are arguments.

    //
    MakeBuffer {
        buffer_name: String,
        buffer_size: usize,
    },
    FillBuffer {
        buffer_name: &'a str,
        value: u8,
        start: usize,
        end: usize,
    },
    ShowBuffer {
        buffer_name: &'a str,
        start: usize,
        end: usize,
    },
    BufferInfo {
        buffer_name: &'a str,
    },
    BufferSetByte {
        buffer_name: &'a str,
        index: usize,
        value: u8,
    },
    CreateFile {
        file_name: String,
        file_size: usize,
    },
    FromFileToBuffer {
        file_name: &'a str,
        buffer_name: &'a str,
        bytes_count: usize,
        file_start: usize,
        buffer_start: usize,
    },
    FromBufferToFile {
        buffer_name: &'a str,
        file_name: &'a str,
        bytes_count: usize,
        buffer_start: usize,
        file_start: usize,
    },
    OpenFile {
        file_name: String,
    },
    ShowFile {
        file_name: &'a str,
        start: usize,
        end: usize,
    },
    BufferWriteBytes {
        buffer_name: &'a str,
        start: usize,
        bytes: Vec<u8>,
    },
    MergeBuffers {
        left_buffer_name: &'a str,
        right_buffer_name: &'a str,
        new_buffer_name: String,
    },
    PullOutSlice {
        buffer_name: &'a str,
        new_buffer_name: String,
        start: usize,
        end: usize,
    },
    TurnBufferToFile {
        buffer_name: &'a str,
        file_name: String,
    },
    Nop,
}
