use crossterm::{
    execute,
    style::Stylize,
    terminal::{Clear, ClearType},
};
use rand::prelude::SliceRandom;
use serde::Deserialize;
use serde_yaml::from_reader;
use std::{fs::File, io::stdout};

#[derive(Deserialize)]
struct Survivor {
    name: String,
    primary: Vec<String>,
    secondary: Vec<String>,
    utility: Vec<String>,
    special: Vec<String>,
}

fn main() {
    let mut rng = rand::thread_rng();

    let file = File::open("survivors.yml").unwrap();
    let list: Vec<Survivor> = from_reader(file).unwrap();

    let survivor = list.choose(&mut rng).unwrap();
    let primary = survivor.primary.choose(&mut rng).unwrap();
    let secondary = survivor.secondary.choose(&mut rng).unwrap();
    let utility = survivor.utility.choose(&mut rng).unwrap();
    let special = survivor.special.choose(&mut rng).unwrap();

    execute!(stdout(), Clear(ClearType::All)).ok();

    println!(
        "{} {}",
        "Survivor:".blue(),
        survivor.name.to_string().bold().red()
    );
    println!("{}", "-------".dark_grey());
    println!("  {}    {}", "primary:".blue(), primary.to_string().red());
    println!("  {}  {}", "secondary:".blue(), secondary.to_string().red());
    println!("  {}    {}", "utility:".blue(), utility.to_string().red());
    println!("  {}    {}", "special:".blue(), special.to_string().red());
    println!("{}", "-------".dark_grey());
}
