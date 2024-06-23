use crate::app::{App, AppAction, Pair};

use fmodifier::core::CoreCommand;
use fmodifier::terminal::commander::{Commander, FileReader, Terminal};
use fmodifier::terminal::runner::Runner;

const FILE_UNEXISTS_ERROR: &str = "A file with the name isn't found.";

fn insert_variables_to_runner<C: Commander>(app: &App, runner: &mut Runner<C>) {
    app.variables().into_iter().for_each(|pair| match pair {
        Pair::IntegerVar { name, value } => {
            let _ = runner.execute(CoreCommand::SetVariableInteger {
                variable_name: name,
                value,
            });
        }
        Pair::StringVar { name, value } => {
            let _ = runner.execute(CoreCommand::SetVariableString {
                variable_name: name,
                value,
            });
        }
    });
}

fn run_interactive(app: &App) {
    let terminal = Terminal::new();
    let mut runner = Runner::new(terminal);

    insert_variables_to_runner(app, &mut runner);

    runner.for_each(|_| {});
}

fn run_from_file(app: &App, file_path: &str) {
    let file_reader = FileReader::new(file_path).expect(FILE_UNEXISTS_ERROR);
    let mut runner = Runner::new(file_reader);

    insert_variables_to_runner(app, &mut runner);

    runner.for_each(|_| {});
}

/// The `run` function takes an application (`App`) and launch the program.
pub fn run(app: &App) {
    match app.action() {
        AppAction::RunInteractive => {
            run_interactive(app);
        }
        AppAction::RunFromFile { file_name } => run_from_file(app, &file_name),
    }
}
