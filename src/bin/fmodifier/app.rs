#![allow(dead_code)]

use crate::clap_app::{build_app, *};

use clap::ArgMatches;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppAction {
    RunInteractive,
    RunFromFile { file_name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pair {
    StringVar { name: String, value: String },
    IntegerVar { name: String, value: usize },
}

/// The `App` structure is responsible for parsing CLI arguments.
pub struct App {
    matches: ArgMatches,
}

fn is_right_name(string: &str) -> bool {
    let mut chars = string.chars();

    chars
        .next()
        .map(|ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '_'))
        .unwrap_or(false)
        && chars.all(|ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'))
}

fn parse_assignment_string(string: &str) -> Option<Pair> {
    let equal_pos = string.find('=')?;

    let name = string[..equal_pos].trim();
    let right_value = string[equal_pos + 1..].trim();

    match right_value.parse::<usize>() {
        Ok(value) => Some(Pair::IntegerVar {
            name: name.to_string(),
            value,
        }),
        Err(_) => {
            if is_right_name(right_value) {
                Some(Pair::StringVar {
                    name: name.to_string(),
                    value: right_value.to_string(),
                })
            } else {
                return None;
            }
        }
    }
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

    /// The `variables` function returns all variables that were defined in CLI.
    pub fn variables(&self) -> Vec<Pair> {
        let mut unparsed = Vec::new();

        let pairs: Vec<Pair> = self
            .matches
            .get_many::<String>(VARIABLE_ARG)
            .unwrap_or_default()
            .filter_map(|string| match parse_assignment_string(string) {
                Some(pair) => Some(pair),
                None => {
                    unparsed.push(string);
                    None
                }
            })
            .collect();

        if !unparsed.is_empty() {
            println!("The program couldn't parse the following assignments:");
            unparsed.iter().for_each(|&string| {
                println!("  {}", string);
            });
        }

        pairs
    }

    fn new_with_matches(matches: ArgMatches) -> App {
        App { matches }
    }
}
