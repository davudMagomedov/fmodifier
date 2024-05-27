use crate::core::command::CoreCommand;
use crate::core::core::Core;
use crate::core::core_e::CoreResult;
use crate::core::output::CoreOutput;

/// The `buffer_write_bytes` function writes given bytes to buffer starting from given index.
pub fn buffer_write_bytes(
    core: &mut Core,
    buffer_name: &str,
    start: usize,
    bytes: &[u8],
) -> CoreResult<CoreOutput> {
    for (offset, &byte) in bytes.into_iter().enumerate() {
        core.execute(CoreCommand::BufferSetByte {
            buffer_name,
            index: start + offset,
            value: byte,
        })?;
    }

    Ok(CoreOutput::new())
}
