use crate::core::command::CoreCommand;
use crate::core::core::Core;
use crate::core::core_e::CoreResult;
use crate::core::output::CoreOutput;

pub fn execute_core_command(core: &mut Core, command: CoreCommand) -> CoreResult<CoreOutput> {
    core.execute(command)
}
