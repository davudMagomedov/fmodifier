use super::command::CoreCommand;
use super::commands::*;
use super::core_e::CoreResult;
use super::output::CoreOutput;
use super::variables::Variables;

pub struct Core {
    pub(super) variables: Variables,
}

impl Core {
    /// The `new` function creates empty core.
    pub fn new() -> Core {
        Core {
            variables: Variables::new(),
        }
    }

    /// The `execute` function takes command and execute it. The returning value is the result of
    /// command's executing.
    ///
    /// A returning value is `Result<CoreOutput, CoreError>`. `CoreOutput` is what the command have
    /// written. For example, `show_buffer` command writes to `CoreOutput` bunch of bytes.
    pub fn execute(&mut self, command: CoreCommand) -> CoreResult<CoreOutput> {
        match command {
            CoreCommand::MakeBuffer {
                buffer_name,
                buffer_size,
            } => make_buffer(self, buffer_name, buffer_size),
            CoreCommand::FillBuffer {
                buffer_name,
                value,
                start,
                end,
            } => fill_buffer(self, buffer_name, value, start, end),
            CoreCommand::ShowBuffer {
                buffer_name,
                start,
                end,
            } => show_buffer(self, buffer_name, start, end),
            CoreCommand::BufferInfo { buffer_name } => buffer_info(self, buffer_name),
            CoreCommand::CreateFile {
                file_name,
                file_size,
            } => create_file(self, file_name, file_size),
            CoreCommand::BufferSetByte {
                buffer_name,
                index,
                value,
            } => buffer_set_byte(self, buffer_name, index, value),
            CoreCommand::FromFileToBuffer {
                file_name,
                buffer_name,
                bytes_count,
                file_start,
                buffer_start,
            } => from_file_to_buffer(
                self,
                file_name,
                buffer_name,
                bytes_count,
                file_start,
                buffer_start,
            ),
            CoreCommand::FromBufferToFile {
                buffer_name,
                file_name,
                bytes_count,
                buffer_start,
                file_start,
            } => from_buffer_to_file(
                self,
                buffer_name,
                file_name,
                bytes_count,
                buffer_start,
                file_start,
            ),
            CoreCommand::OpenFile { file_name } => open_file(self, file_name),
            CoreCommand::ShowFile {
                file_name,
                start,
                end,
            } => show_file(self, file_name, start, end),
            CoreCommand::BufferWriteBytes {
                buffer_name,
                start,
                bytes,
            } => buffer_write_bytes(self, buffer_name, start, &bytes),
            CoreCommand::MergeBuffers {
                left_buffer_name,
                right_buffer_name,
                new_buffer_name,
            } => merge_buffers(self, left_buffer_name, right_buffer_name, new_buffer_name),
            CoreCommand::PullOutSlice {
                buffer_name,
                new_buffer_name,
                start,
                end,
            } => pull_out_slice(self, buffer_name, new_buffer_name, start, end),
            CoreCommand::TurnBufferToFile {
                buffer_name,
                new_file_name,
            } => turn_buffer_to_file(self, buffer_name, new_file_name),
            CoreCommand::TurnFileToBuffer {
                file_name,
                new_buffer_name,
            } => turn_file_to_buffer(self, file_name, new_buffer_name),
            CoreCommand::SetVariableInteger {
                variable_name,
                value,
            } => set_variable_integer(self, variable_name, value),
            CoreCommand::SetVariableString {
                variable_name,
                value,
            } => set_variable_string(self, variable_name, value),
            CoreCommand::GetVariable { variable_name } => get_variable(self, variable_name),
            CoreCommand::Nop => Ok(CoreOutput::new()),
        }
    }

    /// The `buffer_size` function returns size of the buffer with given name. If there's no buffer
    /// with the name, the function returns `None`.
    pub fn buffer_size(&self, buffer_name: &str) -> Option<usize> {
        self.variables
            .get_buffer(buffer_name)
            .map(|buffer| buffer.len())
    }
}
