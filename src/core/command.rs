/// The `CoreCommand` enumeration contains all variants of commands.
pub enum CoreCommand {
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
        buffer_name: String,
        value: u8,
        start: usize,
        end: usize,
    },
    ShowBuffer {
        buffer_name: String,
        start: usize,
        end: usize,
    },
    BufferInfo {
        buffer_name: String,
    },
    BufferSetByte {
        buffer_name: String,
        index: usize,
        value: u8,
    },
    CreateFile {
        file_name: String,
        file_size: usize,
    },
    FromFileToBuffer {
        file_name: String,
        buffer_name: String,
        bytes_count: usize,
        file_start: usize,
        buffer_start: usize,
    },
    FromBufferToFile {
        buffer_name: String,
        file_name: String,
        bytes_count: usize,
        buffer_start: usize,
        file_start: usize,
    },
}
