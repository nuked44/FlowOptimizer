#![allow(dead_code)]

use machine::Machine;
use object::Object;
use recipe::Recipe;

mod object;
mod recipe;
mod machine;

fn main() {
    let iron = Object::new("iron".to_string());
    let coal = Object::new("coal".to_string());
    let iron_ore = Object::new("iron ore".to_string());

    let furnace = Machine::new("furnace".to_string(), 2f64);   

    let iron_recipe_list = vec![(&coal, 2), (&iron_ore, 5)];
    let iron_recipe = Recipe::new_with_recipe(&iron, 5, iron_recipe_list, furnace);

    iron_recipe.print_recipe();
}
