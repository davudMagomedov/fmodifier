use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgAction, Command};

pub const FILE_NAME_ARG: &str = "file_name";
pub const EXEC_FILE_SUBCMD: &str = "execfile";
pub const VARIABLE_ARG: &str = "variable";

/// The `build_app` function returns specially built `clap::Command` object.
pub fn build_app() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand_required(false)
        .arg(
            Arg::new(VARIABLE_ARG)
                .required(false)
                .long(VARIABLE_ARG)
                .short('v')
                .action(ArgAction::Append),
        )
        .subcommand(
            Command::new(EXEC_FILE_SUBCMD).arg(
                Arg::new(FILE_NAME_ARG)
                    .required(true)
                    .help("For executing a file"),
            ),
        )
}
