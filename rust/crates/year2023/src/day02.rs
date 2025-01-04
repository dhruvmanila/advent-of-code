use std::str::FromStr;

use anyhow::{anyhow, bail, Context, Error, Result};

/// Represents a set of cubes of different colors.
#[derive(Debug, Default)]
struct CubeSet {
    /// The number of red cubes.
    red: u32,
    /// The number of green cubes.
    green: u32,
    /// The number of blue cubes.
    blue: u32,
}

impl CubeSet {
    /// The maximum cube set i.e., the number of cubes of each color that is contained in the bag
    /// for the puzzle.
    const MAX: CubeSet = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    /// Returns a new cube set with the given red count.
    #[must_use]
    const fn with_red(mut self, red: u32) -> CubeSet {
        self.red = red;
        self
    }

    /// Returns a new cube set with the given green count.
    #[must_use]
    const fn with_green(mut self, green: u32) -> CubeSet {
        self.green = green;
        self
    }

    /// Returns a new cube set with the given blue count.
    #[must_use]
    const fn with_blue(mut self, blue: u32) -> CubeSet {
        self.blue = blue;
        self
    }

    /// Returns `true` if the cube set is possible. That is, if the cube set
    /// does not exceed the maximum cube set.
    const fn is_possible(&self) -> bool {
        self.red <= CubeSet::MAX.red
            && self.green <= CubeSet::MAX.green
            && self.blue <= CubeSet::MAX.blue
    }

    /// Returns the power of the cube set.
    const fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for CubeSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<CubeSet, Error> {
        let mut cube_set = CubeSet::default();
        for cube_info in s.split(", ") {
            let (count, color) = cube_info.split_once(' ').ok_or_else(|| {
                anyhow!(
                    "Invalid cube info: {:?} (expected '<count> <color>')",
                    cube_info
                )
            })?;
            let count = count.parse::<u32>()?;
            cube_set = match color {
                "red" => cube_set.with_red(count),
                "green" => cube_set.with_green(count),
                "blue" => cube_set.with_blue(count),
                _ => bail!("Invalid cube color: {:?}", color),
            };
        }
        Ok(cube_set)
    }
}

/// Represents a single game of cube sets.
#[derive(Debug)]
struct Game {
    /// The ID of the game.
    id: u32,
    /// The cube sets in the game.
    sets: Vec<CubeSet>,
}

impl Game {
    /// Returns `true` if the game is possible.
    fn is_possible(&self) -> bool {
        self.sets.iter().all(CubeSet::is_possible)
    }

    /// Returns the minimum cube set required for this game to be possible.
    fn min_cube_set(&self) -> CubeSet {
        self.sets
            .iter()
            .fold(CubeSet::default(), |acc, cube_set| CubeSet {
                red: acc.red.max(cube_set.red),
                green: acc.green.max(cube_set.green),
                blue: acc.blue.max(cube_set.blue),
            })
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Game, Error> {
        let (game_section, cube_sets_section) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("Expected a colon in the line"))?;

        Ok(Game {
            id: game_section
                .split_whitespace()
                .nth(1)
                .and_then(|id| id.parse::<u32>().ok())
                .ok_or_else(|| {
                    anyhow!(
                        "Invalid game section of the line: {:?} (expected 'Game <id>')",
                        game_section
                    )
                })?,
            sets: cube_sets_section
                .split("; ")
                .map(CubeSet::from_str)
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

/// Represents a collection of games.
#[derive(Debug, Default)]
struct Games(Vec<Game>);

impl Games {
    /// Returns the sum of the IDs of the possible games.
    fn sum_possible_game_ids(&self) -> u32 {
        self.0
            .iter()
            .filter(|game| game.is_possible())
            .map(|game| game.id)
            .sum()
    }

    /// Returns the sum of the powers of the minimum cube set for each game.
    fn sum_min_cube_set_powers(&self) -> u32 {
        self.0.iter().map(|game| game.min_cube_set().power()).sum()
    }
}

impl FromStr for Games {
    type Err = Error;

    fn from_str(s: &str) -> Result<Games, Error> {
        Ok(Games(
            s.lines()
                .map(|line| {
                    Game::from_str(line).with_context(|| format!("Failed to parse line: {line:?}"))
                })
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let games = Games::from_str(input)?;

    println!("Part 1: {}", games.sum_possible_game_ids());
    println!("Part 2: {}", games.sum_min_cube_set_powers());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn sample() {
        let games = Games::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(games.sum_possible_game_ids(), 8);
        assert_eq!(games.sum_min_cube_set_powers(), 2286);
    }
}
