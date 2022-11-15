use self_avoiding_walk::config::Config;
use std::process;
use std::rc::Rc;

fn main() {
    let config = Rc::new(Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing YAML: {err}");
        process::exit(1);
    }));

    self_avoiding_walk::run(config);
}
