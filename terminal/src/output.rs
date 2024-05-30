mod core_error_impl;

pub trait ToOutput {
    fn to_output(self) -> String;
}
