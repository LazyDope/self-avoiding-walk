use std::collections::HashMap;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::fs::File;
use std::io::{ErrorKind, Write};

pub struct Config {
    size: usize,
    min_length: i32,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let config_file = File::open("walk-config.yaml").unwrap_or_else(|err| match err.kind() {
            ErrorKind::NotFound => match File::create("walk-config.yaml") {
                Ok(mut fc) => {
                    let mut out_str = String::new();
                    let mut emitter = YamlEmitter::new(&mut out_str);
                    emitter.dump()
                    fc.write();
                    fc
                },
                Err(e) => e?
            },
            _ => {
                err?
            }
        });
        YamlLoader::
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn min_length(&self) -> i32 {
        self.min_length
    }
}
