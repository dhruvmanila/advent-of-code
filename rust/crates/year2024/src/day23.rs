use std::collections::{HashMap, HashSet};
use std::fmt::{self, Write};
use std::ops::Deref;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;

/// Name of a computer in the network, represented as a two-character string.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Computer(u8, u8);

impl Computer {
    /// Returns `true` if this computer belongs to the Chief Historian.
    const fn is_chief_historian(self) -> bool {
        self.0 == b't'
    }
}

impl fmt::Debug for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0 as char, self.1 as char)
    }
}

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Computer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Computer, Error> {
        let &[first, second] = s.as_bytes() else {
            return Err(anyhow!(
                "Expected a computer name to have exactly two characters, got {s}"
            ));
        };
        Ok(Computer(first, second))
    }
}

/// A map of computers in a network, where each computer is connected to a set of other computers.
#[derive(Debug)]
struct NetworkMap(HashMap<Computer, HashSet<Computer>>);

impl NetworkMap {
    /// Returns the number of LAN parties (of three computers) that include the Chief Historian.
    fn lan_party_with_chief_historian(&self) -> usize {
        let mut count = 0;
        let mut seen = HashSet::new();

        for (&c1, connections) in &**self {
            for (&c2, &c3) in connections.iter().tuple_combinations() {
                let mut party = [c1, c2, c3];
                if !party.iter().any(|computer| computer.is_chief_historian()) {
                    continue;
                }
                party.sort_unstable();
                if !seen.insert(party) {
                    continue;
                }
                if self.get(&c2).is_some_and(|conn| conn.contains(&c3))
                    && self.get(&c3).is_some_and(|conn| conn.contains(&c2))
                {
                    count += 1;
                }
            }
        }

        count
    }

    /// Returns the password to get into the LAN party.
    fn password(&self) -> Password {
        // This could be done using https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
        let mut cliques: Vec<HashSet<Computer>> = self
            .keys()
            .map(|&computer| HashSet::from_iter([computer]))
            .collect_vec();

        for clique in &mut cliques {
            for computer in self.keys() {
                let connections = &self[computer];
                if clique
                    .iter()
                    .all(|&computer| connections.contains(&computer))
                {
                    clique.insert(*computer);
                }
            }
        }

        let mut largest = cliques
            .iter()
            .max_by_key(|clique| clique.len())
            .expect("cliques should not be empty")
            .iter()
            .copied()
            .collect_vec();
        largest.sort_unstable();

        Password(largest)
    }
}

impl Deref for NetworkMap {
    type Target = HashMap<Computer, HashSet<Computer>>;

    fn deref(&self) -> &HashMap<Computer, HashSet<Computer>> {
        &self.0
    }
}

impl FromStr for NetworkMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<NetworkMap, Error> {
        let mut map: HashMap<Computer, HashSet<Computer>> = HashMap::new();
        for line in s.lines() {
            let Some((left, right)) = line.split_once('-') else {
                return Err(anyhow!("Expected line to contain a hyphen ('-'): {line:?}"));
            };
            let (left, right) = (Computer::from_str(left)?, Computer::from_str(right)?);
            map.entry(left).or_default().insert(right);
            map.entry(right).or_default().insert(left);
        }
        Ok(NetworkMap(map))
    }
}

/// The password to get into the LAN party.
struct Password(Vec<Computer>);

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for &computer in &self.0 {
            if first {
                first = false;
            } else {
                f.write_char(',')?;
            }
            write!(f, "{computer}")?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> Result<()> {
    let map = NetworkMap::from_str(input)?;

    println!("Part 1: {}", map.lan_party_with_chief_historian());
    println!("Part 2: {}", map.password());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn sample() {
        let map = NetworkMap::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(map.lan_party_with_chief_historian(), 7);
        assert_eq!(map.password().to_string(), "co,de,ka,ta");
    }
}
