#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Name(String),
    UInt(usize),
}
