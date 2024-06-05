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
            id: item.id.clone(),
            name: item.name.clone(),
            recipe_ids: item.recipes.iter().map(|r| r.id.clone()).collect(),
        }
    }
}

pub struct SerializableMachine {
    pub id: usize,
    pub name: String,
    pub throughput_per_tu: f64,
}

impl From<Option<&Machine>> for SerializableMachine {
    fn from(machine: Option<&Machine>) -> SerializableMachine {
        if let Some(machine) = machine {
            return SerializableMachine {
                id: machine.id.clone(),
                name: machine.name.clone(),
                throughput_per_tu: machine.throughput_per_tu.clone(),
            };
        }
        SerializableMachine {
            id: 0,
            name: "Epmty Machine".to_string(),
            throughput_per_tu: 0f64,
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
            id: recipe.id.clone(),
            quantity: recipe.quantity.clone(),
            ingredient_ids_and_quantity: match &recipe.ingredients {
                IngredientMachine::Some(ingredients, _) => {
                    ingredients.iter().map(|item| (item.0.id.clone(), item.1.clone())).collect()
                }
                IngredientMachine::None => Vec::new(),
            },
            machine_ids: match &recipe.ingredients {
                IngredientMachine::Some(_, machine) => {
                    if let Some(machine) = machine {
                        machine.id.clone()
                    } else {
                        0
                    }
                }
                IngredientMachine::None => 0,
            },
        }
    }
}
