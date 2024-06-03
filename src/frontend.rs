use core::fmt;

use crate::{item::Item, machine::Machine, recipe::Recipe};

pub mod cli;

pub enum Input<'a> {
    Object(Item<'a>),
    Machine(Machine),
    Recipe(Recipe<'a>),
}

pub trait Frontend {
    fn display_message(&self, message: impl fmt::Display);
    fn get_input(&self, prompt: &str) -> Input;
}
