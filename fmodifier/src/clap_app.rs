use clap::{crate_authors, crate_name, crate_version, Arg, ArgAction, Command};

pub fn build_app() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::new("executable_file_name")
                .required(false)
                .help("For executing a file"),
        )
}
