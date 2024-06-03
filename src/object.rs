use core::fmt;

use crate::recipe::{IngredientMachine, Recipe};

pub struct Object<'a> {
    pub id: usize,
    pub name: String,
    pub recipes: Vec<Recipe<'a>>,
}

impl<'a> Object<'a> {
    pub fn new(id: usize, name: String, recipes: Vec<Recipe<'a>>) -> Object<'a> {
        Object { id, name, recipes }
    }
}

impl<'a> fmt::Display for Object<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (count, recipe) in self.recipes.iter().enumerate() {
            println!("Recipe {} for: {}x {}", count, recipe.quantity, self.name);
            match &recipe.ingredients {
                IngredientMachine::Some(recipe, machine) => {
                    recipe.iter().for_each(|(ingredient, quantity)| {
                        println!("{}x {}", quantity, ingredient.name);
                    });
                    println!(
                        "in {} and it takes {}time units",
                        machine.name,
                        1f64 / machine.throughput_per_min
                    )
                }
                IngredientMachine::None => {
                    println!("There is no recipe")
                }
            }
        };
        write!(f, "")
    }
}