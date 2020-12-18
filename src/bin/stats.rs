use dndtools::gen_stats;
use getopts::Options;
use std::env;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};

fn run(args: Vec<String>) {
    let mut opts = Options::new();
    opts.optopt("n", "", "number of stat blocks to calculate", "[NUM]");
    opts.optopt("f", "file", "write output to file", "[FILE]");
    opts.optopt("j", "jobs", "number of jobs (threads) to run", "[THREADS]");
    opts.optflag("q", "quiet", "suppress command line output");
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

    // check if quiet flag is present
    let quiet;
    if matches.opt_present("q") {
        quiet = true;
    } else {
        quiet = false;
    }

    // define num_rolls from n option, panic if error
    let num_rolls: usize = match matches.opt_get_default("n", 1) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    // open a file with mutex, if necessary
    let file_mode = matches.opt_present("f");
    let file = match matches.opt_str("f") {
        Some(m) => Arc::new(Mutex::new(File::create(Path::new(&m)))),
        None => Arc::new(Mutex::new(File::create(Path::new("")))),
    };

    // define num_threads from j option, panic if error
    let num_threads = match matches.opt_get_default("j", num_cpus::get()) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    (0..num_rolls).into_par_iter().for_each(|_x| {
        let stats = gen_stats();
        // println!("Thread {} here.", match rayon::current_thread_index() {
        //     Some(x) => x,
        //     None => 0,
        // });
        let out = format!(
            "{} {} {} {} {} {}\n",
            stats[5], stats[4], stats[3], stats[2], stats[1], stats[0]
        );
        if file_mode {
            file.lock().unwrap().as_ref().unwrap().write_all(out.as_bytes()).unwrap();
        }
        if !quiet {
            print!("{}", out);
        }
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args);
}
