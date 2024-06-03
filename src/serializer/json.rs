use crate::{item::Item, machine::Machine, recipe::Recipe};

use super::{ResultItem, ResultMachine, ResultRecipe, Serializer};

pub struct Json;

impl Serializer for Json {
    fn serialize(
        &self,
        path: &str,
        items: Option<Vec<&Item>>,
        machines: Option<Vec<&Machine>>,
        recipes: Option<Vec<&Recipe>>,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn deserialize<'a>(
        &self,
        path: String,
    ) -> Result<(ResultItem<'a>, ResultMachine, ResultRecipe<'a>), std::io::Error> {
        todo!()
    }
}
