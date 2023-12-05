use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};

/// The maximum cube set i.e., the number of cubes of each color that is
/// contained in the bag for the puzzle.
const MAX_CUBE_SET: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

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
    /// Returns a new cube set with the given red count.
    fn with_red(mut self, red: u32) -> Self {
        self.red = red;
        self
    }

    /// Returns a new cube set with the given green count.
    fn with_green(mut self, green: u32) -> Self {
        self.green = green;
        self
    }

    /// Returns a new cube set with the given blue count.
    fn with_blue(mut self, blue: u32) -> Self {
        self.blue = blue;
        self
    }

    /// Returns `true` if the cube set is possible. That is, if the cube set
    /// does not exceed the maximum cube set.
    fn is_possible(&self) -> bool {
        self.red <= MAX_CUBE_SET.red
            && self.green <= MAX_CUBE_SET.green
            && self.blue <= MAX_CUBE_SET.blue
    }

    /// Returns the power of the cube set.
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for CubeSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube_set = Self::default();
        for cube_info in s.split(", ") {
            let (count, color) = cube_info
                .split_once(' ')
                .ok_or_else(|| anyhow::anyhow!("Invalid cube: {}", cube_info))?;
            let count = count.parse::<u32>()?;
            cube_set = match color {
                "red" => cube_set.with_red(count),
                "green" => cube_set.with_green(count),
                "blue" => cube_set.with_blue(count),
                _ => bail!("Invalid color: {}", color),
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
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (game_section, cube_sets_section) = line
            .split_once(": ")
            .ok_or_else(|| anyhow!("Invalid line: {}", line))?;

        let game_id = game_section
            .split_whitespace()
            .nth(1)
            .and_then(|id| id.trim_end_matches(':').parse::<u32>().ok())
            .ok_or_else(|| anyhow!("Invalid game section of the line: {}", game_section))?;

        let mut cube_sets = Vec::with_capacity(3);
        for cube_set_section in cube_sets_section.split("; ") {
            cube_sets.push(cube_set_section.parse::<CubeSet>()?);
        }

        Ok(Self {
            id: game_id,
            sets: cube_sets,
        })
    }
}

/// Represents a collection of games.
#[derive(Debug, Default)]
struct Games(Vec<Game>);

impl Games {
    /// Returns an iterator over the IDs of the games that are possible.
    fn possible_game_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.iter()
            .filter(|game| game.is_possible())
            .map(|game| game.id)
    }

    /// Returns an iterator over the powers of the minimum cube set for each
    /// game.
    fn min_cube_set_powers(&self) -> impl Iterator<Item = u32> + '_ {
        self.iter().map(|game| game.min_cube_set().power())
    }
}

impl Deref for Games {
    type Target = Vec<Game>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Games {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Games {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut games = Self::default();
        for line in s.lines() {
            games.push(line.parse::<Game>()?);
        }
        Ok(games)
    }
}

pub fn solve(input: &str) -> Result<()> {
    let games = Games::from_str(input)?;

    println!("Part 1: {:?}", games.possible_game_ids().sum::<u32>());
    println!("Part 2: {:?}", games.min_cube_set_powers().sum::<u32>());

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
    fn test_sample() {
        let games = Games::from_str(SAMPLE_INPUT).unwrap();

        let possible_game_ids = games.possible_game_ids().collect::<Vec<_>>();
        assert_eq!(possible_game_ids, vec![1, 2, 5]);
        assert_eq!(possible_game_ids.iter().sum::<u32>(), 8);

        let min_cube_set_powers = games.min_cube_set_powers().collect::<Vec<_>>();
        assert_eq!(min_cube_set_powers, vec![48, 12, 1560, 630, 36]);
        assert_eq!(min_cube_set_powers.iter().sum::<u32>(), 2286);
    }
}
