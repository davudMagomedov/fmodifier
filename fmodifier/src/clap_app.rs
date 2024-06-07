use clap::{crate_authors, crate_name, crate_version, Arg, Command};

const FILE_NAME_ARG: &str = "file_name";

/// The `build_app` function returns specially built `clap::Command` object.
pub fn build_app() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::new(FILE_NAME_ARG)
                .required(false)
                .help("For executing a file"),
        )
}
