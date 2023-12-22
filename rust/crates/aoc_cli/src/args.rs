//! Command-line arguments for the Advent of Code runner.

use std::fmt;
use std::str::FromStr;

use chrono::{Datelike, FixedOffset, TimeZone, Utc};

#[derive(Debug, clap::Parser)]
#[command(name = "aoc")]
pub struct Args {
    #[command(flatten)]
    pub date: Option<PuzzleDate>,
}

#[derive(Debug, Default, clap::Parser)]
pub struct PuzzleDate {
    /// The year of the puzzle
    #[arg(short, long)]
    pub year: Option<PuzzleYear>,

    /// The day of the puzzle
    #[arg(short, long)]
    pub day: Option<PuzzleDay>,
}

/// The month number of December (12).
const DECEMBER: u32 = 12;
/// The offset of the puzzle release timezone from UTC in seconds.
const RELEASE_TIMEZONE_OFFSET: i32 = -5 * 3600;

#[derive(Clone, Debug)]
pub struct PuzzleYear(i32);

impl PuzzleYear {
    pub fn as_inner(&self) -> i32 {
        self.0
    }
}

impl FromStr for PuzzleYear {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year = i32::from_str(s).map_err(|_| format!("Invalid year: {s}"))?;
        if year < 2015 {
            return Err(format!("{year} is not a valid Advent of Code year"));
        }
        Ok(Self(year))
    }
}

impl Default for PuzzleYear {
    fn default() -> Self {
        let now = FixedOffset::east_opt(RELEASE_TIMEZONE_OFFSET)
            .unwrap()
            .from_utc_datetime(&Utc::now().naive_utc());
        let year = now.year();
        if now.month() < DECEMBER {
            Self(year - 1)
        } else {
            Self(year)
        }
    }
}

impl fmt::Display for PuzzleYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct PuzzleDay(u32);

impl PuzzleDay {
    pub fn as_inner(&self) -> u32 {
        self.0
    }
}

impl FromStr for PuzzleDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day = u32::from_str(s).map_err(|_| format!("Invalid day: {s}"))?;
        if !(1..=25).contains(&day) {
            return Err(format!("{day} is not a valid Advent of Code day"));
        }
        Ok(Self(day))
    }
}

impl Default for PuzzleDay {
    fn default() -> Self {
        let now = FixedOffset::east_opt(RELEASE_TIMEZONE_OFFSET)
            .unwrap()
            .from_utc_datetime(&Utc::now().naive_utc());
        let day = now.day();
        if now.month() < DECEMBER || day > 25 {
            Self(25)
        } else {
            Self(day)
        }
    }
}

impl fmt::Display for PuzzleDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}", self.0)
    }
}
