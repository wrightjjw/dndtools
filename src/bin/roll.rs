use dndtools::{roll_dice, DiceToRoll};
use getopts::Options;
use std::env;

fn run(args: Vec<String>) {
    // define options
    let mut opts = Options::new();
    opts.optflag("h", "help", "display help information");
    let matches = opts.parse(&args).unwrap();

    // process options
    if matches.opt_present("h") {
        let brief = format!("Usage: {} [OPTIONS] ROLLS", args[0]);
        print!("{}", opts.usage(&brief));
        return;
    }

    // convert args to a Vec<DiceToRoll>
    let mut dice: Vec<DiceToRoll> = Vec::new();
    for roll in matches.free.iter() {
        let this_dice = match DiceToRoll::from_string(roll.to_string()) {
            Ok(x) => x,
            Err(e) => panic!(e),
        };
        dice.push(this_dice);
    }

    // roll dice
    let dice = roll_dice(dice);

    // output
    for die in dice.types.iter() {
        println!("{}: {} ({:?})", die.die as u32, die.total, die.rolls);
    }
    println!("Total: {}", dice.total);
}

fn main() {
    let args = env::args().collect();
    run(args);
}
