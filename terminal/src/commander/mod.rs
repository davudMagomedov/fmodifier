pub trait Commander {
    fn read_command(&mut self) -> String;
    fn write_result(&mut self, result: String);
}
