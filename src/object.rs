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
        writeln!(f, "{}(Id: {}):", self.name, self.id).unwrap();
        for (count, recipe) in self.recipes.iter().enumerate() {
            writeln!(
                f,
                "Recipe {} for: {}x {}",
                count + 1,
                recipe.quantity,
                self.name
            )
            .unwrap();
            match &recipe.ingredients {
                IngredientMachine::Some(recipe, machine) => {
                    recipe.iter().for_each(|(ingredient, quantity)| {
                        writeln!(f, "{}x {}(Id: {})", quantity, ingredient.name, ingredient.id).unwrap();
                    });
                    writeln!(
                        f,
                        "in {}(Id: {}) and it takes {}time units\n",
                        machine.name,
                        machine.id,
                        1f64 / machine.throughput_per_min
                    )
                    .unwrap()
                }
                IngredientMachine::None => writeln!(f, "There is no recipe\n").unwrap(),
            }
        }
        write!(f, "")
    }
}
