mod app;
mod clap_app;
mod run;

use terminal::commander::Terminal;
use terminal::runner::Runner;

fn main() {
    let commander = Terminal::new();
    let mut runner = Runner::new(commander);

    loop {
        runner.next();
    }
}
