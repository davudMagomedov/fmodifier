mod app;
mod clap_app;
mod run;

use terminal::commander::Terminal;
use terminal::runner::Runner;

fn main() {
    let application = app::App::new();
    run::run(&application);
}
