use super::*;

fn info(buffer_name: &str, start: usize, count_of_written: usize) -> String {
    format!("Bytes in buffer '{buffer_name}' starting from {start} were written. Count of written bytes: {count_of_written}.")
}

/// NABWW - Not All Bytes Were Written
#[allow(non_snake_case)]
fn warning_NABWW() -> String {
    format!("Not all bytes were written.")
}

pub fn buffer_write_bytes(
    core: &mut Core,
    buffer_name: &str,
    start: usize,
    bytes: &[u8],
) -> CoreResult<CoreOutput> {
    let buffer = core
        .variables
        .get_buffer_mut(buffer_name)
        .ok_or_else(|| CoreError::undefined_variable(buffer_name.to_string()))?;

    let count_of_written = buffer.write_bytes(bytes, start);

    let mut output = CoreOutput::new();
    output.push_info(info(buffer_name, start, count_of_written));

    if count_of_written < bytes.len() {
        output.push_warning(warning_NABWW());
    }

    Ok(output)
}
