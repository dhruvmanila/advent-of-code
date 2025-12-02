use std::{ops::RangeInclusive, str::FromStr};

use anyhow::{Context, Error, Result, bail};

/// Split the given number into equal parts of the given size in digits.
fn split_equally(n: u64, ndigits: u32, size: u32) -> Option<impl Iterator<Item = u64>> {
    if size == 0 || !ndigits.is_multiple_of(size) {
        return None;
    }
    let nparts = ndigits / size;
    let divisor = 10u64.pow(size);
    Some((0..nparts).map(move |i| (n / divisor.pow(nparts - i - 1)) % divisor))
}

#[derive(Debug)]
struct ProductIdRanges {
    ranges: Vec<RangeInclusive<u64>>,
}

impl ProductIdRanges {
    fn invalid_product_ids(&self) -> impl Iterator<Item = u64> + '_ {
        self.ranges
            .iter()
            .flat_map(std::clone::Clone::clone)
            .filter(|&id| {
                let ndigits = id.ilog10() + 1;
                if ndigits.is_multiple_of(2) {
                    let divisor = 10u64.pow(ndigits / 2);
                    let (left, right) = (id / divisor, id % divisor);
                    if left == right {
                        return true;
                    }
                }
                false
            })
    }

    fn invalid_product_ids_with_silly_patterns(&self) -> impl Iterator<Item = u64> + '_ {
        self.ranges
            .iter()
            .flat_map(std::clone::Clone::clone)
            .filter(|&id| {
                let ndigits = id.ilog10() + 1;
                for size in 1..=ndigits / 2 {
                    let Some(mut parts) = split_equally(id, ndigits, size) else {
                        continue;
                    };
                    let Some(first_part) = parts.next() else {
                        continue;
                    };
                    if parts.all(|part| part == first_part) {
                        return true;
                    }
                }
                false
            })
    }

    fn sum_invalid_product_ids(&self) -> u64 {
        self.invalid_product_ids().sum()
    }

    fn sum_invalid_product_ids_with_silly_patterns(&self) -> u64 {
        self.invalid_product_ids_with_silly_patterns().sum()
    }
}

impl FromStr for ProductIdRanges {
    type Err = Error;

    fn from_str(s: &str) -> Result<ProductIdRanges, Error> {
        Ok(ProductIdRanges {
            ranges: s
                .strip_suffix('\n')
                .unwrap_or(s)
                .split(',')
                .map(|part| {
                    let Some((start_str, end_str)) = part.split_once('-') else {
                        bail!("invalid range format: {part:?} (expected 'start-end')");
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

pub fn solve(input: &str) -> Result<()> {
    let ranges = ProductIdRanges::from_str(input)?;

    println!("Part 1: {}", ranges.sum_invalid_product_ids());
    println!(
        "Part 2: {}",
        ranges.sum_invalid_product_ids_with_silly_patterns()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    const SAMPLE_INPUT: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    #[test]
    fn sample() {
        let ranges = ProductIdRanges::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(ranges.sum_invalid_product_ids(), 1_227_775_554);
        assert_eq!(
            ranges.sum_invalid_product_ids_with_silly_patterns(),
            4_174_379_265
        );
    }

    #[test_case(123, 3, 2, None)]
    #[test_case(123, 3, 1, Some(vec![1, 2, 3]))]
    #[test_case(11_22_33_44_55, 10, 2, Some(vec![11, 22, 33, 44, 55]))]
    #[allow(clippy::needless_pass_by_value)]
    fn test_split_equally(n: u64, ndigits: u32, size: u32, expected: Option<Vec<u64>>) {
        let result: Option<Vec<u64>> =
            split_equally(n, ndigits, size).map(std::iter::Iterator::collect);
        assert_eq!(result, expected);
    }
}
