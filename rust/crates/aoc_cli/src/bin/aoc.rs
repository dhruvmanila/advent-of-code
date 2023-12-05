use std::fs;

use anyhow::{bail, Context, Result};
use clap::Parser;

use aoc_cli::args::{Args, PuzzleDate};

fn main() -> Result<()> {
    let args = Args::parse();

    let PuzzleDate { year, day } = args.date.unwrap_or_default();
    let year = year.unwrap_or_default();
    let day = day.unwrap_or_default();

    let input = fs::read_to_string(format!("./crates/year{}/input/{}.txt", year, day))
        .with_context(|| format!("Failed to read input file for year {} day {}", year, day))?;

    if let 2023 = year.as_inner() {
        match day.as_inner() {
            1 => year2023::day01::solve(&input),
            2 => year2023::day02::solve(&input)?,
            _ => bail!("No solution for year {} day {}", year, day),
        }
    } else {
        bail!("No solution for year {}", year)
    }

    Ok(())
}
