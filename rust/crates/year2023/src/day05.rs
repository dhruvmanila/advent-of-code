use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FusedIterator;
use std::ops::Range;
use std::str::FromStr;

use anyhow::{Error, Result, anyhow};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Category {
    type Err = Error;

    fn from_str(s: &str) -> Result<Category> {
        match s {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err(anyhow!("Invalid category: {:?}", s)),
        }
    }
}

/// Maps a source range to a destination range.
#[derive(Debug)]
struct RangeMap {
    source: Range<u64>,
    destination: Range<u64>,
}

impl RangeMap {
    /// Maps a value from the source range to the destination range.
    ///
    /// Returns `None` if the value is not contained in the source range.
    fn get(&self, value: u64) -> Option<u64> {
        if self.source.contains(&value) {
            let offset = value - self.source.start;
            Some(self.destination.start + offset)
        } else {
            None
        }
    }

    /// Maps a range from the source range to the destination range.
    ///
    /// Returns `None` if there is no overlap between the source range and the given range.
    /// Otherwise, returns the destination range and the remaining ranges that were not mapped.
    fn get_range(&self, range: &Range<u64>) -> Option<(Range<u64>, Remaining)> {
        let start = self.source.start.max(range.start);
        let end = self.source.end.min(range.end);

        if start >= end {
            // There is no overlap between the source range and the given range.
            return None;
        }

        let destination = Range {
            start: self.destination.start + start - self.source.start,
            end: self.destination.start + end - self.source.start,
        };

        let remaining = match (range.start < start, range.end > end) {
            (true, true) => Remaining::Two(range.start..start, end..range.end),
            (true, false) => Remaining::One(range.start..start),
            (false, true) => Remaining::One(end..range.end),
            (false, false) => Remaining::None,
        };

        Some((destination, remaining))
    }
}

/// The remaining ranges that were not mapped.
#[derive(Debug)]
enum Remaining {
    /// There are no remaining ranges.
    None,
    /// There is one remaining range.
    One(Range<u64>),
    /// There are two remaining ranges.
    Two(Range<u64>, Range<u64>),
}

impl IntoIterator for Remaining {
    type Item = Range<u64>;
    type IntoIter = RemainingIter;

    fn into_iter(self) -> RemainingIter {
        match self {
            Remaining::None => RemainingIter::None,
            Remaining::One(range) => RemainingIter::One(std::iter::once(range)),
            Remaining::Two(range1, range2) => RemainingIter::Two([range1, range2].into_iter()),
        }
    }
}

/// An iterator over the remaining ranges that were not mapped.
enum RemainingIter {
    None,
    One(std::iter::Once<Range<u64>>),
    Two(IntoIter<Range<u64>, 2>),
}

impl Iterator for RemainingIter {
    type Item = Range<u64>;

    fn next(&mut self) -> Option<Range<u64>> {
        match self {
            RemainingIter::None => None,
            RemainingIter::One(iter) => iter.next(),
            RemainingIter::Two(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            RemainingIter::None => (0, Some(0)),
            RemainingIter::One(iter) => iter.size_hint(),
            RemainingIter::Two(iter) => iter.size_hint(),
        }
    }
}

impl ExactSizeIterator for RemainingIter {}
impl FusedIterator for RemainingIter {}

#[derive(Debug)]
struct Almanac {
    /// The values for the seeds.
    seeds: Vec<u64>,

    /// Maps a source category to a destination category and a list of range maps.
    ///
    /// The range maps are used to map numbers from the source category to the destination
    /// category.
    map: HashMap<Category, (Category, Vec<RangeMap>)>,
}

impl Almanac {
    /// Returns the lowest location number that can be reached from the available seeds.
    fn lowest_location_number(&self) -> Result<u64> {
        let mut lowest_number = u64::MAX;

        for seed in &self.seeds {
            let mut current_number = *seed;
            let mut current_category = Category::Seed;

            loop {
                if current_category == Category::Location {
                    // We've reached the location category, so stop and move on to the next seed.
                    lowest_number = lowest_number.min(current_number);
                    break;
                }

                let &(next_category, ref range_maps) = self
                    .map
                    .get(&current_category)
                    .ok_or_else(|| anyhow!("No ranges for category {:?}", current_category))?;

                for range_map in range_maps {
                    if let Some(next_number) = range_map.get(current_number) {
                        // We can directly update the current number because the input specifies
                        // that:
                        //
                        // > Any source numbers that aren't mapped correspond to the same
                        // > destination number.
                        current_number = next_number;
                        break;
                    }
                }

                current_category = next_category;
            }
        }

        Ok(lowest_number)
    }

    /// Converts the almanac into a ranged almanac.
    ///
    /// This is done by converting the seed values into ranges by taking each pair of consecutive
    /// seed numbers. Within each pair, the first number is the start of the range and the second
    /// number is the length of the range.
    ///
    /// # Panics
    ///
    /// Panics if the number of seeds is not even.
    fn into_ranged_almanac(self) -> RangedAlamanac {
        assert!(self.seeds.len().is_multiple_of(2));

        RangedAlamanac {
            seed_ranges: self
                .seeds
                .iter()
                .tuples()
                .map(|(&start, &length)| start..start + length)
                .collect(),
            map: self.map,
        }
    }
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(s: &str) -> Result<Almanac> {
        let (seed_section, map_section) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("Expected two sections separated by two newlines"))?;

        let mut map = HashMap::new();
        for section in map_section.split("\n\n") {
            let mut lines = section.lines();

            let (source, destination) = lines
                .next()
                .ok_or_else(|| anyhow!("Expected a category line"))?
                .split_ascii_whitespace()
                .next()
                .ok_or_else(|| anyhow!("Expected a category map on the category line"))?
                .split_once("-to-")
                .ok_or_else(|| anyhow!("Expected a '-to-' separator in the category map"))?;

            let mut ranges = Vec::new();
            for line in lines {
                let &[destination_start, source_start, length] = line
                    .split_ascii_whitespace()
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()?
                    .as_slice()
                else {
                    return Err(anyhow!("Expected three numbers on a line"));
                };
                ranges.push(RangeMap {
                    source: source_start..source_start + length,
                    destination: destination_start..destination_start + length,
                });
            }

            map.insert(source.parse()?, (destination.parse()?, ranges));
        }

        Ok(Almanac {
            seeds: seed_section
                .split_ascii_whitespace()
                .skip(1)
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
            map,
        })
    }
}

#[derive(Debug)]
struct RangedAlamanac {
    /// The ranges for the seed values.
    seed_ranges: Vec<Range<u64>>,

    /// Maps a source category to a destination category and a list of range maps.
    ///
    /// The range maps are used to map numbers from the source category to the destination
    /// category.
    map: HashMap<Category, (Category, Vec<RangeMap>)>,
}

impl RangedAlamanac {
    /// Returns the lowest location number that can be reached from the available seeds.
    fn lowest_location_number(&self) -> Result<u64> {
        let mut lowest_number = u64::MAX;

        for seed_range in &self.seed_ranges {
            let mut current_ranges = vec![seed_range.clone()];
            let mut current_category = Category::Seed;

            loop {
                if current_category == Category::Location {
                    // We've reached the location category, so stop and move on to the next seed.
                    lowest_number = lowest_number.min(
                        current_ranges
                            .iter()
                            .map(|range| range.start)
                            .min()
                            .ok_or_else(|| anyhow!("No ranges found for the location category"))?,
                    );
                    break;
                }

                let &(next_category, ref range_maps) = self
                    .map
                    .get(&current_category)
                    .ok_or_else(|| anyhow!("No ranges for category {:?}", current_category))?;

                // The ranges that are moved to the next category.
                let mut next_category_ranges = Vec::new();

                // The ranges that are remaining after trying to map the current ranges. These will
                // be merged into the `current_ranges` vector for the next range map.
                let mut remaining_ranges = Vec::new();

                for range_map in range_maps {
                    // We consume all of the current ranges because otherwise we have no way of
                    // knowing which of them have been mapped and which haven't. This difference is
                    // made explicit by using the `remaining_ranges` vector.
                    while let Some(current_range) = current_ranges.pop() {
                        if let Some((next_range, remaining)) = range_map.get_range(&current_range) {
                            next_category_ranges.push(next_range);
                            remaining_ranges.extend(remaining);
                        } else {
                            // The current range was not mapped, so add it to the remaining ranges
                            // for it to be processed using the next range map.
                            remaining_ranges.push(current_range);
                        }
                    }
                    current_ranges.append(&mut remaining_ranges);
                }

                // The current ranges might also contain the ranges that were not mapped by any of
                // the range maps. These ranges will be moved to the next category as they are as
                // specified in the input:
                //
                // > Any source numbers that aren't mapped correspond to the same
                // > destination number.
                current_ranges.append(&mut next_category_ranges);

                current_category = next_category;
            }
        }

        Ok(lowest_number)
    }
}

pub fn solve(input: &str) -> Result<()> {
    let almanac = Almanac::from_str(input)?;
    println!("Part 1: {}", almanac.lowest_location_number()?);

    let ranged_almanac = almanac.into_ranged_almanac();
    println!("Part 2: {}", ranged_almanac.lowest_location_number()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn sample() {
        let almanac = Almanac::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(almanac.lowest_location_number().unwrap(), 35);

        let ranged_almanac = almanac.into_ranged_almanac();
        assert_eq!(ranged_almanac.lowest_location_number().unwrap(), 46);
    }
}
