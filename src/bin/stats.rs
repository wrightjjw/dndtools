use dndtools::gen_stats;
use std::env;

fn run(args: Vec<String>) {
    let stats = gen_stats();
    for s in stats.iter().rev() {
        print!("{} ", s);
    }
    print!("\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args);
}
