use rand::prelude::SliceRandom;
use serde::Deserialize;
use serde_yaml::from_reader;
use std::fs::File;

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

    println!("{}", survivor.name);
    println!("-------");
    println!("  primary:   {}", primary);
    println!("  secondary: {}", secondary);
    println!("  utility:   {}", utility);
    println!("  special:   {}", special);
}
