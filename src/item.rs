use core::fmt;

use crate::recipe::{IngredientMachine, Recipe};

pub struct Item<'a> {
    pub id: usize,
    pub name: String,
    pub recipes: Vec<&'a Recipe<'a>>,
}

impl<'a> Item<'a> {
    pub fn new(id: usize, name: String, recipes: Vec<&'a Recipe<'a>>) -> Item<'a> {
        Item { id, name, recipes }
    }
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}(Id: {}):", self.name, self.id)?;
        for (count, recipe) in self.recipes.iter().enumerate() {
            writeln!(
                f,
                "Recipe {} for: {}x {}",
                count + 1,
                recipe.quantity,
                self.name
            )?;
            match &recipe.ingredients {
                IngredientMachine::Some(recipe, machine) => {
                    for (ingredient, quantity) in recipe {
                        writeln!(
                            f,
                            "{}x {}(Id: {})",
                            quantity, ingredient.name, ingredient.id
                        )?;
                    }
                    writeln!(
                        f,
                        "in {}(Id: {}) and it takes {}time units\n",
                        machine.name,
                        machine.id,
                        1f64 / machine.throughput_per_tu
                    )?;
                }
                IngredientMachine::None => writeln!(f, "There is no recipe\n")?,
            }
        }
        write!(f, "")
    }
}
