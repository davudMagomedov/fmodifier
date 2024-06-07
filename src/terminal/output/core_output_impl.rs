use super::ToOutput;

use crate::core::output::*;

mod string_rectangle;
mod stringify_table;
mod stringify_table_2_col;

use stringify_table::stringify_table;
use stringify_table_2_col::stringify_table_2_col;

const TABLE_CAPTION: &str = "Table:";
const TABLE_2COL_CAPTION: &str = "Table:";

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
            write_to.push_str(TABLE_CAPTION);
            write_to.push('\n');

            stringify_table_2_col(data, write_to);
        }
        OtherInfo::BigTable { table } => {
            write_to.push_str(TABLE_2COL_CAPTION);
            write_to.push('\n');

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

fn stringify_warning(warning: &Warning, write_to: &mut String) {
    debug_assert!(!warning.contains('\n'));

    write_to.push_str("Warning: ");
    write_to.push_str(warning);
}

/// The `stringify_warning` function writes to given string all all warnings. There's no extra
/// characters in the end and in the start.
fn stringify_warnings(warnings: &[Warning], write_to: &mut String) {
    if warnings.is_empty() {
        return;
    }

    warnings[..warnings.len() - 1]
        .into_iter()
        .for_each(|other_info| {
            stringify_warning(other_info, write_to);
            write_to.push('\n');
        });

    stringify_warning(&warnings[warnings.len() - 1], write_to);
}

impl ToOutput for CoreOutput {
    fn to_output(self) -> String {
        let mut output = String::new();

        let infos = self.info();
        if !infos.is_empty() {
            stringify_infos(self.info(), &mut output);
            output.push_str("\n");
        }

        let other_infos = self.other_info();
        if !other_infos.is_empty() {
            stringify_other_infos(other_infos, &mut output);
            output.push_str("\n");
        }

        let warnings = self.warnings();
        if !warnings.is_empty() {
            stringify_warnings(self.warnings(), &mut output);
            output.push_str("\n");
        }

        output
    }
}
