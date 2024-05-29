use core::token::Token;

pub mod error {
    use std::error::Error as ErrorTrait;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    #[derive(Debug)]
    pub enum TokenizeError<'a> {
        CouldNotTokenizeWord { word: &'a str },
    }

    impl<'a> TokenizeError<'a> {
        pub fn couldnot_tokenize_word(word: &'a str) -> Self {
            TokenizeError::CouldNotTokenizeWord { word }
        }
    }

    impl<'a> Display for TokenizeError<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "TokenizeError")
        }
    }

    impl<'a> ErrorTrait for TokenizeError<'a> {}
}

pub use error::TokenizeError;

/// The `tokenize_integer` returns `Token::UInt` taking a word.
///
/// Accepted guarantees:
/// - `is_integer(word)`.
/// - `!word.is_empty()`
fn tokenize_integer(word: &str) -> Token {
    debug_assert!(is_integer(word));
    debug_assert!(!word.is_empty());

    Token::uinteger(word.parse::<usize>().unwrap())
}

fn is_integer(word: &str) -> bool {
    word.chars().all(|ch| matches!(ch, '0'..='9'))
}

/// The `tokenize_name` returns `Token::Word` taking a word.
///
/// Accepted guarantees:
/// - `is_name(word)`.
/// - `!word.is_empty()`
fn tokenize_name(word: &str) -> Token {
    debug_assert!(is_name(word));
    debug_assert!(!word.is_empty());

    Token::word(word.to_string())
}

fn is_name(word: &str) -> bool {
    let mut chars = word.chars();

    matches!(chars.next().unwrap(), 'A'..='Z' | 'a'..='z' | '_')
        && chars.all(|ch| matches!(ch, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))
}

/// The `tokenize_word` function takes word and returns appropriate token. If the function couldn't
/// tokenize the word, the function returns Err.
fn tokenize_word(word: &str) -> Result<Token, TokenizeError> {
    debug_assert!(!word.is_empty());
    debug_assert!(word.find('\n').is_none());
    debug_assert!(word.find('\t').is_none());
    debug_assert!(word.find(' ').is_none());

    if is_integer(word) {
        Ok(tokenize_integer(word))
    } else if is_name(word) {
        Ok(tokenize_name(word))
    } else {
        Err(TokenizeError::couldnot_tokenize_word(word))
    }
}

fn split_on_words(string: &str) -> Vec<&str> {
    string.split_whitespace().collect()
}

/// The `tokenize` function takes a string and parses it to tokens. If the function couldn't parse
/// the string, it returns `Err`.
///
/// #### Example
/// ```ignore
/// use terminal::tokenizer::tokenize;
/// use core::token::Token;
///
/// let tokens = tokenize("make_buffer mbr 512").unwrap();
/// assert_eq!(tokens, vec![
///     Token::word("make_buffer".to_string()),
///     Token::word("mbr".to_string()),
///     Token::uinteger(512),
/// ]);
/// ```
pub fn tokenize(string: &str) -> Result<Vec<Token>, TokenizeError> {
    let words = split_on_words(string);
    words.into_iter().map(|word| tokenize_word(word)).collect()
}
