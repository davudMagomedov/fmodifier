mod core_error_impl;
mod core_output_impl;
mod parse_error_impl;
mod tokenize_error_impl;
mod tokens_translation_error_impl;

pub trait ToOutput {
    /// The `output` function turns the self into a string. In the end of returning string there
    /// must be the new-line symbol.
    fn to_output(self) -> String;
}
