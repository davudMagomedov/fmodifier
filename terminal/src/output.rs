mod core_error_impl;
mod parse_error_impl;
mod tokenize_error_impl;

pub trait ToOutput {
    fn to_output(self) -> String;
}
