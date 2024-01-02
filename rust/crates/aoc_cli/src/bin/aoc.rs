use std::fs;

use anyhow::{bail, Context, Result};
use clap::Parser;

use aoc_cli::args::{Args, PuzzleDate};

fn main() -> Result<()> {
    let args = Args::parse();

    let PuzzleDate { year, day } = args.date.unwrap_or_default();
    let year = year.unwrap_or_default();
    let day = day.unwrap_or_default();

    let input = fs::read_to_string(format!("./crates/year{year}/input/{day}.txt"))
        .with_context(|| format!("Failed to read input file for year {year} day {day}"))?;

    if let 2023 = year.as_inner() {
        match day.as_inner() {
            1 => year2023::day01::solve(&input),
            2 => year2023::day02::solve(&input),
            3 => year2023::day03::solve(&input),
            4 => year2023::day04::solve(&input),
            6 => year2023::day06::solve(&input),
            7 => year2023::day07::solve(&input),
            8 => year2023::day08::solve(&input),
            9 => year2023::day09::solve(&input),
            11 => year2023::day11::solve(&input),
            _ => bail!("No solution for year {} day {}", year, day),
        }
        .with_context(|| format!("Failed to solve year {year} day {day}"))?;
    } else {
        bail!("No solution for year {}", year)
    }

    Ok(())
}
