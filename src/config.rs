use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use yaml_rust::yaml::Hash;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

pub struct Config {
    size: usize,
    min_length: i32,
}

const DEFAULT_SIZE: usize = 10;
const DEFAULT_LENGTH: i32 = 10;

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let config_file = match File::open("walk-config.yaml") {
            Ok(fc) => Ok(fc),
            Err(err) if err.kind() == ErrorKind::NotFound => {
                let mut fc = File::create("walk-config.yaml")?;
                let mut out_str = String::new();
                let mut emitter = YamlEmitter::new(&mut out_str);
                emitter.dump(
                    &(Config {
                        size: DEFAULT_SIZE,
                        min_length: DEFAULT_LENGTH,
                    }
                    .into()),
                )?;
                fc.write_all(&out_str.as_bytes())?;
                Ok(fc)
            }
            Err(err) => Err(err),
        }?;
        let mut out_str = String::new();
        config_file.read_to_string(&mut out_str)?;
        Ok(YamlLoader::load_from_str(&out_str)?.into())
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn min_length(&self) -> i32 {
        self.min_length
    }
}

impl Into<Yaml> for Config {
    fn into(self) -> Yaml {
        let mut mapping = Hash::new();
        mapping.insert(
            Yaml::String(String::from("size")),
            Yaml::Integer(self.size as i64),
        );
        mapping.insert(
            Yaml::String(String::from("min_length")),
            Yaml::Integer(self.min_length as i64),
        );
        Yaml::Hash(mapping)
    }
}

impl From<Vec<Yaml>> for Config {
    fn from(yaml: Vec<Yaml>) -> Config {
        let loaded = yaml[0].as_hash().unwrap();
        let mut size = DEFAULT_SIZE;
        let mut min_length = DEFAULT_LENGTH;

        if let Yaml::Integer(num) = loaded[&Yaml::String(String::from("size"))] {
            size = num as usize;
        }

        if let Yaml::Integer(num) = loaded[&Yaml::String(String::from("min_length"))] {
            min_length = num as i32;
        }
        Config { size, min_length }
    }
}
