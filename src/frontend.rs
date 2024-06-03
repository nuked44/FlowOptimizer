use core::fmt;

use crate::{machine::Machine, object::Object, recipe::Recipe};

pub mod cli_frontend;

pub enum FrontendInput<'a> {
    Object(Object<'a>),
    Machine(Machine),
    Recipe(Recipe<'a>),
}

pub trait Frontend {
    fn display_message(&self, message: impl fmt::Display);
    fn get_input(&self, prompt: &str) -> FrontendInput;
}
