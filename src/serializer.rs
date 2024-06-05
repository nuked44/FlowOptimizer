use std::io::Error;

use crate::{
    item::Item,
    machine::Machine,
    recipe::{IngredientMachine, Recipe},
};

pub mod json;

pub type ResultItem<'a> = Option<Vec<Item<'a>>>;
pub type ResultMachine = Option<Vec<Machine>>;
pub type ResultRecipe<'a> = Option<Vec<Recipe<'a>>>;

pub trait Serializer {
    fn serialize(
        &self,
        path: &str,
        items: Option<Vec<SerializableItem>>,
        machines: Option<Vec<Option<SerializableMachine>>>,
        recipes: Option<Vec<SerializableRecipe>>,
    ) -> Result<(), Error>;

    fn deserialize<'a>(
        &self,
        path: String,
    ) -> Result<(ResultItem<'a>, ResultMachine, ResultRecipe<'a>), Error>;
}

pub struct SerializableItem {
    pub id: usize,
    pub name: String,
    pub recipe_ids: Vec<usize>,
}

impl From<&Item<'_>> for SerializableItem {
    fn from(item: &Item<'_>) -> SerializableItem {
        SerializableItem {
            id: item.id,
            name: item.name.clone(),
            recipe_ids: item.recipes.iter().map(|r| r.id).collect(),
        }
    }
}

pub struct SerializableMachine {
    pub id: usize,
    pub name: String,
    pub throughput_per_tu: f64,
}

impl From<&Machine> for SerializableMachine {
    fn from(machine: &Machine) -> SerializableMachine {
        SerializableMachine {
            id: machine.id,
            name: machine.name.clone(),
            throughput_per_tu: machine.throughput_per_tu,
        }
    }
}

pub struct SerializableRecipe {
    pub id: usize,
    pub quantity: usize,
    pub ingredient_ids_and_quantity: Vec<(usize, usize)>,
    pub machine_ids: usize,
}

impl From<&Recipe<'_>> for SerializableRecipe {
    fn from(recipe: &Recipe) -> SerializableRecipe {
        SerializableRecipe {
            id: recipe.id,
            quantity: recipe.quantity,
            ingredient_ids_and_quantity: match &recipe.ingredients {
                IngredientMachine::Some(ingredients, _) => ingredients
                    .iter()
                    .map(|item| (item.0.id, item.1))
                    .collect(),
                IngredientMachine::None => Vec::new(),
            },
            machine_ids: match &recipe.ingredients {
                IngredientMachine::Some(_, machine) => machine.id,
                IngredientMachine::None => 0,
            },
        }
    }
}
