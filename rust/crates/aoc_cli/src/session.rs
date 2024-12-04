use anyhow::Context;

use crate::args::{PuzzleDay, PuzzleYear};

/// Fetches the puzzle input for the given year and day from the Advent of Code website.
///
/// This will cache the input in `~/.cache/aoc` to avoid fetching it multiple times.
///
/// This function will return an error in the following cases:
/// * If the session token cannot be read from `~/.config/aoc/token`.
/// * If the request to the Advent of Code website fails.
/// * If the response from the Advent of Code website is not valid UTF-8.
/// * If the input cannot be written to the cache.
pub fn get_puzzle_input(year: PuzzleYear, day: PuzzleDay) -> anyhow::Result<String> {
    if let Some(input) = cache::get(year, day) {
        Ok(input)
    } else {
        let token = read_session_token().with_context(|| "Failed to read the session token")?;
        let input = ureq::get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
            .set("Cookie", &format!("session={token}"))
            .call()?
            .into_string()?;
        cache::set(year, day, &input)?;
        Ok(input)
    }
}

mod cache {
    use std::io::Write;
    use std::path::PathBuf;

    use crate::args::{PuzzleDay, PuzzleYear};

    fn path(year: PuzzleYear, day: PuzzleDay) -> PathBuf {
        PathBuf::from(&*shellexpand::tilde("~/.cache/aoc")).join(format!("{year}/{day}.txt"))
    }

    pub(super) fn set(year: PuzzleYear, day: PuzzleDay, input: &str) -> std::io::Result<()> {
        let input_path = path(year, day);
        let prefix = input_path.parent().unwrap();
        std::fs::create_dir_all(prefix)?;
        let mut file = std::fs::File::create(&input_path)?;
        file.write_all(input.as_bytes())
    }

    pub(super) fn get(year: PuzzleYear, day: PuzzleDay) -> Option<String> {
        std::fs::read_to_string(path(year, day)).ok()
    }
}

/// Reads the session token from the default location `~/.config/aoc/token`.
fn read_session_token() -> anyhow::Result<String> {
    let path = shellexpand::tilde("~/.config/aoc/token");
    Ok(std::fs::read_to_string(&*path)
        .with_context(|| format!("{path:?}"))?
        .trim()
        .to_string())
}
