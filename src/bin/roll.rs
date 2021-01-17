// Copyright (C) 2021 Josh Wright

use dndtools::{roll_dice, DiceToRoll};
use getopts::Options;
use std::env;
use std::process::exit;

fn run(args: Vec<String>) -> Result<(), String> {
    // define options
    let mut opts = Options::new();
    opts.optflag("h", "help", "display help information");
    let matches = opts.parse(&args).unwrap();

    // process options
    if matches.opt_present("h") {
        let brief = format!("Usage: {} [OPTIONS] ROLLS", args[0]);
        print!("{}", opts.usage(&brief));
        return Ok(());
    }

    // check for no arguments
    if matches.free.len() == 1 {
        return Err("missing dice".to_string());
    }

    // convert args to a Vec<DiceToRoll>
    let mut dice: Vec<DiceToRoll> = Vec::new();
    for roll in matches.free.iter().skip(1) {
        let this_dice = DiceToRoll::from_string(roll.to_string())?;
        dice.push(this_dice);
    }

    // roll dice
    let dice = roll_dice(dice);

    // output
    for die in dice.types.iter() {
        println!(
            "{}d{}: {} {:?}",
            die.rolls.len(),
            die.die as u32,
            die.total,
            die.rolls
        );
    }
    if matches.free.len() > 2 {
        println!("Total: {}", dice.total);
    }

    Ok(())
}

fn main() {
    let args = env::args().collect();
    match run(args) {
        Ok(_) => exit(0),
        Err(e) => {
            println!("roll: {}", e);
            println!("Try 'roll --help' for more info.");
            exit(1);
        }
    };
}
