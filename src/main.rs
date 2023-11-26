use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, style::Stylize};
use rand::prelude::SliceRandom;
use serde::Deserialize;
use std::{fs::File, io::stdout};

#[derive(Deserialize)]
struct Survivor {
    name: String,
    primary: Vec<String>,
    secondary: Vec<String>,
    utility: Vec<String>,
    special: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    let file = File::open("survivors.yml").unwrap();
    let list: Vec<Survivor> = serde_yaml::from_reader(file).unwrap();

    loop {
        let survivor = list.choose(&mut rng).unwrap();
        let primary = survivor.primary.choose(&mut rng).unwrap();
        let secondary = survivor.secondary.choose(&mut rng).unwrap();
        let utility = survivor.utility.choose(&mut rng).unwrap();
        let special = survivor.special.choose(&mut rng).unwrap();

        execute!(stdout(), Clear(ClearType::All))?;

        println!(
            "{}     {}",
            "Survivor:".blue(),
            survivor.name.to_string().bold().red(),
        );
        println!("{}", "-------".dark_grey());
        println!("  {}    {}", "primary:".blue(), primary.to_string().red());
        println!("  {}  {}", "secondary:".blue(), secondary.to_string().red());
        println!("  {}    {}", "utility:".blue(), utility.to_string().red());
        println!("  {}    {}", "special:".blue(), special.to_string().red());
        println!("{}", "-------".dark_grey());
        println!();
        println!("{}", "press any key for a new loadout");
        println!("{}", "press esc to exit");

        enable_raw_mode()?;
        match crossterm::event::read().unwrap() {
            Event::Key(event) => {
                if [KeyCode::Esc, KeyCode::Char('q')].contains(&event.code) {
                    break;
                }

                if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
                    break;
                }
            }
            _ => {}
        }
        disable_raw_mode()?;
    }
    disable_raw_mode()?;

    return Ok(());
}
