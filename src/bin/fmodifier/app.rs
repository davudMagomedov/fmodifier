#![allow(dead_code)]

use crate::clap_app::{build_app, *};

use clap::ArgMatches;

pub enum AppAction {
    RunInteractive,
    RunFromFile { file_name: String },
}

/// The `App` structure is responsible for parsing CLI arguments.
pub struct App {
    matches: ArgMatches,
}

impl App {
    pub fn new() -> App {
        let command = build_app();
        App::new_with_matches(command.get_matches())
    }

    /// The `action` function returns, surprisingly, action the app is launched for.
    pub fn action(&self) -> AppAction {
        match self.matches.subcommand() {
            Some((EXEC_FILE_SUBCMD, sub_matches)) => {
                let file_name: String = sub_matches
                    .get_one::<String>(FILE_NAME_ARG)
                    .expect("Couldn't find the file name argument")
                    .clone();

                AppAction::RunFromFile { file_name }
            }
            None => AppAction::RunInteractive,
            _ => unreachable!(),
        }
    }

    fn new_with_matches(matches: ArgMatches) -> App {
        App { matches }
    }
}
