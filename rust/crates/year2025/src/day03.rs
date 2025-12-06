use std::fmt;

use anyhow::Result;

#[derive(Debug)]
struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    /// Find the largest joltage considering `n` batteries from the bank.
    fn largest_joltage(&self, n: u8) -> u64 {
        fn inner(largest: &mut u64, joltage: u64, batteries: &[u8], remaining: u8) {
            if remaining == 0 {
                if joltage > *largest {
                    *largest = joltage;
                }
                return;
            }
            let ratings = if *largest == 0 {
                // We still haven't found a largest yet so use the entire search space.
                (1..=9).rev()
            } else {
                // There's a largest value which can be used as a reference to prune the search
                // space. This would be done by checking the position we're currently at, the digit
                // at that position in the largest value and avoiding anything smaller than that
                // digit.
                // SAFETY: mod 10 should mean that the value is always between 0 and 9
                let digit =
                    u8::try_from((*largest / 10u64.pow(u32::from(remaining - 1))) % 10).unwrap();
                (digit..=9).rev()
            };
            for rating in ratings {
                let Some(position) = batteries.iter().position(|&battery| battery == rating) else {
                    continue;
                };
                let remaining_batteries = &batteries[position + 1..];
                if remaining_batteries.len() < usize::from(remaining - 1) {
                    continue;
                }
                inner(
                    largest,
                    joltage * 10 + u64::from(rating),
                    remaining_batteries,
                    remaining - 1,
                );
            }
        }

        let mut largest = 0;
        inner(&mut largest, 0, &self.batteries, n);
        largest
    }
}

impl From<&str> for Bank {
    fn from(s: &str) -> Bank {
        Bank {
            batteries: s.bytes().map(|b| b - b'0').collect(),
        }
    }
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &joltage in &self.batteries {
            write!(f, "{joltage}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Banks(Vec<Bank>);

impl Banks {
    fn largest_joltages(&self, batteries: u8) -> impl Iterator<Item = u64> {
        self.0
            .iter()
            .map(move |bank| bank.largest_joltage(batteries))
    }

    fn sum_largest_joltages(&self, batteries: u8) -> u64 {
        self.largest_joltages(batteries).sum()
    }
}

impl From<&str> for Banks {
    fn from(s: &str) -> Banks {
        Banks(s.lines().map(Bank::from).collect())
    }
}

pub fn solve(input: &str) -> Result<()> {
    let banks = Banks::from(input);

    println!("Part 1: {}", banks.sum_largest_joltages(2));
    println!("Part 2: {}", banks.sum_largest_joltages(12));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn sample() {
        let banks = Banks::from(SAMPLE_INPUT);
        // for bank in &banks.0 {
        //     println!("{} -> {}", &bank, bank.largest_joltage(2));
        // }
        assert_eq!(banks.sum_largest_joltages(2), 357);
        assert_eq!(banks.sum_largest_joltages(12), 3_121_910_778_619);
    }
}
