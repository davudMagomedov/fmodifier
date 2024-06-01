use terminal::runner::Runner;

fn main() {
    let mut runner = Runner::new();
    loop {
        runner.next();
    }
}
