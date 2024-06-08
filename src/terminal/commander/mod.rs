pub mod file;
pub mod terminal;

pub use file::FileReader;
pub use terminal::Terminal;

/// The `Commander` trait issues commands an accepts results. What to do with the results and how
/// to issue commands is determined by the implementation.
pub trait Commander {
    fn is_terminal(&self) -> bool;
    fn read_command(&mut self) -> Option<String>;
    fn write_result(&mut self, result: String);
}
