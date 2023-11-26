use crossterm::event::{Event, KeyCode, KeyModifiers, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor::MoveToPreviousLine, execute, style::Stylize};
use rand::prelude::SliceRandom;
use serde::Deserialize;
use std::io::{stdout, Write};
use std::{fs::File, time::SystemTime};

#[derive(Deserialize)]
struct Survivor {
    name: String,
    enabled: bool,
    primary: Vec<String>,
    secondary: Vec<String>,
    utility: Vec<String>,
    special: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    let file = File::open("survivors.yml").expect("could not find survivors.yml");
    let list: Vec<Survivor> = serde_yaml::from_reader(file).expect("unable to parse yaml file");

    execute!(stdout(), Clear(ClearType::All))?;

    loop {
        let survivor = loop {
            let chosen = list.choose(&mut rng).unwrap();
            if chosen.enabled {
                break chosen;
            }
        };

        let primary = survivor.primary.choose(&mut rng).unwrap();
        let secondary = survivor.secondary.choose(&mut rng).unwrap();
        let utility = survivor.utility.choose(&mut rng).unwrap();
        let special = survivor.special.choose(&mut rng).unwrap();

        println!();
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
        println!();
        println!("{}", "-------".dark_grey());
        println!("press any key for a new loadout");
        println!("press esc to exit");

        if read_input(SystemTime::now()).unwrap() {
            break;
        }

        execute!(
            stdout(),
            MoveToPreviousLine(3),
            Clear(ClearType::FromCursorDown),
        )?;
    }

    return Ok(());
}

fn read_input(now: SystemTime) -> std::io::Result<bool> {
    let exit = loop {
        enable_raw_mode()?;
        let event = crossterm::event::read().unwrap();
        disable_raw_mode()?;

        match event {
            Event::Key(key) => {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if [KeyCode::Esc, KeyCode::Char('q')].contains(&key.code) {
                    break true;
                }

                if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                    break true;
                }

                if now.elapsed().unwrap().as_millis() < 500 {
                    print!(".");
                    stdout().flush()?;
                    continue;
                }

                break false;
            }

            _ => {
                continue;
            }
        }
    };

    return Ok(exit);
}
