use crate::recipe::{IngredientMachine, Recipe};

pub struct Object<'a> {
    pub name: String,
    pub recipes: Vec<Recipe<'a>>,
}

impl<'a> Object<'a> {
    pub fn new(name: String, recipes: Vec<Recipe<'a>>) -> Object<'a> {
        Object { name, recipes }
    }

    pub fn print_recipe(&self) {
        for (count, recipe) in self.recipes.iter().enumerate() {
            let counter;
            if self.recipes.len() <= 1 {
                counter = "".to_string();
            } else {
                counter = count.to_string() + " ";
            };
            println!("Recipe for: {}{}x {}", counter, recipe.quantity, self.name);
            match &recipe.ingredients {
                IngredientMachine::Some(recipe, machine) => {
                    recipe.iter().for_each(|(ingredient, quantity)| {
                        println!("{}x {}", quantity, ingredient.name);
                    });
                    println!(
                        "in {} and it takes {}time units",
                        machine.name, 1f64/machine.throughput_per_min
                    )
                }
                IngredientMachine::None => {
                    println!("There is no recipe")
                }
            }
        }
    }
}
