use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

pub struct Config {
    pub savepath: String,
}

impl Config {
    fn new() -> Config {
        Config {
            savepath: String::new(),
        }
    }

    pub fn read_from_file(path: String) -> Result<Config, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut config = Config::new();

        for (linenumber, line) in reader.lines().enumerate() {
            let line = line?;

            let parts: Vec<&str> = line.split('=').map(str::trim).collect();
            assert!(
                parts.len() == 2,
                "Invalid config file format at line number: {linenumber}"
            );

            match parts.first() {
                Some(setting) => match *setting {
                    "savepath" => config.savepath = parts[1].to_string(),
                    _ => panic!("Unknown parameter: {}", parts[0]),
                },
                None => unreachable!(),
            }
        }

        Ok(config)
    }
}
