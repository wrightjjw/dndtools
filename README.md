# dndtools
A library of tools to use for Dungeons and Dragons, specifically 5th Edition.
The tools are available as a library of Rust functions
as well as a collection of cli binaries.

So far there is a stat generator and a dice roller.
There are plans to add an HP generator.

# CLI Usage
All programs take a `-h` or `--help` flag to display help information.

## stats
When used with no options,
`stats` will print out a block of PC stats
where each stat is calculated by rolling 4d6 and dropping the lowest roll.

- `-n NUM` will generate `NUM` blocks. Defaults to 1.
- `-f FILE` will write output to `FILE`.
- `-j JOBS` will set the number of jobs (threads) to use.
- `-q` will supress command line output.
Defaults to the maximum number of CPU threads on the system.

## roll
`roll` takes any number of arguments in valid die formats,
such as "2d6" or "d20".
It prints the output of each individual roll and total of each group given,
as well as a grand total.
It must be provided with at least one argument.
