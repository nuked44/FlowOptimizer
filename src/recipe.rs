use crate::{item::Item, machine::Machine};

pub enum IngredientMachine<'a> {
    Some(Vec<(&'a Item<'a>, usize)>, &'a Machine),
    None,
}

pub struct Recipe<'a> {
    pub id: usize,
    pub quantity: usize,
    pub ingredients: IngredientMachine<'a>,
}

impl<'a> Recipe<'a> {
    pub fn no_recipe() -> Vec<&'a Recipe<'a>> {
        vec![&Recipe {
            id: 0,
            quantity: 0,
            ingredients: IngredientMachine::None,
        }]
    }

    pub fn new_recipe(
        id: usize,
        quantity: usize,
        ingredients: Vec<(&'a Item, usize)>,
        machine: &'a Machine,
    ) -> Recipe<'a> {
        Recipe {
            id,
            quantity,
            ingredients: IngredientMachine::Some(ingredients, machine),
        }
    }
}
