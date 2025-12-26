use std::{
    collections::{HashSet, VecDeque},
    fmt::{self, Write},
    ops::Deref,
    str::FromStr,
};

use anyhow::{Context, Error, Result, bail};
use aoc_lib::{Rational, matrix::Matrix};

/// Represents the indicator light diagram.
///
/// ## Representation
///
/// Internally, these lights are represented as a bitmask within a `u16` with a sentinel bit
/// marking the end of the lights. Using `[.##.#]` as an example:
///
///     .##.#
///  00101101
///    ^
///    |
///    | Sentinel bit
///
/// Here, once the leading zeros are removed, we'd need to skip the sentinel bit to get the actual
/// lights.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct IndicatorLightDiagram(u16);

impl IndicatorLightDiagram {
    /// Create an indicator light diagram completely off with the given number of lights.
    ///
    /// A maximum of 15 lights is supported.
    ///
    /// # Panics
    ///
    /// If `count` is greater than 15.
    const fn off(count: u8) -> IndicatorLightDiagram {
        assert!(count <= 15);
        IndicatorLightDiagram(1 << count)
    }

    /// Return the number of lights in this diagram.
    #[allow(clippy::cast_possible_truncation)] // SAFETY: max 15 lights fits in u16
    const fn count(self) -> u8 {
        15 - self.0.leading_zeros() as u8
    }

    /// Return a new indicator light diagram with the lights toggled according to the given
    /// button wiring schematic.
    ///
    /// # Panics
    ///
    /// If the schematic's count does not match this diagram's count.
    fn toggle(self, schematic: &ButtonWiringSchematic) -> IndicatorLightDiagram {
        assert_eq!(schematic.count(), self.count());
        // XOR cancels both sentinels, so restore it with an OR
        IndicatorLightDiagram(self.0 ^ schematic.mask | (1u16 << self.count()))
    }
}

impl FromStr for IndicatorLightDiagram {
    type Err = Error;

    fn from_str(s: &str) -> Result<IndicatorLightDiagram, Error> {
        let mut lights = 0u16;
        let mut count = 0u8;

        for ch in s.trim_matches(|ch| ch == '[' || ch == ']').chars().rev() {
            match ch {
                '#' => lights |= 1 << count,
                '.' => {}
                _ => bail!("invalid character in indicator light diagram: {}", ch),
            }
            count += 1;
        }

        // Add sentinel bit after the last light position
        lights |= 1 << count;

        Ok(IndicatorLightDiagram(lights))
    }
}

impl fmt::Debug for IndicatorLightDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('[')?;
        for bit in (0..self.count()).rev() {
            f.write_char(if (self.0 >> bit) & 1 == 1 { '#' } else { '.' })?;
        }
        f.write_char(']')
    }
}

/// Represents a button wiring schematic.
#[derive(Clone)]
struct ButtonWiringSchematic {
    indexes: Vec<u8>,
    mask: u16,
}

impl ButtonWiringSchematic {
    fn from_str(s: &str, count: u8) -> Result<ButtonWiringSchematic, Error> {
        let mut schematic = 0u16;
        let mut buttons = Vec::with_capacity(usize::from(count));

        for part in s.trim_matches(|ch| ch == '(' || ch == ')').split(',') {
            let index = part
                .parse()
                .with_context(|| format!("invalid button wiring index: {part:?}"))?;
            if index >= count {
                bail!("button wiring index out of range: {}", index);
            }
            schematic |= 1u16 << (count - 1 - index);
            buttons.push(index);
        }

        // Add sentinel bit after the last position
        schematic |= 1u16 << count;

        Ok(ButtonWiringSchematic {
            mask: schematic,
            indexes: buttons,
        })
    }

    /// Return the number of lights this schematic is wired for.
    #[allow(clippy::cast_possible_truncation)] // SAFETY: max 15 lights fits in u16
    const fn count(&self) -> u8 {
        15 - self.mask.leading_zeros() as u8
    }
}

impl fmt::Debug for ButtonWiringSchematic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('(')?;
        let mut first = true;
        let count = self.count();
        for bit in 0..count {
            if (self.mask >> (count - 1 - bit)) & 1 == 1 {
                if !first {
                    f.write_char(',')?;
                }
                write!(f, "{bit}")?;
                first = false;
            }
        }
        f.write_char(')')
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct JoltageRequirements(Vec<u16>);

impl JoltageRequirements {
    /// Return the number of joltage requirements.
    const fn len(&self) -> usize {
        self.0.len()
    }
}

impl Deref for JoltageRequirements {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for JoltageRequirements {
    type Err = Error;

    fn from_str(s: &str) -> Result<JoltageRequirements, Error> {
        Ok(JoltageRequirements(
            s.trim_matches(|ch| ch == '{' || ch == '}')
                .split(',')
                .map(|part| {
                    part.parse()
                        .with_context(|| format!("invalid joltage requirement: {part:?}"))
                })
                .collect::<Result<Vec<u16>, _>>()?,
        ))
    }
}

impl fmt::Debug for JoltageRequirements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('{')?;
        let mut first = true;
        for value in &self.0 {
            if !first {
                f.write_char(',')?;
            }
            write!(f, "{value}")?;
            first = false;
        }
        f.write_char('}')
    }
}

#[derive(Debug)]
struct Machine {
    indicator_light_diagram: IndicatorLightDiagram,
    button_wiring_schematics: Vec<ButtonWiringSchematic>,
    joltage_requirements: JoltageRequirements,
}

impl Machine {
    /// Compute the fewest button presses required to go from all lights off to the required
    /// indicator light diagram.
    ///
    /// This uses a breadth-first search to find the shortest path. Each state is represented by
    /// the current light diagram and the number of button presses taken to reach it. The fact that
    /// BFS explores all states at a given depth before moving to the next depth ensures that the
    /// first time we reach the target diagram is via the shortest path.
    fn fewest_button_presses_for_indicator_lights(&self) -> usize {
        let off = IndicatorLightDiagram::off(self.indicator_light_diagram.count());

        let mut queue = VecDeque::from([(off, 0usize)]);
        let mut visited = HashSet::from([off]);

        while let Some((lights, presses)) = queue.pop_front() {
            if lights == self.indicator_light_diagram {
                return presses;
            }

            for schematic in &self.button_wiring_schematics {
                let next = lights.toggle(schematic);
                if visited.insert(next) {
                    queue.push_back((next, presses + 1));
                }
            }
        }

        panic!("no solution found");
    }

    /// Compute the fewest button presses required to reach the target joltage requirements.
    ///
    /// Uses Gaussian elimination to reduce the system, then searches over free variables.
    fn fewest_button_presses_for_joltage_requirements(&self) -> usize {
        // E.g., [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        //              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^ constants
        //              number of variables
        //
        // 0 0 0 0 1 1 | 3
        // 0 1 0 0 0 1 | 5
        // 0 0 1 1 1 0 | 4
        // 1 1 0 1 0 0 | 7
        let rows = self.joltage_requirements.len(); // number of equations
        let cols = self.button_wiring_schematics.len(); // number of variables

        // Build an augmented matrix [A | b] as shown above. `cols` excludes the augmented column.
        let mut matrix = Matrix::new_with(rows, cols + 1, Rational::ZERO);

        for (col, buttons) in self.button_wiring_schematics.iter().enumerate() {
            for &row in &buttons.indexes {
                matrix[(row as usize, col)] = Rational::ONE;
            }
        }

        for (row, &joltage) in self.joltage_requirements.iter().enumerate() {
            matrix[(row, cols)] = Rational::from_int(i64::from(joltage));
        }

        let result = matrix.reduced_row_echelon_form();

        self.search_free_variables(&matrix, result.pivot_cols(), &result.free_variables())
    }

    /// Search over free variable combinations to find minimum total button presses.
    fn search_free_variables(
        &self,
        matrix: &Matrix<Rational>,
        pivot_cols: &[usize],
        free_variables: &[usize],
    ) -> usize {
        /// Given free variable values, compute pivot variables and return total if valid.
        ///
        /// Since the matrix is in RREF, each pivot is 1, so for each pivot row:
        ///   `1 * pivot_variable + sum(coefficient * free_variable) = rhs`
        ///   which simplifies to `pivot_variable = rhs - sum(coefficient * free_variable)`
        fn compute_solution(
            matrix: &Matrix<Rational>,
            pivot_cols: &[usize],
            free_variables: &[usize],
            free_variable_values: &[Rational],
        ) -> Option<Rational> {
            let mut total: Rational = free_variable_values.iter().sum();
            let target_col = matrix.ncols() - 1;

            // For each pivot row, solve for the pivot variable
            for pivot_row in 0..pivot_cols.len() {
                let rhs = matrix[(pivot_row, target_col)];

                // pivot_variable = rhs - sum(coefficient * free_variable)
                let mut pivot_variable = rhs;
                for (&free_col, &free_variable_value) in
                    free_variables.iter().zip(free_variable_values)
                {
                    pivot_variable -= matrix[(pivot_row, free_col)] * free_variable_value;
                }

                // Check if solution is a non-negative integer
                if !pivot_variable.is_integer() || pivot_variable.is_negative() {
                    return None;
                }

                total += pivot_variable;
            }

            Some(total)
        }

        fn search_recursive(
            matrix: &Matrix<Rational>,
            pivot_cols: &[usize],
            free_variables: &[usize],
            upper_bounds: &[i64],
            free_variable_values: &mut [Rational],
            depth: usize,
            best: &mut Rational,
        ) {
            if depth == free_variables.len() {
                // All free variables assigned, compute pivot variables
                if let Some(total) =
                    compute_solution(matrix, pivot_cols, free_variables, free_variable_values)
                    && total < *best
                {
                    *best = total;
                }
                return;
            }

            // Current sum of free variables so far
            let free_variables_sum: Rational = free_variable_values[..depth].iter().sum();

            if &free_variables_sum >= best {
                return; // Prune: already exceeded best
            }

            for value in 0..=upper_bounds[depth] {
                free_variable_values[depth] = Rational::from_int(value);
                search_recursive(
                    matrix,
                    pivot_cols,
                    free_variables,
                    upper_bounds,
                    free_variable_values,
                    depth + 1,
                    best,
                );
            }
        }

        // Compute upper bounds for the free variables.
        //
        // This is the minimum number of button presses required to satisfy any of the joltage
        // requirements that depend on that free variable.
        let free_variable_upper_bounds: Vec<i64> = free_variables
            .iter()
            .map(|&col| {
                self.button_wiring_schematics[col]
                    .indexes
                    .iter()
                    .map(|&index| i64::from(self.joltage_requirements[index as usize]))
                    .min()
                    .unwrap_or(0)
            })
            .collect();

        // Initialize free variable values to zero to start the search.
        let mut free_variable_values = vec![Rational::ZERO; free_variables.len()];
        let mut best = Rational::MAX;

        search_recursive(
            matrix,
            pivot_cols,
            free_variables,
            &free_variable_upper_bounds,
            &mut free_variable_values,
            0,
            &mut best,
        );

        let Some(best_value) = best.to_i64() else {
            panic!("non-integer solution found: {best}");
        };

        usize::try_from(best_value).expect("negative best value")
    }
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Machine, Error> {
        let mut parts = s.split_whitespace();
        let Some(diagram_str) = parts.next() else {
            bail!("missing indicator light diagram in {s:?}");
        };
        let indicator_light_diagram = IndicatorLightDiagram::from_str(diagram_str)?;
        let mut parts = parts.rev();
        let Some(joltage_str) = parts.next() else {
            bail!("missing joltage requirements in {s:?}");
        };
        let joltage_requirements = JoltageRequirements::from_str(joltage_str)?;
        assert_eq!(
            joltage_requirements.len(),
            indicator_light_diagram.count() as usize,
            "joltage requirements count does not match indicator light diagram count"
        );
        let button_wiring_schematics = parts
            .rev()
            .map(|part| {
                ButtonWiringSchematic::from_str(part, indicator_light_diagram.count())
                    .with_context(|| format!("invalid button wiring schematic: {part:?}"))
            })
            .collect::<Result<Vec<ButtonWiringSchematic>, _>>()?;
        Ok(Machine {
            indicator_light_diagram,
            button_wiring_schematics,
            joltage_requirements,
        })
    }
}

#[derive(Debug)]
struct Machines(Vec<Machine>);

impl Machines {
    fn sum_fewest_button_presses_for_indicator_lights(&self) -> usize {
        self.0
            .iter()
            .map(Machine::fewest_button_presses_for_indicator_lights)
            .sum()
    }

    fn sum_fewest_button_presses_for_joltage_requirements(&self) -> usize {
        self.0
            .iter()
            .map(Machine::fewest_button_presses_for_joltage_requirements)
            .sum()
    }
}

impl FromStr for Machines {
    type Err = Error;

    fn from_str(s: &str) -> Result<Machines, Error> {
        Ok(Machines(
            s.lines()
                .map(Machine::from_str)
                .collect::<Result<Vec<Machine>, _>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let machines = Machines::from_str(input)?;

    println!(
        "Part 1: {}",
        machines.sum_fewest_button_presses_for_indicator_lights()
    );

    println!(
        "Part 2: {}",
        machines.sum_fewest_button_presses_for_joltage_requirements()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    const SAMPLE_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn sample() {
        let machines = Machines::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(machines.sum_fewest_button_presses_for_indicator_lights(), 7);
        assert_eq!(
            machines.sum_fewest_button_presses_for_joltage_requirements(),
            33
        );
    }

    #[test]
    fn light_indicator_debug() {
        for diagram in ["[.##.]", "[#..#.]", "[###]", "[...]"] {
            let got = format!("{:?}", IndicatorLightDiagram::from_str(diagram).unwrap());
            assert_eq!(got, diagram);
        }
        assert_eq!(format!("{:?}", IndicatorLightDiagram::off(5)), "[.....]");
    }

    #[test_case("(0,2)", 5)]
    #[test_case("(1,3,4)", 5)]
    #[test_case("(2)", 5)]
    #[test_case("(0,1,2,3)", 5)]
    fn button_wiring_schematic_debug(schematic: &str, count: u8) {
        let got = format!(
            "{:?}",
            ButtonWiringSchematic::from_str(schematic, count).unwrap()
        );
        assert_eq!(got, schematic);
    }

    #[test]
    fn toggle() {
        let mut diagram = IndicatorLightDiagram::from_str("[.##..]").unwrap();

        diagram = diagram.toggle(&ButtonWiringSchematic::from_str("(1,3)", 5).unwrap());
        assert_eq!(diagram, IndicatorLightDiagram::from_str("[..##.]").unwrap());

        diagram = diagram.toggle(&ButtonWiringSchematic::from_str("(0,2,4)", 5).unwrap());
        assert_eq!(diagram, IndicatorLightDiagram::from_str("[#..##]").unwrap());
    }
}
