use super::Frontend;
use std::{fmt::Display, io::{self, Write}};
pub struct CliFrontend;

impl Frontend for CliFrontend {
    fn display_message(&self, message: impl Display) {
        print!("{}", message)
    }

    fn get_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}