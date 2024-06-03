use std::io::Error;

use crate::{item::Item, machine::Machine, recipe::Recipe};

pub mod json;

pub type ResultItem<'a> = Option<Vec<Item<'a>>>;
pub type ResultMachine = Option<Vec<Machine>>;
pub type ResultRecipe<'a> = Option<Vec<Recipe<'a>>>;

pub trait Serializer {
    fn serialize(
        &self,
        path: &str,
        items: Option<Vec<&Item>>,
        machines: Option<Vec<&Machine>>,
        recipes: Option<Vec<&Recipe>>,
    ) -> Result<(), Error>;

    fn deserialize<'a>(
        &self,
        path: String,
    ) -> Result<(ResultItem<'a>, ResultMachine, ResultRecipe<'a>), Error>;
}
