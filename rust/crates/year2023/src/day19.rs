use std::collections::HashMap;
use std::iter::FusedIterator;
use std::ops::Range;
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};

/// The destination to send the part to.
#[derive(Debug)]
enum Destination {
    /// The part is accepted.
    Accepted,
    /// The part is rejected.
    Rejected,
    /// The part is sent to another workflow.
    Workflow(String),
}

impl From<&str> for Destination {
    fn from(s: &str) -> Destination {
        match s {
            "A" => Destination::Accepted,
            "R" => Destination::Rejected,
            _ => Destination::Workflow(s.to_owned()),
        }
    }
}

/// The category of the rating.
#[derive(Debug, Copy, Clone)]
enum RatingCategory {
    /// Extremely cool looking category.
    X,
    /// Musical category.
    M,
    /// Aerodynamic category.
    A,
    /// Shiny category.
    S,
}

impl TryFrom<u8> for RatingCategory {
    type Error = Error;

    fn try_from(value: u8) -> Result<RatingCategory> {
        match value {
            b'x' => Ok(RatingCategory::X),
            b'm' => Ok(RatingCategory::M),
            b'a' => Ok(RatingCategory::A),
            b's' => Ok(RatingCategory::S),
            _ => bail!("Invalid rating category: {}", value as char),
        }
    }
}

/// The rating of the part.
#[derive(Debug)]
struct Rating {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Rating {
    /// Returns the rating for the given category.
    const fn get(&self, category: RatingCategory) -> u32 {
        match category {
            RatingCategory::X => self.x,
            RatingCategory::M => self.m,
            RatingCategory::A => self.a,
            RatingCategory::S => self.s,
        }
    }

    /// Returns the sum of rating for all categories.
    const fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Rating {
    type Err = Error;

    fn from_str(s: &str) -> Result<Rating> {
        let mut ratings = s
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse::<u32>().ok());

        Ok(Rating {
            x: ratings
                .next()
                .ok_or_else(|| anyhow!("Invalid rating: {s} (missing 'x')"))?,
            m: ratings
                .next()
                .ok_or_else(|| anyhow!("Invalid rating: {s} (missing 'm')"))?,
            a: ratings
                .next()
                .ok_or_else(|| anyhow!("Invalid rating: {s} (missing 'a')"))?,
            s: ratings
                .next()
                .ok_or_else(|| anyhow!("Invalid rating: {s} (missing 's')"))?,
        })
    }
}

/// The rating of the part described as a range.
///
/// The default range is from 1 to 4000 (inclusive) for all categories.
#[derive(Clone, Debug)]
struct RatingRange {
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl RatingRange {
    /// Returns the rating range for the given category.
    const fn get(&self, category: RatingCategory) -> &Range<u32> {
        match category {
            RatingCategory::X => &self.x,
            RatingCategory::M => &self.m,
            RatingCategory::A => &self.a,
            RatingCategory::S => &self.s,
        }
    }

    /// Creates a new rating range by replacing the range for the given category.
    const fn with_category_range(
        mut self,
        category: RatingCategory,
        new_range: Range<u32>,
    ) -> RatingRange {
        match category {
            RatingCategory::X => self.x = new_range,
            RatingCategory::M => self.m = new_range,
            RatingCategory::A => self.a = new_range,
            RatingCategory::S => self.s = new_range,
        }
        self
    }

    /// Returns the number of combinations of the ratings in the range.
    fn combinations(&self) -> u64 {
        self.x.len() as u64 * self.m.len() as u64 * self.a.len() as u64 * self.s.len() as u64
    }
}

impl Default for RatingRange {
    fn default() -> RatingRange {
        RatingRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }
}

/// The operation for the rule condition.
#[derive(Debug, Copy, Clone)]
enum Operation {
    /// The less than (`<`) operation.
    Lt,
    /// The greater than (`>`) operation.
    Gt,
}

impl TryFrom<u8> for Operation {
    type Error = Error;

    fn try_from(value: u8) -> Result<Operation> {
        match value {
            b'<' => Ok(Operation::Lt),
            b'>' => Ok(Operation::Gt),
            _ => bail!("Invalid operation: {}", value as char),
        }
    }
}

/// The rule condition to evaluate the part.
#[derive(Debug)]
struct RuleCondition {
    /// The rating category this condition applies to.
    category: RatingCategory,
    /// The operation to perform.
    operation: Operation,
    /// The value to compare against.
    value: u32,
}

impl RuleCondition {
    /// Evalute the given `part` with this rule condition. Returns `true` if the condition is met.
    const fn evaluate(&self, part: &Rating) -> bool {
        let rating = part.get(self.category);
        match self.operation {
            Operation::Lt => rating < self.value,
            Operation::Gt => rating > self.value,
        }
    }

    /// Evaluate the given `part` range with this rule condition.
    ///
    /// Returns a tuple of two range ratings: the evaluated range (where the condition is met) and
    /// the remaining range (where the condition is not met).
    fn evaluate_range(&self, part: RatingRange) -> (RatingRange, RatingRange) {
        let mut evaluated = None;
        let mut remaining = None;

        let &Range { start, end } = part.get(self.category);
        match self.operation {
            Operation::Lt => {
                if start < self.value {
                    evaluated = Some(start..end.min(self.value));
                }
                if end > self.value {
                    remaining = Some(start.max(self.value)..end);
                }
            }
            Operation::Gt => {
                if end > self.value + 1 {
                    evaluated = Some((self.value + 1).max(start)..end);
                }
                if start <= self.value {
                    remaining = Some(start..(self.value + 1).min(end));
                }
            }
        }

        match (evaluated, remaining) {
            (None, None) => unreachable!(),
            (Some(evaluated), None) => (
                part.clone().with_category_range(self.category, evaluated),
                part,
            ),
            (None, Some(remaining)) => (
                part.clone(),
                part.with_category_range(self.category, remaining),
            ),
            (Some(evaluated), Some(remaining)) => (
                part.clone().with_category_range(self.category, evaluated),
                part.with_category_range(self.category, remaining),
            ),
        }
    }
}

impl FromStr for RuleCondition {
    type Err = Error;

    fn from_str(s: &str) -> Result<RuleCondition, Error> {
        let [category, operation, remaining @ ..] = s.as_bytes() else {
            bail!("Invalid rule condition: {s} (expected at least 3 bytes)");
        };

        let mut value = 0u32;
        for &byte in remaining {
            if !byte.is_ascii_digit() {
                bail!(
                    "Invalid rule condition: {s} (expected digit, found {})",
                    byte as char
                );
            }
            value = value * 10 + (byte - b'0') as u32;
        }

        Ok(RuleCondition {
            category: RatingCategory::try_from(*category)?,
            operation: Operation::try_from(*operation)?,
            value,
        })
    }
}

/// The rule to process the part.
#[derive(Debug)]
struct Rule {
    /// The condition to evaluate the part.
    condition: RuleCondition,
    /// The destination to send the part to if the condition is met.
    destination: Destination,
}

impl Rule {
    /// Process the given `part` with this rule. Returns the destination to send the part to if the
    /// condition is met, otherwise `None`.
    fn process<'a>(&'a self, part: &Rating) -> Option<&'a Destination> {
        if self.condition.evaluate(part) {
            Some(&self.destination)
        } else {
            None
        }
    }

    /// Process the given `part` range with this rule. Returns a tuple of the destination to send
    /// the part to and the evaluated range.
    fn process_range<'a>(
        &'a self,
        part: RatingRange,
    ) -> (&'a Destination, (RatingRange, RatingRange)) {
        (&self.destination, self.condition.evaluate_range(part))
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Rule> {
        let Some((conditions, destination)) = s.split_once(':') else {
            bail!("Invalid rule: {s} (missing ':')");
        };
        Ok(Rule {
            condition: RuleCondition::from_str(conditions)?,
            destination: Destination::from(destination),
        })
    }
}

/// The workflow to process the part.
#[derive(Debug)]
struct Workflow {
    /// The rules to process the part with.
    rules: Vec<Rule>,
    /// The destination to send the part to if no rule is met.
    fallback: Destination,
}

impl Workflow {
    /// Process the given `part` with this workflow. Returns the destination to send the part to.
    fn process<'a>(&'a self, part: &Rating) -> &'a Destination {
        for rule in &self.rules {
            if let Some(destination) = rule.process(part) {
                return destination;
            }
        }
        return &self.fallback;
    }

    /// Process the given `part` range with this workflow. Returns an iterator of the destination
    /// to send the part to and the evaluated range.
    fn process_range<'a>(&'a self, part: RatingRange) -> WorkflowRangeProcessor<'a> {
        WorkflowRangeProcessor {
            rules: self.rules.iter(),
            fallback: Some(&self.fallback),
            remaining: Some(part),
        }
    }
}

/// An iterator to process the part range with the workflow.
struct WorkflowRangeProcessor<'a> {
    rules: std::slice::Iter<'a, Rule>,
    fallback: Option<&'a Destination>,
    remaining: Option<RatingRange>,
}

impl<'a> Iterator for WorkflowRangeProcessor<'a> {
    type Item = (&'a Destination, RatingRange);

    fn next(&mut self) -> Option<(&'a Destination, RatingRange)> {
        let Some(rule) = self.rules.next() else {
            // If there are no rules left, return the fallback destination and the remaining range.
            // Consume both the values to prevent further iteration.
            return Some((self.fallback.take()?, self.remaining.take()?));
        };
        let (destination, (evaluated, remaining)) = rule.process_range(self.remaining.take()?);
        self.remaining = Some(remaining);
        Some((destination, evaluated))
    }
}

impl FusedIterator for WorkflowRangeProcessor<'_> {}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Workflow> {
        let mut rules = s.split(',');
        let fallback =
            Destination::from(rules.next_back().ok_or_else(|| anyhow!("Empty workflow"))?);
        Ok(Workflow {
            rules: rules.map(Rule::from_str).collect::<Result<_>>()?,
            fallback,
        })
    }
}

#[derive(Debug)]
struct Workflows(HashMap<String, Workflow>);

impl Workflows {
    /// Returns `true` if the part is accepted by the workflows.
    fn is_accepted(&self, part: &Rating) -> bool {
        let mut current = "in";

        loop {
            let workflow = self
                .0
                .get(current)
                .unwrap_or_else(|| panic!("missing workflow: {current}"));

            match workflow.process(part) {
                Destination::Accepted => return true,
                Destination::Rejected => return false,
                Destination::Workflow(next) => current = next,
            }
        }
    }

    /// Returns the number of accepted combinations of the part range.
    ///
    /// For example, if the range is `1..3` for all categories, this method will try all possible
    /// combinations of the ratings in the range and return the number of combinations that are
    /// accepted by the workflows.
    fn accepted_combinations(&self, part: RatingRange) -> u64 {
        let mut combinations = 0;
        let mut stack = vec![("in", part)];

        while let Some((current, part)) = stack.pop() {
            let workflow = self
                .0
                .get(current)
                .unwrap_or_else(|| panic!("missing workflow: {current}"));

            for (next, next_part) in workflow.process_range(part) {
                match next {
                    Destination::Accepted => combinations += next_part.combinations(),
                    Destination::Rejected => {}
                    Destination::Workflow(next) => stack.push((next, next_part)),
                }
            }
        }

        combinations
    }
}

impl FromStr for Workflows {
    type Err = Error;

    fn from_str(s: &str) -> Result<Workflows> {
        let mut workflows = HashMap::new();

        for line in s.lines() {
            let (name, rules) = line
                .split_once('{')
                .ok_or_else(|| anyhow!("Invalid workflow: {line} (missing '{{')"))?;
            workflows.insert(
                name.to_owned(),
                Workflow::from_str(rules.trim_end_matches('}'))?,
            );
        }

        Ok(Workflows(workflows))
    }
}

/// The system of workflows and parts.
#[derive(Debug)]
struct System {
    workflows: Workflows,
    parts: Vec<Rating>,
}

impl System {
    /// Returns the sum of the ratings for all the parts that are accepted by the workflows.
    fn sum_accepted_parts_rating(&self) -> u32 {
        self.parts
            .iter()
            .filter_map(|part| self.workflows.is_accepted(part).then(|| part.sum()))
            .sum()
    }

    /// Returns the sum of the ratings for all the parts that are accepted by the workflows in the
    /// default rating range.
    ///
    /// The default rating range is from 1 to 4000 (inclusive) for all categories.
    fn sum_accepted_parts_rating_range(&self) -> u64 {
        self.workflows.accepted_combinations(RatingRange::default())
    }
}

impl FromStr for System {
    type Err = Error;

    fn from_str(s: &str) -> Result<System> {
        let (workflows, ratings) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("Invalid system"))?;

        Ok(System {
            workflows: Workflows::from_str(workflows)?,
            parts: ratings
                .lines()
                .map(Rating::from_str)
                .collect::<Result<_>>()?,
        })
    }
}

pub fn solve(input: &str) -> Result<()> {
    let system = System::from_str(input)?;

    println!("Part 1: {}", system.sum_accepted_parts_rating());
    println!("Part 2: {}", system.sum_accepted_parts_rating_range());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn sample() {
        let system = System::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(system.sum_accepted_parts_rating(), 19114);
        assert_eq!(
            system.sum_accepted_parts_rating_range(),
            167_409_079_868_000
        );
    }
}
