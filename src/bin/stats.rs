use dndtools::gen_stats;
use getopts::Options;
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use std::sync::{Arc, Mutex};

fn run(args: Vec<String>) -> Result<(), String> {
    let mut opts = Options::new();
    opts.optopt("n", "", "number of stat blocks to calculate", "[NUM]");
    opts.optopt("f", "file", "write output to file", "[FILE]");
    opts.optopt("j", "jobs", "number of jobs (threads) to run", "[THREADS]");
    opts.optflag("q", "quiet", "suppress command line output");
    opts.optflag("h", "help", "display help information");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [OPTIONS]", args[0]);
        print!("{}", opts.usage(&brief));
        return Ok(());
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
        Ok(n) => n,
        Err(e) => {
            return Err("-n: ".to_string() + &e.to_string());
        }
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
        Err(e) => {
            return Err("-j: ".to_string() + &e.to_string());
        }
    };
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .map_err(|e| e.to_string())?;

    (0..num_rolls)
        .into_par_iter()
        .try_for_each(|_| -> Result<(), String> {
            let stats = gen_stats();
            let out = format!(
                "{} {} {} {} {} {}\n",
                stats[5], stats[4], stats[3], stats[2], stats[1], stats[0]
            );
            if file_mode {
                file.lock()
                    .map_err(|e| e.to_string())?
                    .as_ref()
                    .map_err(|e| e.to_string())?
                    .write_all(out.as_bytes())
                    .map_err(|e| e.to_string())?
            }
            if !quiet {
                print!("{}", out);
            }
            Ok(())
        })?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match run(args) {
        Ok(_) => exit(0),
        Err(e) => {
            println!("stats: {}", e);
            println!("Try 'stats --help' for more info.");
            exit(1);
        }
    };
}
