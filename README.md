# dndtools
A library of tools to use for Dungeons and Dragons, specifically 5th Edition.
The tools are available as a library of Rust functions
as well as a collection of cli binaries.

So far there is a stat generator. That's about it so far.

# CLI Usage
All programs take a `-h` or `--help` flag to display help information.

## stats
When used with no options,
`stats` will print out a block of PC stats
where each stat is calculated by rolling 4d6 and dropping the lowest roll.

- `-n NUM` will generate `NUM` blocks. Defaults to 1.
