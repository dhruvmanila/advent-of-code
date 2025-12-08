# Advent of Code (Rust)

## Solution template

```rs
use std::str::FromStr;

use anyhow::{Error, Result, anyhow, bail};

struct ParsedInput;

impl FromStr for ParsedInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<ParsedInput, Error> {
        Err(anyhow!("Not yet implemented"))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let _ = ParsedInput::from_str(input)?;

    Err(anyhow!("Not yet implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
";

    #[test]
    fn sample() {
        let _ = ParsedInput::from_str(SAMPLE_INPUT).unwrap();
    }
}
```
