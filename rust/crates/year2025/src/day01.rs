use std::{fmt, str::FromStr};

use anyhow::{Context, Error, Result, anyhow};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Direction, Error> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow!("Invalid direction: {}", s)),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Left => f.write_str("L"),
            Direction::Right => f.write_str("R"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Rotation {
    direction: Direction,
    distance: u16,
}

impl FromStr for Rotation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Rotation, Error> {
        let (direction_str, distance_str) = s.split_at(1);
        Ok(Rotation {
            direction: direction_str.parse()?,
            distance: distance_str
                .parse()
                .with_context(|| format!("invalid distance: {distance_str}"))?,
        })
    }
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.direction, self.distance)
    }
}

#[derive(Debug)]
struct Document {
    rotations: Vec<Rotation>,
}

impl Document {
    fn password(&self) -> (u32, u32) {
        let mut dial = Dial::START;
        let mut password1 = 0;
        let mut password2 = 0;

        for rotation in &self.rotations {
            let result = dial.rotate(*rotation);
            if result.at_zero {
                password1 += 1;
            }
            password2 += result.zero_count;
        }

        (password1, password2)
    }
}

impl FromStr for Document {
    type Err = Error;

    fn from_str(s: &str) -> Result<Document, Error> {
        Ok(Document {
            rotations: s
                .lines()
                .map(str::parse)
                .collect::<Result<Vec<Rotation>, Error>>()?,
        })
    }
}

#[derive(Debug)]
struct Dial(u16);

impl Dial {
    const START: Dial = Dial(50);

    fn rotate(&mut self, rotation: Rotation) -> RotationResult {
        let (mut zero_count, remainder) = (rotation.distance / 100, rotation.distance % 100);
        match rotation.direction {
            Direction::Left => {
                let value = self.0 + 100 - remainder;
                if value / 100 == 0 && self.0 != 0 {
                    // Wrapped around zero
                    zero_count += 1;
                }
                self.0 = value % 100;
            }
            Direction::Right => {
                let value = self.0 + remainder;
                if value / 100 == 1 && value != 100 {
                    zero_count += 1;
                }
                self.0 = value % 100;
            }
        }
        RotationResult {
            at_zero: self.0 == 0,
            zero_count: u32::from(zero_count) + u32::from(self.0 == 0),
        }
    }
}

#[derive(Debug)]
struct RotationResult {
    at_zero: bool,
    zero_count: u32,
}

pub fn solve(input: &str) -> Result<()> {
    let document = Document::from_str(input)?;
    let (password1, password2) = document.password();

    println!("Part 1: {password1}");
    println!("Part 2: {password2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn sample() {
        let document = Document::from_str(SAMPLE_INPUT).unwrap();
        let (password1, password2) = document.password();
        assert_eq!(password1, 3);
        assert_eq!(password2, 6);
    }
}
