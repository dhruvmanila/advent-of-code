use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::num::{NonZeroU8, ParseIntError};
use std::str::FromStr;

use anyhow::{anyhow, Result};

/// A page number in the safety manual that can range from 1 to 127.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct PageNumber(NonZeroU8);

impl PageNumber {
    /// Create a new page number.
    ///
    /// # Panics
    ///
    /// Panics if the page number is 0.
    fn new(page_number: u8) -> Self {
        assert!(page_number > 0 && page_number < 128);
        Self(NonZeroU8::new(page_number).unwrap())
    }

    /// Returns the contained value as a primitive type.
    fn get(self) -> u8 {
        self.0.get()
    }

    /// Returns the zero-indexed primitive value of the page number.
    fn to_zero_indexed(self) -> u8 {
        self.get() - 1
    }
}

impl fmt::Debug for PageNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl FromStr for PageNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(PageNumber(NonZeroU8::from_str(s)?))
    }
}

fn page_number_ordering(a: PageNumber, b: PageNumber, rules: &OrderingRules) -> Ordering {
    if rules.contains(a, b) {
        Ordering::Less
    } else if rules.contains(b, a) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[derive(Debug, Default, Clone)]
struct Update(Vec<PageNumber>);

impl Update {
    /// Returns the middle page number in the update.
    ///
    /// # Panics
    ///
    /// Panics if the number of page numbers in the update is even or less than 3.
    fn middle(&self) -> PageNumber {
        self.0[self.0.len() / 2]
    }

    /// Returns `true` if the update is sorted as per the given ordering rules.
    fn is_sorted(&self, rules: &OrderingRules) -> bool {
        self.0
            .is_sorted_by(|a, b| page_number_ordering(*a, *b, rules) == Ordering::Less)
    }

    /// Sorts the update page numbers in place as per the given ordering rules.
    fn sort(&mut self, rules: &OrderingRules) {
        self.0.sort_by(|a, b| page_number_ordering(*a, *b, rules));
    }

    /// Returns a new update page numbers sorted by the given ordering rules.
    fn to_sorted(&self, rules: &OrderingRules) -> Update {
        let mut sorted = self.clone();
        sorted.sort(rules);
        sorted
    }
}

impl FromStr for Update {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let update = s
            .split(',')
            .map(PageNumber::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(update))
    }
}

#[derive(Debug, Default)]
struct OrderingRules(HashSet<(PageNumber, PageNumber)>);

impl OrderingRules {
    /// Returns `true` if `(a, b)` is present in the ordering rules.
    fn contains(&self, a: PageNumber, b: PageNumber) -> bool {
        self.0.contains(&(a, b))
    }
}

impl FromStr for OrderingRules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = HashSet::new();
        for line in s.lines() {
            let (left, right) = line
                .split_once('|')
                .ok_or_else(|| anyhow!("Expected a pipe character"))?;
            rules.insert((PageNumber::from_str(left)?, PageNumber::from_str(right)?));
        }
        Ok(Self(rules))
    }
}

#[derive(Debug, Default)]
struct PrintingInstruction {
    rules: OrderingRules,
    updates: Vec<Update>,
}

impl PrintingInstruction {
    /// Returns the sum of the middle page number in the ordered and sorted unordered update.
    fn sum(&self) -> (u32, u32) {
        self.updates
            .iter()
            .fold((0, 0), |(ordered, unordered), update| {
                if update.is_sorted(&self.rules) {
                    (ordered + u32::from(update.middle().get()), unordered)
                } else {
                    (
                        ordered,
                        unordered + u32::from(update.to_sorted(&self.rules).middle().get()),
                    )
                }
            })
    }
}

impl FromStr for PrintingInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (rules_section, updates_section) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("Expected the two sections to be separated by two newlines"))?;

        Ok(Self {
            rules: OrderingRules::from_str(rules_section)?,
            updates: updates_section
                .lines()
                .map(Update::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

pub fn solve(input: &str) -> Result<()> {
    let (ordered, sorted_unordered) = PrintingInstruction::from_str(input)?.sum();

    println!("Part 1: {ordered}");
    println!("Part 2: {sorted_unordered}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn sample() {
        let (ordered, unordered) = PrintingInstruction::from_str(SAMPLE_INPUT).unwrap().sum();
        assert_eq!(ordered, 143);
        assert_eq!(unordered, 123);
    }
}

/// An over-engineered solution that uses bitset to represent a set of numbers and performs various
/// bit operations to check whether the update is sorted and utilizes the topological sorting
/// algorithm.
#[allow(unused)]
mod over_engineered {
    use std::collections::HashMap;
    use std::fmt;
    use std::iter::FusedIterator;
    use std::str::FromStr;

    use anyhow::{anyhow, Result};

    use super::{PageNumber, Update};

    /// A set of page numbers in the safety manual.
    ///
    /// The representation of the set only allows to store page numbers from 1 to 127.
    #[derive(Clone)]
    struct PageNumberSet(u128);

    impl PageNumberSet {
        /// Create an empty set.
        fn empty() -> Self {
            Self(0)
        }

        /// Add a page number to the set.
        fn add(&mut self, page_number: PageNumber) {
            self.0 |= 1u128 << page_number.to_zero_indexed();
        }

        /// Remove a page number from the set if it exists.
        fn remove(&mut self, page_number: PageNumber) {
            self.0 &= !(1u128 << page_number.to_zero_indexed());
        }

        /// Create the set from the given page numbers.
        fn from_page_numbers(page_numbers: &[PageNumber]) -> Self {
            let mut set = Self::empty();
            for page_number in page_numbers {
                set.add(*page_number);
            }
            set
        }

        /// Check if the set contains a page number.
        fn contains(&self, page_number: PageNumber) -> bool {
            self.0 & (1u128 << page_number.to_zero_indexed()) != 0
        }

        /// Return the intersection of two sets.
        fn intersection(&self, other: &PageNumberSet) -> PageNumberSet {
            PageNumberSet(self.0 & other.0)
        }

        /// Return the length of the set.
        fn len(&self) -> usize {
            self.0.count_ones() as usize
        }

        /// Check if the set is empty.
        fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Returns an iterator that yields all the page number that's present in the set.
        fn iter(&self) -> PageNumberSetIterator {
            PageNumberSetIterator { set: self.clone() }
        }
    }

    impl Default for PageNumberSet {
        fn default() -> Self {
            Self::empty()
        }
    }

    struct PageNumberSetIterator {
        set: PageNumberSet,
    }

    impl Iterator for PageNumberSetIterator {
        type Item = PageNumber;

        fn next(&mut self) -> Option<Self::Item> {
            if self.set.is_empty() {
                return None;
            }
            let trailing_zeros = self.set.0.trailing_zeros();
            // The number of bits in the set is 128 which always fits in a u8.
            #[allow(clippy::cast_possible_truncation)]
            let page_number = PageNumber::new(trailing_zeros as u8 + 1);
            self.set.remove(page_number);
            Some(page_number)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.set.len();
            (len, Some(len))
        }
    }

    impl ExactSizeIterator for PageNumberSetIterator {}
    impl FusedIterator for PageNumberSetIterator {}

    impl fmt::Debug for PageNumberSet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut tuple = f.debug_tuple("PageNumberSet");
            for page_number in self.iter() {
                tuple.field(&page_number.get());
            }
            tuple.finish()
        }
    }

    #[derive(Debug, Default)]
    struct OrderingRule {
        before: PageNumberSet,
        after: PageNumberSet,
    }

    impl Update {
        /// Check if the update is ordered according to the rules.
        fn is_ordered(&self, rules: &OrderingRules) -> bool {
            self.0
                .iter()
                .copied()
                .enumerate()
                .all(|(index, page_number)| {
                    rules.get(page_number).map_or(true, |existing_rule| {
                        PageNumberSet::from_page_numbers(&self.0[..index])
                            .intersection(&existing_rule.after)
                            .is_empty()
                            && PageNumberSet::from_page_numbers(&self.0[index + 1..])
                                .intersection(&existing_rule.before)
                                .is_empty()
                    })
                })
        }

        /// Returns a new update with the page numbers sorted according to the rules.
        ///
        /// This utilizes a topological sort algorithm.
        fn topological_sort(&self, rules: &OrderingRules) -> Update {
            let mut sorted = Vec::with_capacity(self.0.len());

            // Set of page numbers in the update, the ones that are added to the sorted vector are
            // removed from this set.
            let mut update_set = PageNumberSet::from_page_numbers(&self.0);

            // Initialize the queue with all the page numbers that doesn't have any page number from
            // the update set coming before it i.e., the node doesn't have any incoming edge.
            let mut queue = self
                .0
                .iter()
                .filter(|page_number| {
                    rules.get(**page_number).map_or(true, |rule| {
                        rule.before.intersection(&update_set).is_empty()
                    })
                })
                .copied()
                .collect::<Vec<_>>();

            while let Some(page_number) = queue.pop() {
                sorted.push(page_number);
                update_set.remove(page_number);

                if let Some(rule) = rules.get(page_number) {
                    // Iterate over all the page numbers that are in the update set and coming after
                    // the current page number.
                    for next in rule.after.intersection(&update_set).iter() {
                        // Add the ones to the queue which doesn't have any page number from the update
                        // set coming before it i.e., no incoming edges.
                        if rules.get(next).map_or(true, |rule| {
                            rule.before.intersection(&update_set).is_empty()
                        }) {
                            queue.push(next);
                        }
                    }
                }
            }

            assert_eq!(
                sorted.len(),
                self.0.len(),
                "Unable to sort because of cycles"
            );

            Update(sorted)
        }
    }

    /// A hashmap of page numbers to corresponding ordering rules.
    #[derive(Debug, Default)]
    struct OrderingRules(HashMap<PageNumber, OrderingRule>);

    impl OrderingRules {
        /// Returns the ordering rule for a page number if it exists.
        fn get(&self, page_number: PageNumber) -> Option<&OrderingRule> {
            self.0.get(&page_number)
        }
    }

    impl FromStr for OrderingRules {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self> {
            let mut rules = HashMap::new();
            for line in s.lines() {
                let (left, right) = line
                    .split_once('|')
                    .ok_or_else(|| anyhow!("Expected a pipe character"))?;

                let before = PageNumber::from_str(left)?;
                let after = PageNumber::from_str(right)?;

                rules
                    .entry(before)
                    .or_insert_with(OrderingRule::default)
                    .after
                    .add(after);
                rules
                    .entry(after)
                    .or_insert_with(OrderingRule::default)
                    .before
                    .add(before);
            }
            Ok(Self(rules))
        }
    }
}
