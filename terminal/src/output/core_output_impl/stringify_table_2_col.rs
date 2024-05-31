fn stringify_couple(couple: &(String, String)) -> String {
    format!("{}\t\t{}", couple.0, couple.1)
}

fn stringify_couples(couples: &[(String, String)]) -> Vec<String> {
    couples
        .into_iter()
        .map(|couple| stringify_couple(couple))
        .collect()
}

/// The `stringify_table_2_col` function stringifies given table. There's no extra characters (new
/// line, space, etc) in the start and in the end of returning string (if there's no spaces in
/// table's elements).
pub fn stringify_table_2_col(table: &[(String, String)]) -> String {
    debug_assert!(table
        .into_iter()
        .all(|(k, v)| !k.contains('\n') && !v.contains('\n')));

    let mut string_table = "".to_string();

    let string_couples = stringify_couples(table);
    debug_assert_eq!(string_couples.len(), table.len());

    (0..table.len() - 1).for_each(|couple_index| {
        string_table.push_str(&string_couples[couple_index]);
        string_table.push('\n');
    });

    string_table.push_str(&string_couples[table.len() - 1]);

    todo!()
}
