use anyhow::{anyhow, Context, Result};
use clap::Parser;

use aoc_cli::args::{Args, PuzzleDate};
use aoc_cli::session::get_puzzle_input;

fn main() -> Result<()> {
    let args = Args::parse();

    let PuzzleDate { year, day } = args.date.unwrap_or_default();
    let year = year.unwrap_or_default();
    let day = day.unwrap_or_default();
    let input = get_puzzle_input(year, day)?;

    match year.as_inner() {
        2023 => match day.as_inner() {
            1 => year2023::day01::solve(&input),
            2 => year2023::day02::solve(&input),
            3 => year2023::day03::solve(&input),
            4 => year2023::day04::solve(&input),
            6 => year2023::day06::solve(&input),
            7 => year2023::day07::solve(&input),
            8 => year2023::day08::solve(&input),
            9 => year2023::day09::solve(&input),
            11 => year2023::day11::solve(&input),
            _ => Err(anyhow!("No solution available")),
        },
        2024 => match day.as_inner() {
            1 => year2024::day01::solve(&input),
            2 => year2024::day02::solve(&input),
            3 => year2024::day03::solve(&input),
            4 => year2024::day04::solve(&input),
            5 => year2024::day05::solve(&input),
            _ => Err(anyhow!("No solution available")),
        },
        _ => Err(anyhow!("No solution available")),
    }
    .with_context(|| format!("Failed to solve year {year} day {day}"))?;

    Ok(())
}
