use std::rc::Rc;

use crate::{
    machine::Machine,
    object::Object,
};

enum RecipeMachine<'a> {
    Some(Vec<(&'a Object, usize)>, Machine),
    None,
}

pub struct Recipe<'a> {
    product: &'a Object,
    quantity: usize,
    recipe: RecipeMachine<'a>,
}

impl<'a> Recipe<'a> {
    pub fn new(product: &'a Object, quantity: usize) -> Rc<Recipe> {
        Rc::new(Recipe {
            product,
            quantity,
            recipe: RecipeMachine::None,
        })
    }

    pub fn new_with_recipe(
        product: &'a Object,
        quantity: usize,
        recipe: Vec<(&'a Object, usize)>,
        machine: Machine,
    ) -> Recipe<'a> {
        Recipe {
            product,
            quantity,
            recipe: RecipeMachine::Some(recipe, machine),
        }
    }

    pub fn add_recipe(&mut self, ingredients: Vec<(&'a Object, usize)>, machine: Machine) {
        self.recipe = RecipeMachine::Some(ingredients, machine);
    }

    pub fn print_recipe(&self) {
        println!("Recipe for: {}x {}", self.quantity, self.product.name);
        match &self.recipe {
            RecipeMachine::Some(recipe, machine) => {
                recipe.iter().for_each(|(ingredient, quantity)| {
                    println!("{}x {}", quantity, ingredient.name);
                });
                println!(
                    "in {} and it takes {}time units",
                    machine.name, machine.throughput
                )
            }
            RecipeMachine::None => {
                println!("There is no recipe")
            }
        }
    }
}
