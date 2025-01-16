use anyhow::{anyhow, bail, Context, Error, Result};
use aoc_lib::geom::point2;
use aoc_lib::matrix::CardinalDirection;

/// An instruction for the digger.
#[derive(Debug)]
struct Instruction {
    /// The direction to dig.
    direction: CardinalDirection,
    /// The number of steps to dig in the given direction.
    distance: u32,
}

/// A plan for the digger.
///
/// Invariant: The number of instructions and colors should be the same.
#[derive(Debug)]
struct DigPlan<'a> {
    /// The instructions for the digger.
    instructions: Vec<Instruction>,
    /// The colors corresponding to each instruction.
    colors: Vec<&'a str>,
}

impl<'a> DigPlan<'a> {
    /// Parses the given string into a dig plan.
    ///
    /// Returns an error if the string is not in the expected format.
    fn parse(s: &'a str) -> Result<DigPlan<'a>, Error> {
        let mut instructions = Vec::new();
        let mut colors = Vec::new();

        for line in s.lines() {
            let mut words = line.split_ascii_whitespace();

            let direction = if let Some(direction) = words.next() {
                match direction {
                    "U" => CardinalDirection::Up,
                    "R" => CardinalDirection::Right,
                    "D" => CardinalDirection::Down,
                    "L" => CardinalDirection::Left,
                    _ => bail!("Invalid direction letter: {direction}"),
                }
            } else {
                bail!("Invalid instruction: {s} (missing direction)");
            };

            let distance = if let Some(distance) = words.next() {
                distance
                    .parse()
                    .with_context(|| format!("Failed to parse distance: {distance}"))?
            } else {
                bail!("Invalid instruction: {s} (missing distance)");
            };

            let color = words
                .next()
                .ok_or_else(|| anyhow!("Invalid instruction: {s} (missing color)"))?
                .trim_start_matches("(#")
                .trim_end_matches(')');

            instructions.push(Instruction {
                direction,
                distance,
            });
            colors.push(color);
        }

        Ok(DigPlan {
            instructions,
            colors,
        })
    }

    /// Fixes the instructions by extracting them from the corresponding colors.
    ///
    /// This method will mutate the instructions in place.
    fn fix_instructions(&mut self) -> Result<()> {
        for (instruction, color) in self.instructions.iter_mut().zip(&self.colors) {
            let (distance, direction) = color.split_at(color.len() - 1);
            instruction.distance = u32::from_str_radix(distance, 16)
                .with_context(|| format!("Failed to parse distance: {distance}"))?;
            instruction.direction = match direction {
                "0" => CardinalDirection::Right,
                "1" => CardinalDirection::Down,
                "2" => CardinalDirection::Left,
                "3" => CardinalDirection::Up,
                _ => return Err(anyhow!("Invalid direction: {direction} (expected 0-3)")),
            };
        }
        Ok(())
    }

    /// Executes the dig plan and returns the cubic meters dug.
    ///
    /// This utilizes the [Pick's theorem] to calculate the total number of interior points in the
    /// polygon formed by the loop and uses the [Shoelace formula] to calculate the area of the
    /// polygon.
    ///
    /// [Pick's theorem]: https://en.wikipedia.org/wiki/Pick%27s_theorem
    /// [Shoelace formula]: https://en.wikipedia.org/wiki/Shoelace_formula
    fn execute(&self) -> i64 {
        let mut perimeter = 0;
        let mut determinant_sum = 0;

        let mut current_position = point2::<i64>(0, 0);
        let mut previous_corner = current_position;

        for instruction in &self.instructions {
            perimeter += i64::from(instruction.distance);
            match instruction.direction {
                CardinalDirection::Up => current_position.y += i64::from(instruction.distance),
                CardinalDirection::Right => current_position.x += i64::from(instruction.distance),
                CardinalDirection::Down => current_position.y -= i64::from(instruction.distance),
                CardinalDirection::Left => current_position.x -= i64::from(instruction.distance),
            }
            determinant_sum += previous_corner.determinant(&current_position);
            previous_corner = current_position;
        }

        // Calculate the area of the polygon using the Shoelace formula.
        let area = determinant_sum.abs() / 2;

        // Modified pick's theorem to calculate the number of interior points:
        //
        // A = i + b/2 - 1 => i = A - b/2 + 1
        let interior = area - perimeter / 2 + 1;

        perimeter + interior
    }
}

pub fn solve(input: &str) -> Result<()> {
    let mut plan = DigPlan::parse(input)?;
    println!("Part 1: {}", plan.execute());

    plan.fix_instructions()?;
    println!("Part 2: {}", plan.execute());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn sample() {
        let mut plan = DigPlan::parse(SAMPLE_INPUT).unwrap();
        assert_eq!(plan.execute(), 62);
        plan.fix_instructions().unwrap();
        assert_eq!(plan.execute(), 952_408_144_115);
    }
}
