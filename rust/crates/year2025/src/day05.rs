use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::{Context, Error, Result, bail};

enum Remaining {
    None,
    One(RangeInclusive<u64>),
    Two(RangeInclusive<u64>, RangeInclusive<u64>),
}

/// Performs the range difference operation `a - b` and returns the remaining parts of `a` that
/// are not covered by `b`.
fn range_difference(a: RangeInclusive<u64>, b: RangeInclusive<u64>) -> Remaining {
    let max_start = a.start().max(b.start());
    let min_end = a.end().min(b.end());

    if max_start > min_end {
        return Remaining::One(a); // No overlap
    }

    match (a.start() < b.start(), a.end() > b.end()) {
        (true, true) => {
            // a: |------------------|
            // b:      |---------|
            //    ^^^^^           ^^^^ remaining
            Remaining::Two(*a.start()..=b.start() - 1, b.end() + 1..=*a.end())
        }
        (true, false) => {
            // a: |------------|
            // b:       |---------|
            //    ^^^^^^ remaining
            Remaining::One(*a.start()..=b.start() - 1)
        }
        (false, true) => {
            // a:   |------------|
            // b: |---------|
            //               ^^^^^ remaining
            Remaining::One(b.end() + 1..=*a.end())
        }
        (false, false) => {
            // a:   |--------|
            // b: |---------------|
            Remaining::None
        }
    }
}

#[derive(Debug)]
struct FreshIngredientIds {
    ranges: Vec<RangeInclusive<u64>>,
}

impl FreshIngredientIds {
    /// Returns true if the given ingredient ID is fresh i.e., falls within any of the fresh ID
    /// ranges.
    fn contains(&self, id: u64) -> bool {
        self.ranges.iter().any(|range| range.contains(&id))
    }

    /// Returns the count of all the unique fresh ingredient IDs.
    fn unique_count(&self) -> usize {
        let mut ranges = self.ranges.iter();
        let Some(first_range) = ranges.next() else {
            return 0;
        };

        let mut unique_ranges: Vec<RangeInclusive<u64>> = Vec::with_capacity(self.ranges.len());
        unique_ranges.push(first_range.clone());

        for range in &self.ranges {
            // This holds the remaining parts of the `range` that are not covered by any of the
            // unique ranges.
            let mut remaining_ranges = vec![range.clone()];

            for unique_range in &unique_ranges {
                // This holds the remaining parts of `remaining_ranges` after subtracting the
                // `unique_range`.
                let mut current_remaining_ranges = vec![];

                while let Some(current_range) = remaining_ranges.pop() {
                    match range_difference(current_range, unique_range.clone()) {
                        Remaining::None => {
                            // current_range is fully covered by unique_range
                        }
                        Remaining::One(r) => {
                            current_remaining_ranges.push(r);
                        }
                        Remaining::Two(r1, r2) => {
                            current_remaining_ranges.push(r1);
                            current_remaining_ranges.push(r2);
                        }
                    }
                }

                remaining_ranges.append(&mut current_remaining_ranges);
            }

            unique_ranges.append(&mut remaining_ranges);
        }

        unique_ranges
            .into_iter()
            .map(std::iter::Iterator::count)
            .sum()
    }
}

impl FromStr for FreshIngredientIds {
    type Err = Error;

    fn from_str(s: &str) -> Result<FreshIngredientIds, Error> {
        Ok(FreshIngredientIds {
            ranges: s
                .lines()
                .map(|line| {
                    let Some((start_str, end_str)) = line.split_once('-') else {
                        bail!("invalid range format: {line:?} (expected 'start-end')");
                    };
                    let start = start_str
                        .parse::<u64>()
                        .with_context(|| format!("invalid start of range: {start_str:?}"))?;
                    let end = end_str
                        .parse::<u64>()
                        .with_context(|| format!("invalid end of range: {end_str:?}"))?;
                    Ok(start..=end)
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug)]
struct Database {
    fresh_ingredient_ids: FreshIngredientIds,
    available_ingredient_ids: Vec<u64>,
}

impl Database {
    fn fresh_available_count(&self) -> usize {
        self.available_ingredient_ids
            .iter()
            .filter(|&&id| self.fresh_ingredient_ids.contains(id))
            .count()
    }

    fn fresh_ingredient_ids(&self) -> &FreshIngredientIds {
        &self.fresh_ingredient_ids
    }
}

impl FromStr for Database {
    type Err = Error;

    fn from_str(s: &str) -> Result<Database, Error> {
        let Some((fresh_ids_str, available_ids_str)) = s.split_once("\n\n") else {
            bail!("invalid input format: expected two sections separated by a blank line");
        };
        Ok(Database {
            fresh_ingredient_ids: FreshIngredientIds::from_str(fresh_ids_str)?,
            available_ingredient_ids: available_ids_str
                .lines()
                .map(|line| {
                    line.parse::<u64>()
                        .with_context(|| format!("invalid available ingredient ID: {line:?}"))
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

pub fn solve(input: &str) -> Result<()> {
    let database = Database::from_str(input)?;

    println!("Part 1: {}", database.fresh_available_count());
    println!("Part 2: {}", database.fresh_ingredient_ids().unique_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn sample() {
        let database = Database::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(database.fresh_available_count(), 3);
        assert_eq!(database.fresh_ingredient_ids().unique_count(), 14);
    }
}
