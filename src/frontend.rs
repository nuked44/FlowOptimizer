use core::fmt;

pub mod cli_frontend;

pub trait Frontend {
    fn display_message(&self, message: impl fmt::Display);
    fn get_input(&self, prompt: &str) -> String;
}