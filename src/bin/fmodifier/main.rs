mod app;
mod clap_app;
mod run;

fn main() {
    let application = app::App::new();
    run::run(&application);
}
