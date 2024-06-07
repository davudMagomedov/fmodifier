use clap::{crate_authors, crate_name, crate_version, Arg, Command};

pub const FILE_NAME_ARG: &str = "file_name";
pub const EXEC_FILE_SUBCMD: &str = "execfile";

/// The `build_app` function returns specially built `clap::Command` object.
pub fn build_app() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand_required(false)
        .subcommand(
            Command::new(EXEC_FILE_SUBCMD).arg(
                Arg::new(FILE_NAME_ARG)
                    .required(false)
                    .help("For executing a file"),
            ),
        )
}
