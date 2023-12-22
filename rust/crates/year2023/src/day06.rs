use std::str::FromStr;

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    /// Returns the distance traveled for the given hold time.
    fn distance_traveled(&self, hold_time: u64) -> u64 {
        (self.time - hold_time) * hold_time
    }

    /// Returns an iterator over the distances traveled for each hold time.
    fn distances(&self) -> impl Iterator<Item = u64> + '_ {
        (0..self.time).map(|hold_time| self.distance_traveled(hold_time))
    }

    /// Returns the number of races won from all possible hold times.
    fn win_count(&self) -> usize {
        self.distances()
            .filter(|&distance| distance > self.distance)
            .count()
    }
}

#[derive(Debug, Default)]
struct Races(Vec<Race>);

impl Races {
    /// Return the margin of error for all the races.
    fn margin_of_error(&self) -> u64 {
        self.0.iter().map(Race::win_count).product::<usize>() as u64
    }

    /// Returns a [`Race`] which is the combined version of all numbers put
    /// together i.e., ignore any spaces between the numbers.
    fn combined(&self) -> Race {
        Race {
            time: self.0.iter().fold(0, |acc, race| {
                acc * 10u64.pow(number_of_digits(race.time)) + race.time
            }),
            distance: self.0.iter().fold(0, |acc, race| {
                acc * 10u64.pow(number_of_digits(race.distance)) + race.distance
            }),
        }
    }
}

/// Returns the number of digits in the given number.
fn number_of_digits(n: u64) -> u32 {
    let mut n = n;
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

impl FromStr for Races {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (time_line, distance_line) = s
            .split_once('\n')
            .ok_or_else(|| anyhow!("Invalid input: {:?}", s))?;

        let times = time_line
            .split_ascii_whitespace()
            .skip(1)
            .map(|s| {
                s.parse::<u64>()
                    .map_err(|e| anyhow!("Invalid time {:?}: {:?}", s, e))
            })
            .collect::<Result<Vec<_>>>()?;

        let distances = distance_line
            .split_ascii_whitespace()
            .skip(1)
            .map(|s| {
                s.parse::<u64>()
                    .map_err(|e| anyhow!("Invalid distance {:?}: {:?}", s, e))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Races(
            times
                .iter()
                .zip(distances)
                .map(|(time, distance)| Race::new(*time, distance))
                .collect(),
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let races = input.parse::<Races>()?;
    println!("Part 1: {}", races.margin_of_error());

    let combined = races.combined();
    println!("Part 2: {}", combined.win_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_sample() -> Result<()> {
        let races = SAMPLE_INPUT.parse::<Races>()?;
        assert_eq!(races.margin_of_error(), 288);
        assert_eq!(races.combined().win_count(), 71503);
        Ok(())
    }
}
