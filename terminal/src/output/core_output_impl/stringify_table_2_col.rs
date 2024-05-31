fn stringify_couple(couple: &(String, String), write_to: &mut String) {
    write_to.push_str(&couple.0);
    write_to.push_str("\t\t");
    write_to.push_str(&couple.1);
}

fn stringify_couples(couples: &[(String, String)], write_to: &mut String) {
    if couples.is_empty() {
        return;
    }

    couples[0..couples.len() - 1]
        .into_iter()
        .for_each(|couple| {
            stringify_couple(couple, write_to);
            write_to.push('\n');
        });

    stringify_couple(&couples[couples.len() - 1], write_to);
}

/// The `stringify_table_2_col` function writes to given string stringified given table. There's no
/// extra chars in the end and in the start.
pub fn stringify_table_2_col(table: &[(String, String)], write_to: &mut String) {
    debug_assert!(table
        .into_iter()
        .all(|(k, v)| !k.contains('\n') && !v.contains('\n')));

    stringify_couples(table, write_to);
}
