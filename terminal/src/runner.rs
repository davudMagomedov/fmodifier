use crate::terminal::Terminal;

use core::Core;

/// The `Runner` structure is iterator in which each iteration means following actions:
/// 1. Take user input.
/// 2. Parse user input.
/// 3. Execute user input.
/// 4. Output result.
///
/// It means that you can stop this runner whenever you want.
pub struct Runner {
    core: Core,
    terminal: Terminal,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            core: Core::new(),
            terminal: Terminal::new(),
        }
    }
}
