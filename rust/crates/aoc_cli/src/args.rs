//! Command-line arguments for the Advent of Code runner.

use std::fmt;
use std::str::FromStr;

use chrono::{Datelike, FixedOffset, TimeZone, Utc};

#[derive(Debug, clap::Parser)]
#[command(name = "aoc")]
pub struct Args {
    /// Print the puzzle input
    #[arg(long)]
    pub show_input: bool,

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

/// The year of an Advent of Code puzzle which ranges from 2015 (the first year) to the current
/// year.
#[derive(Clone, Copy, Debug)]
pub struct PuzzleYear(u16);

impl PuzzleYear {
    pub fn as_inner(&self) -> u16 {
        self.0
    }
}

impl FromStr for PuzzleYear {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year = u16::from_str(s).map_err(|err| err.to_string())?;
        if year < 2015 {
            return Err("the first Advent of Code year is 2015".to_string());
        }
        Ok(Self(year))
    }
}

impl Default for PuzzleYear {
    fn default() -> Self {
        let now = FixedOffset::east_opt(RELEASE_TIMEZONE_OFFSET)
            .unwrap()
            .from_utc_datetime(&Utc::now().naive_utc());
        // SAFETY: `u16` should be more than enough to represent the year.
        let year = u16::try_from(now.year()).unwrap();
        if now.month() < DECEMBER {
            Self(year - 1)
        } else {
            Self(year)
        }
    }
}

impl fmt::Display for PuzzleYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The day of an Advent of Code puzzle which ranges from 1 to 25.
///
/// The default value depends on the current month:
/// * If the current month is before December, the default day is 25.
/// * Otherwise, the default day is the current day of the month clamped to 25.
#[derive(Clone, Copy, Debug)]
pub struct PuzzleDay(u8);

impl PuzzleDay {
    pub fn as_inner(&self) -> u8 {
        self.0
    }
}

impl FromStr for PuzzleDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day = u8::from_str(s).map_err(|err| err.to_string())?;
        if !(1..=25).contains(&day) {
            return Err("the day of an Advent of Code puzzle ranges from 1 to 25".to_string());
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
        let max_day: u8 = if now.year() < 2025 { 25 } else { 12 };
        if now.month() < DECEMBER || day > u32::from(max_day) {
            Self(max_day)
        } else {
            // SAFETY: The library specifies that `day` is always in the range 1..=31.
            Self(u8::try_from(day).unwrap())
        }
    }
}

impl fmt::Display for PuzzleDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
