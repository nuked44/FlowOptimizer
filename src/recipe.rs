use crate::{machine::Machine, object::Object};

pub enum IngredientMachine<'a> {
    Some(Vec<(&'a Object<'a>, usize)>, Machine),
    None,
}

pub struct Recipe<'a> {
    pub quantity: usize,
    pub ingredients: IngredientMachine<'a>,
}

impl<'a> Recipe<'a> {
    pub fn no_recipe() -> Vec<Recipe<'a>> {
        vec![Recipe {
            quantity: 0,
            ingredients: IngredientMachine::None,
        }]
    }

    pub fn new_recipe(
        quantity: usize,
        ingredients: Vec<(&'a Object, usize)>,
        machine: Machine,
    ) -> Recipe<'a> {
        Recipe {
            quantity,
            ingredients: IngredientMachine::Some(ingredients, machine),
        }
    }
}
