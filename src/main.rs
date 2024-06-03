#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::Error;

use config::Config;
use frontend::{cli::Cli, Frontend};
use id_generator::IdGenerator;
use item::Item;
use machine::Machine;
use recipe::Recipe;
use serializer::{json::Json, Serializer};

mod config;
mod frontend;
mod id_generator;
mod item;
mod machine;
mod recipe;
mod serializer;

fn main() -> Result<(), Error> {
    let config = Config::read_from_file("src/.project_config/config.cfg".to_string())?;

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

    let coal = Item::new(
        object_id_generator.generate_id(),
        "coal".to_string(),
        Recipe::no_recipe(),
    );
    let iron_ore = Item::new(
        object_id_generator.generate_id(),
        "iron ore".to_string(),
        Recipe::no_recipe(),
    );

    let iron_recipe_furnace = Recipe::new_recipe(
        recipe_id_generator.generate_id(),
        5,
        vec![(&coal, 2), (&iron_ore, 5)],
        &furnace,
    );

    let iron_recipe_blast_furnace = Recipe::new_recipe(
        recipe_id_generator.generate_id(),
        10,
        vec![(&coal, 2), (&iron_ore, 10)],
        &blast_furnace,
    );

    let iron = Item::new(
        object_id_generator.generate_id(),
        "iron".to_string(),
        vec![&iron_recipe_furnace, &iron_recipe_blast_furnace],
    );

    let cli_frontend = Cli;
    cli_frontend.display_message(&iron);

    let json_serializer = Json;
    json_serializer.serialize(
        &config.savepath,
        Some(vec![&coal, &iron_ore, &iron]),
        Some(vec![&furnace, &blast_furnace]),
        Some(vec![&iron_recipe_furnace, &iron_recipe_blast_furnace]),
    )?;

    println!("{}", config.savepath);

    Ok(())
}
