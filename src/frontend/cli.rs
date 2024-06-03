use super::{Frontend, Input};
use std::{
    fmt::Display,
    io::{self, Write},
};
pub struct Cli;

fn parse_input<'a>(input: &str) -> Input<'a> {
    print!("{input}");
    todo!()
}

impl Frontend for Cli {
    fn display_message(&self, message: impl Display) {
        print!("{message}");
    }

    fn get_input(&self, prompt: &str) -> Input {
        print!("{prompt}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        parse_input(input.trim())
    }
}
