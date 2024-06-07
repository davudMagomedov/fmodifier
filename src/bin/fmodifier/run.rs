use crate::app::{App, AppAction};

use fmodifier::terminal::commander::{FileReader, Terminal};
use fmodifier::terminal::runner::Runner;

const FILE_UNEXISTS_ERROR: &str = "A file with the name isn't found.";

fn run_interactive() {
    let terminal = Terminal::new();
    let runner = Runner::new(terminal);

    runner.for_each(|_| {});
}

fn run_from_file(file_path: &str) {
    let file_reader = FileReader::new(file_path).expect(FILE_UNEXISTS_ERROR);
    let runner = Runner::new(file_reader);

    runner.for_each(|_| {});
}

/// The `run` function takes an application (`App`) and launch the program.
pub fn run(app: &App) {
    match app.action() {
        AppAction::RunInteractive => {
            run_interactive();
        }
        AppAction::RunFromFile { file_name } => run_from_file(&file_name),
    }
}
