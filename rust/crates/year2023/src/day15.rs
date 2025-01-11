use std::fmt::{self, Write};

use anyhow::{anyhow, Error, Result};
use itertools::Either;

/// Returns the hash value of the bytes according to the algorithm as described in the problem.
///
/// The hash value is guaranteed to be in the range 0..=255.
fn hash(bytes: impl Iterator<Item = u8>) -> u8 {
    let mut value = 0;
    for byte in bytes {
        value += u32::from(byte);
        value *= 17;
        value &= 0xFF;
    }
    // SAFETY: The value is masked to 8 bits at the end of each iteration in the above loop.
    u8::try_from(value).unwrap()
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Dash,
    Equal(u8),
}

impl Operation {
    /// Returns an iterator over the bytes that represents the string value of the operation.
    fn bytes(self) -> impl Iterator<Item = u8> {
        match self {
            Operation::Dash => Either::Left([b'-'].into_iter()),
            Operation::Equal(value) => Either::Right([b'=', value + b'0'].into_iter()),
        }
    }
}

#[derive(Debug)]
struct Step<'a> {
    label: &'a str,
    operation: Operation,
}

impl<'a> Step<'a> {
    /// Parses a step from a string.
    fn parse(s: &'a str) -> Result<Step<'a>, Error> {
        match s.as_bytes() {
            [label @ .., b'-'] => Ok(Step {
                label: unsafe { std::str::from_utf8_unchecked(label) },
                operation: Operation::Dash,
            }),
            [label @ .., b'=', value] => Ok(Step {
                label: unsafe { std::str::from_utf8_unchecked(label) },
                operation: Operation::Equal(value - b'0'),
            }),
            _ => Err(anyhow!("invalid step: {s}")),
        }
    }

    /// Returns the hash value of the step.
    fn hash(&self) -> u8 {
        hash(self.label.bytes().chain(self.operation.bytes()))
    }
}

impl fmt::Display for Step<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.operation {
            Operation::Dash => write!(f, "{}-", self.label),
            Operation::Equal(value) => write!(f, "{}={value}", self.label),
        }
    }
}

#[derive(Debug)]
struct InitializationSequence<'a>(Vec<Step<'a>>);

impl<'a> InitializationSequence<'a> {
    /// Parses an initialization sequence from a string.
    fn parse(s: &'a str) -> Result<InitializationSequence<'a>, Error> {
        Ok(InitializationSequence(
            s.trim_end_matches('\n')
                .split(',')
                .map(Step::parse)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    /// Returns the sum of the hash values of the steps.
    fn sum_hash(&self) -> u32 {
        self.0.iter().map(|step| u32::from(step.hash())).sum()
    }

    /// Run the initialization sequence to create a lens configuration.
    fn lens_configuration(&self) -> LensConfiguration<'_> {
        let mut config = [const { Vec::<(&str, u8)>::new() }; 256];
        for step in &self.0 {
            let slots = &mut config[usize::from(hash(step.label.bytes()))];
            let label_position = slots.iter().position(|(label, _)| *label == step.label);
            match step.operation {
                Operation::Dash => {
                    if let Some(position) = label_position {
                        slots.remove(position);
                    }
                }
                Operation::Equal(focal_length) => {
                    if let Some(position) = label_position {
                        slots[position].1 = focal_length;
                    } else {
                        slots.push((step.label, focal_length));
                    }
                }
            }
        }
        LensConfiguration(config)
    }
}

impl fmt::Display for InitializationSequence<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for step in &self.0 {
            if first {
                first = false;
            } else {
                f.write_char(',')?;
            }
            write!(f, "{step}")?;
        }
        Ok(())
    }
}

struct LensConfiguration<'a>([Vec<(&'a str, u8)>; 256]);

impl LensConfiguration<'_> {
    /// Returns the focusing power of the lens configuration.
    fn focusing_power(&self) -> u32 {
        let mut power = 0;
        for (box_number, slots) in (1u32..).zip(self.0.iter()) {
            for (slot_number, &(_, focal_length)) in (1u32..).zip(slots.iter()) {
                power += box_number * slot_number * u32::from(focal_length);
            }
        }
        power
    }
}

impl fmt::Debug for LensConfiguration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (box_number, slots) in self.0.iter().enumerate() {
            if slots.is_empty() {
                continue;
            }
            write!(f, "Box {box_number}:")?;
            for &(label, focal_length) in slots {
                write!(f, " [{label} {focal_length}]")?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> Result<()> {
    let sequence = InitializationSequence::parse(input)?;

    println!("Part 1: {}", sequence.sum_hash());
    println!("Part 2: {}", sequence.lens_configuration().focusing_power());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    #[test]
    fn sample() {
        let sequence = InitializationSequence::parse(SAMPLE_INPUT).unwrap();
        assert_eq!(sequence.sum_hash(), 1320);
        assert_eq!(sequence.lens_configuration().focusing_power(), 145);
    }
}
