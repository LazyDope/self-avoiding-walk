use std::process;
use self_avoiding_walk::config::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing YAML: {err}");
        process::exit(1);
    });

    self_avoiding_walk::run(&config);
}
