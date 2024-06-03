#![allow(dead_code)]

use id_generator::IdGenerator;
use machine::Machine;
use object::Object;
use recipe::Recipe;

mod machine;
mod object;
mod recipe;
mod id_generator;

fn main() {
    let furnace = Machine::new("furnace".to_string(), 2f64);
    let blast_furnace = Machine::new("blastfurnace".to_string(), 20f64);

    let coal = Object::new("coal".to_string(), Recipe::no_recipe());
    let iron_ore = Object::new("iron ore".to_string(), Recipe::no_recipe());

    let iron_recipe_list_furnace = vec![(&coal, 2), (&iron_ore, 5)];
    let iron_recipe_list_blast_furnace = vec![(&coal, 2), (&iron_ore, 10)];

    let iron = Object::new(
        "iron".to_string(),
        vec![
            Recipe::new_recipe(5, iron_recipe_list_furnace, furnace),
            Recipe::new_recipe(10, iron_recipe_list_blast_furnace, blast_furnace),
        ],
    );

    iron.print_recipe();
}
