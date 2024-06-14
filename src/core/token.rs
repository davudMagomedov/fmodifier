#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    UInt(usize),
    Word(String),
    Variable(String),
}

impl Token {
    /// The `uinteger` function returns Token as integer.
    pub fn uinteger(val: usize) -> Token {
        Token::UInt(val)
    }

    /// The `word` function returns Token as word.
    pub fn word(val: String) -> Token {
        Token::Word(val)
    }

    /// The `variable` function returns Token as variable.
    pub fn variable(varname: String) -> Token {
        Token::Variable(varname)
    }
}
