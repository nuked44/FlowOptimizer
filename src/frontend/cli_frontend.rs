use super::{Frontend, FrontendInput};
use std::{
    fmt::Display,
    io::{self, Write},
};
pub struct CliFrontend;

pub fn parse_input<'a>(_input: String) -> FrontendInput<'a> {
    todo!();
}

impl Frontend for CliFrontend {
    fn display_message(&self, message: impl Display) {
        print!("{}", message)
    }

    fn get_input(&self, prompt: &str) -> FrontendInput {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        parse_input(input.trim().to_string())
    }
}
