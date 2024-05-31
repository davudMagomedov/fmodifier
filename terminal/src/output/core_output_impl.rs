use super::ToOutput;

use core::output::*;

mod stringify_table;
mod stringify_table_2_col;

use stringify_table::stringify_table;
use stringify_table_2_col::stringify_table_2_col;

fn stringify_info_line(info_line: &InfoLine, write_to: &mut String) {
    write_to.push_str("- ");
    write_to.push_str(info_line);
}

/// The `stringify_infos` function writes to given string all given information lines. There's no
/// extra characters in the end and in the start of function.
///
/// For this function is right used, push to string the `\n` character before calling or make sure
/// that the string this function writes to is on empty line.
fn stringify_infos(infos: &[InfoLine], write_to: &mut String) {
    if infos.is_empty() {
        return;
    }

    infos[..infos.len() - 1].into_iter().for_each(|info_line| {
        stringify_info_line(info_line, write_to);
        write_to.push('\n');
    });

    stringify_info_line(&infos[infos.len() - 1], write_to);
}

fn stringify_other_info(other_info: &OtherInfo, write_to: &mut String) {
    match other_info {
        OtherInfo::Table2Column { data } => {
            stringify_table_2_col(data, write_to);
        }
        OtherInfo::BigTable { table } => {
            stringify_table(table, write_to);
        }
    }
}

/// The `stringify_other_infos` function writes the all given other informations into the given
/// string. There's no extra characters (new line, space and so on) in the end and in the start.
fn stringify_other_infos(other_infos: &[OtherInfo], write_to: &mut String) {
    if other_infos.is_empty() {
        return;
    }

    other_infos[..other_infos.len() - 1]
        .into_iter()
        .for_each(|other_info| {
            stringify_other_info(other_info, write_to);
            write_to.push('\n');
        });

    stringify_other_info(&other_infos[other_infos.len() - 1], write_to);
}
