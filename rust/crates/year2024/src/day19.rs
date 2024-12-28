use std::iter::Zip;
use std::ops::{Deref, RangeFrom};
use std::str::Bytes;

use anyhow::Result;

fn color_index(color: u8) -> usize {
    match color {
        b'w' => 0,
        b'u' => 1,
        b'b' => 2,
        b'r' => 3,
        b'g' => 4,
        _ => panic!("Unrecognized color character: {}", color as char),
    }
}

/// A node in the trie data structure.
#[derive(Debug, Default)]
struct TrieNode {
    /// The children of the node where the index is the color character while the value at the
    /// index is the pointer to the child node.
    children: [usize; 5],

    /// Whether the node is a terminal node i.e., the end of a pattern.
    is_terminal: bool,
}

/// A specialized trie data structure for storing towel color patterns.
///
/// It is specialized in that it only supports the colors `w`, `u`, `b`, `r`, and `g`.
#[derive(Debug)]
struct Trie {
    nodes: Vec<TrieNode>,
}

impl Default for Trie {
    fn default() -> Self {
        Self {
            nodes: vec![TrieNode::default()],
        }
    }
}

impl Trie {
    /// Create a new trie from a list of `patterns`.
    fn from_patterns<'a, I>(patterns: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut trie = Self::default();
        for pattern in patterns {
            trie.insert(pattern);
        }
        trie
    }

    /// Insert a new `pattern` into the trie.
    ///
    /// This only recognizes the letters `w`, `u`, `b`, `r`, and `g`.
    ///
    /// # Panics
    ///
    /// Panics if the `pattern` contains an unrecognized character.
    fn insert(&mut self, pattern: &str) {
        let mut current = 0;
        for color in pattern.bytes() {
            let index = color_index(color);
            if self.nodes[current].children[index] == 0 {
                self.nodes.push(TrieNode::default());
                self.nodes[current].children[index] = self.nodes.len() - 1;
            }
            current = self.nodes[current].children[index];
        }
        self.nodes[current].is_terminal = true;
    }

    /// Look for the common prefixes of `query` and return an iterator over their lengths.
    ///
    /// This only recognizes the letters `w`, `u`, `b`, `r`, and `g`.
    ///
    /// # Panics
    ///
    /// Panics if the `query` contains an unrecognized character.
    fn common_prefix_length<'c, 't>(&'t self, query: &'c str) -> PrefixLengthIter<'c, 't> {
        PrefixLengthIter {
            trie: self,
            bytes: (1usize..).zip(query.bytes()),
            current: 0,
        }
    }
}

struct PrefixLengthIter<'c, 't> {
    trie: &'t Trie,
    bytes: Zip<RangeFrom<usize>, Bytes<'c>>,
    current: usize,
}

impl Iterator for PrefixLengthIter<'_, '_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (length, color) = self.bytes.next()?;
            let index = color_index(color);
            self.current = self.trie.nodes[self.current].children[index];
            if self.current == 0 {
                return None;
            } else if self.trie.nodes[self.current].is_terminal {
                return Some(length);
            }
        }
    }
}

#[derive(Debug)]
struct TowelDesign<'a>(&'a str);

impl TowelDesign<'_> {
    fn arrangement_count(&self, patterns: &Trie) -> usize {
        fn inner(remaining: &str, patterns: &Trie, cache: &mut [usize]) -> usize {
            if remaining.is_empty() {
                return 1;
            }
            let cached = cache[remaining.len() - 1];
            if cached != usize::MAX {
                return cached;
            }
            let mut count = 0;
            for length in patterns.common_prefix_length(remaining) {
                count += inner(&remaining[length..], patterns, cache);
            }
            cache[remaining.len() - 1] = count;
            count
        }

        let mut cache = vec![usize::MAX; self.len()];
        inner(self, patterns, &mut cache)
    }
}

impl Deref for TowelDesign<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug)]
struct Onsen<'a> {
    patterns: Trie,
    designs: Vec<TowelDesign<'a>>,
}

impl Onsen<'_> {
    /// Returns a tuple of two count values where:
    /// 1. Number of designs that are possible
    /// 2. Total number of possible arrangements for all designs
    fn count(&self) -> (usize, usize) {
        self.designs
            .iter()
            .fold((0, 0), |(possible, total), design| {
                let count = design.arrangement_count(&self.patterns);
                (possible + usize::from(possible > 0), total + count)
            })
    }
}

impl<'a> From<&'a str> for Onsen<'a> {
    fn from(value: &'a str) -> Self {
        let (first, second) = value.split_once("\n\n").unwrap();
        Self {
            patterns: Trie::from_patterns(first.split(", ")),
            designs: second.lines().map(TowelDesign).collect(),
        }
    }
}

pub fn solve(input: &str) -> Result<()> {
    let (possible, total) = Onsen::from(input).count();

    println!("Part 1: {possible}");
    println!("Part 1: {total}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn sample() {
        let (possible, total) = Onsen::from(SAMPLE_INPUT).count();
        assert_eq!(possible, 6);
        assert_eq!(total, 16);
    }
}
