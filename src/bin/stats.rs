use dndtools::gen_stats;
use getopts::Options;
use std::env;

fn run(args: Vec<String>) {
    let mut opts = Options::new();
    opts.optopt("n", "", "number of stat blocks to calculate", "[NUM]");
    opts.optflag("h", "help", "display help information");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [OPTIONS]", args[0]);
        print!("{}", opts.usage(&brief));
        return;
    }

    // define num_rolls from n option, panic if error
    let num_rolls: u32 = match matches.opt_get_default("n", 1) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    let mut i = 0;
    while i < num_rolls {
        let stats = gen_stats();
        for s in stats.iter().rev() {
            print!("{} ", s);
        }
        print!("\n");
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args);
}
