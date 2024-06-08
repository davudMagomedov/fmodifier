use crate::terminal::commander::Commander;
use crate::terminal::runner::Runner;

use colored::Colorize;

const PROJECT_NAME: &str = "FModifier";

fn stylize_command_template(command_template: &str) -> String {
    format!("{}", command_template.truecolor(170, 170, 170).italic())
}

/// The `help_mesas`
fn help_message_stylized() -> String {
    format!(
        "\
{} is a handy utility for modifying files. Its main feature is that it allows you to conveniently operate with bytes without having to think about such things as moving the file pointer.
All commands are listed below.
    * {help_tmp} - prints help in general or for command.
    * {exit_tmp} - suprisingly, exits from the program.
    * {make_buffer_tmp} - creates a buffer named <buffer_name> and sized <buffer_size>.
    * {fill_buffer_tmp} - fills bytes from <start> inclusive to <end> not inclusive (bytes are counted from zero) of the buffer with the value value.
    * {show_buffer_tmp} - outputs the contents of the buffer <buffer_name> to the console starting from <start> inclusive and up to <end> not inclusive (bytes are counted from zero).
    * {buffer_info_tmp} - gives information about the buffer <buffer_name>.
    * {buffer_set_byte_tmp} - sets the value of the index <index> of the buffer <buffer_name> to the value <value>.
    * {create_file_tmp} - creates a file with the name <file> and the size <file_size>.
    * {from_file_to_buffer_tmp} - copies bytes in the amount of <bytes_count> pieces from the file <file_name> starting from the <file_start>th byte (bytes count from zero) to the buffer, which is being written starting from the <buffer_start> th byte.
    * {from_buffer_to_file_tmp} - copies bytes in the amount of <bytes_count> pieces from the buffer <buffer_name> starting from the <buffer_start>th byte (bytes are counted from zero) to the file <file_name>, which is recorded starting from <file_start>.
    * {buffer_write_bytes_tmp} - writes a sequence of bytes <...buffer> (bytes are separated by a space) in the buffer <buffer_name> starting from the position <start> (bytes count starts from zero).
    * {pull_out_slice_tmp} - creates a new buffer <new_buffer_name> with the exact size of <end> - <start> bytes and writes there the values from the buffer <buffer_name> starting with <start> and ending with <end>.
    * {merge_buffers_tmp} - creates a new buffer <new_buffer_name>, in which the first part of the bytes is copied from <left_buffer_name>, and the second part is copied from the buffer <right_buffer_name>. That is, the size of the new buffer is equal to the sum of the other two.
    * {open_file_tmp} - opens a file named <file_name> in the current directory strictly for reading.
    * {show_file_tmp} - reads a file named <file_name> starting with <start> and ending with <end>.
",
        PROJECT_NAME.bold(),
        help_tmp = stylize_command_template("help [<command>]"),
        exit_tmp = stylize_command_template("exit"),
        make_buffer_tmp = stylize_command_template("make_buffer <buffer_name> <buffer_size>"),
        fill_buffer_tmp = stylize_command_template("fill_buffer <buffer_name> <value> <start> <end>"),
        show_buffer_tmp = stylize_command_template("show_buffer <buffer_name> <start> <end>"),
        buffer_info_tmp = stylize_command_template("buffer_info <buffer_name>"),
        buffer_set_byte_tmp = stylize_command_template("buffer_set_byte <buffer_name> <index> <value>"),
        create_file_tmp = stylize_command_template("create_file <file_name> <file_size>"),
        from_file_to_buffer_tmp = stylize_command_template("from_file_to_buffer <file_name> <buffer_name> <bytes_count> <file_start> <buffer_start>"),
        from_buffer_to_file_tmp = stylize_command_template("from_buffer_to_file <buffer_name> <file_name> <bytes_count> <buffer_start> <file_start>"),
        buffer_write_bytes_tmp = stylize_command_template("buffer_write_bytes <buffer_name> <start> <...bytes>"),
        pull_out_slice_tmp = stylize_command_template("pull_out_slice <buffer_name> <new_buffer_name> <start> <end>"),
        merge_buffers_tmp = stylize_command_template("merge_buffers <left_buffer_name> <right_buffer_name> <new_buffer_name>"),
        open_file_tmp = stylize_command_template("open_file <file_name>"),
        show_file_tmp = stylize_command_template("show_file <file_name> <start> <end>"),
    )
}

fn help_message_regular() -> String {
    format!(
        "\
{} is a handy utility for modifying files. Its main feature is that it allows you to conveniently operate with bytes without having to think about such things as moving the file pointer.
All commands are listed below.
    * {help_tmp} - prints help in general or for command.
    * {exit_tmp} - suprisingly, exits from the program.
    * {make_buffer_tmp} - creates a buffer named <buffer_name> and sized <buffer_size>.
    * {fill_buffer_tmp} - fills bytes from <start> inclusive to <end> not inclusive (bytes are counted from zero) of the buffer with the value value.
    * {show_buffer_tmp} - outputs the contents of the buffer <buffer_name> to the console starting from <start> inclusive and up to <end> not inclusive (bytes are counted from zero).
    * {buffer_info_tmp} - gives information about the buffer <buffer_name>.
    * {buffer_set_byte_tmp} - sets the value of the index <index> of the buffer <buffer_name> to the value <value>.
    * {create_file_tmp} - creates a file with the name <file> and the size <file_size>.
    * {from_file_to_buffer_tmp} - copies bytes in the amount of <bytes_count> pieces from the file <file_name> starting from the <file_start>th byte (bytes count from zero) to the buffer, which is being written starting from the <buffer_start> th byte.
    * {from_buffer_to_file_tmp} - copies bytes in the amount of <bytes_count> pieces from the buffer <buffer_name> starting from the <buffer_start>th byte (bytes are counted from zero) to the file <file_name>, which is recorded starting from <file_start>.
    * {buffer_write_bytes_tmp} - writes a sequence of bytes <...buffer> (bytes are separated by a space) in the buffer <buffer_name> starting from the position <start> (bytes count starts from zero).
    * {pull_out_slice_tmp} - creates a new buffer <new_buffer_name> with the exact size of <end> - <start> bytes and writes there the values from the buffer <buffer_name> starting with <start> and ending with <end>.
    * {merge_buffers_tmp} - creates a new buffer <new_buffer_name>, in which the first part of the bytes is copied from <left_buffer_name>, and the second part is copied from the buffer <right_buffer_name>. That is, the size of the new buffer is equal to the sum of the other two.
    * {open_file_tmp} - opens a file named <file_name> in the current directory strictly for reading.
    * {show_file_tmp} - reads a file named <file_name> starting with <start> and ending with <end>.
",
        PROJECT_NAME,
        help_tmp = "help [<command>]",
        exit_tmp = "exit",
        make_buffer_tmp = "make_buffer <buffer_name> <buffer_size>",
        fill_buffer_tmp = "fill_buffer <buffer_name> <value> <start> <end>",
        show_buffer_tmp = "show_buffer <buffer_name> <start> <end>",
        buffer_info_tmp = "buffer_info <buffer_name>",
        buffer_set_byte_tmp = "buffer_set_byte <buffer_name> <index> <value>",
        create_file_tmp = "create_file <file_name> <file_size>",
        from_file_to_buffer_tmp = "from_file_to_buffer <file_name> <buffer_name> <bytes_count> <file_start> <buffer_start>",
        from_buffer_to_file_tmp = "from_buffer_to_file <buffer_name> <file_name> <bytes_count> <buffer_start> <file_start>",
        buffer_write_bytes_tmp = "buffer_write_bytes <buffer_name> <start> <...bytes>",
        pull_out_slice_tmp = "pull_out_slice <buffer_name> <new_buffer_name> <start> <end>",
        merge_buffers_tmp = "merge_buffers <left_buffer_name> <right_buffer_name> <new_buffer_name>",
        open_file_tmp = "open_file <file_name>",
        show_file_tmp = "show_file <file_name> <start> <end>",
    )
}

/// The `help` function prints the help message. The function takes into account the fact that
/// runner can be opened either in terminal mode or in regular mode.
pub fn help<C: Commander>(runner: &mut Runner<C>) {
    if C::is_terminal() {
        runner.print(help_message_stylized());
    } else {
        runner.print(help_message_regular());
    }
}
