#![allow(dead_code)]

use frontend::{cli_frontend::CliFrontend, Frontend};
use id_generator::IdGenerator;
use machine::Machine;
use object::Object;
use recipe::Recipe;

mod frontend;
mod id_generator;
mod machine;
mod object;
mod recipe;

fn main() {
    let machine_id_generator = IdGenerator::new();
    let object_id_generator = IdGenerator::new();
    let recipe_id_generator = IdGenerator::new();

    let furnace = Machine::new(
        machine_id_generator.generate_id(),
        "furnace".to_string(),
        2f64,
    );
    let blast_furnace = Machine::new(
        machine_id_generator.generate_id(),
        "blastfurnace".to_string(),
        20f64,
    );

    let coal = Object::new(
        object_id_generator.generate_id(),
        "coal".to_string(),
        Recipe::no_recipe(),
    );
    let iron_ore = Object::new(
        object_id_generator.generate_id(),
        "iron ore".to_string(),
        Recipe::no_recipe(),
    );

    let iron_recipe_list_furnace = vec![(&coal, 2), (&iron_ore, 5)];
    let iron_recipe_list_blast_furnace = vec![(&coal, 2), (&iron_ore, 10)];

    let iron = Object::new(
        object_id_generator.generate_id(),
        "iron".to_string(),
        vec![
            Recipe::new_recipe(
                recipe_id_generator.generate_id(),
                5,
                iron_recipe_list_furnace,
                furnace,
            ),
            Recipe::new_recipe(
                recipe_id_generator.generate_id(),
                10,
                iron_recipe_list_blast_furnace,
                blast_furnace,
            ),
        ],
    );

    let cli_frontend = CliFrontend;
    cli_frontend.display_message(iron);
}
