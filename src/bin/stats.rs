use dndtools::gen_stats;
use getopts::Options;
use std::env;
use rayon::prelude::*;

fn run(args: Vec<String>) {
    let mut opts = Options::new();
    opts.optopt("n", "", "number of stat blocks to calculate", "[NUM]");
    // opts.optopt("j", "jobs", "number of jobs (threads) to run", "[THREADS]");
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
    let num_rolls: usize = match matches.opt_get_default("n", 1) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };


    // define num_threads from j option, panic if error
    //TODO: add -j option

    (0..num_rolls).into_par_iter().for_each(|_x| {
        let stats = gen_stats();
        println!("{} {} {} {} {} {}", stats[5], stats[4], stats[3], stats[2], stats[1], stats[0]);
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args);
}
