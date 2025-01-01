use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

/// A wire in the monitoring device that is made up of three characters.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Wire(u8, u8, u8);

impl Wire {
    /// Construct a wire with `z` name and the given bit position.
    const fn z_at(pos: u8) -> Wire {
        Wire(b'z', b'0' + (pos / 10), b'0' + (pos % 10))
    }

    /// Check if the wire starts with the given character.
    const fn starts_with(self, ch: char) -> bool {
        self.0 == ch as u8
    }
}

impl PartialEq<&str> for Wire {
    fn eq(&self, other: &&str) -> bool {
        let &[first, second, third] = other.as_bytes() else {
            return false;
        };
        self.0 == first && self.1 == second && self.2 == third
    }
}

impl FromStr for Wire {
    type Err = Error;

    fn from_str(s: &str) -> Result<Wire, Error> {
        let &[first, second, third] = s.as_bytes() else {
            return Err(anyhow!(
                "Expected a wire name to have exactly three characters, got {s}"
            ));
        };
        Ok(Wire(first, second, third))
    }
}

impl fmt::Debug for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.0 as char, self.1 as char, self.2 as char)
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// The kind of logic gate in the monitoring device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GateKind {
    And,
    Or,
    Xor,
}

impl FromStr for GateKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<GateKind, Error> {
        match s {
            "AND" => Ok(GateKind::And),
            "OR" => Ok(GateKind::Or),
            "XOR" => Ok(GateKind::Xor),
            _ => Err(anyhow!("Unrecognized gate kind: {s}")),
        }
    }
}

/// A gate in the monitoring device that connects two wires with a logic gate that connects to an
/// output wire.
#[derive(Debug)]
struct Gate {
    /// The first input wire.
    in1: Wire,
    /// The kind of logic gate.
    kind: GateKind,
    /// The second input wire.
    in2: Wire,
    /// The output wire.
    out: Wire,
}

impl FromStr for Gate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Gate, Error> {
        let mut parts = s.split_whitespace();

        let in1 = parts
            .next()
            .ok_or_else(|| anyhow!("Expected input wire 1"))?
            .parse()?;
        let kind = parts
            .next()
            .ok_or_else(|| anyhow!("Expected gate kind"))?
            .parse()?;
        let in2 = parts
            .next()
            .ok_or_else(|| anyhow!("Expected input wire 2"))?
            .parse()?;
        let _ = parts.next().ok_or_else(|| anyhow!("Expected '->'"))?;
        let out = parts
            .next()
            .ok_or_else(|| anyhow!("Expected output wire"))?
            .parse()?;

        Ok(Gate {
            in1,
            kind,
            in2,
            out,
        })
    }
}

#[derive(Debug)]
struct MonitoringDevice {
    wires: HashMap<Wire, bool>,
    gates: Vec<Gate>,
}

impl MonitoringDevice {
    /// Return a simulator for the monitoring device that can be used to simulate the system of
    /// gates and wires in the device.
    ///
    /// This method will clone the wires as the simulator will modify the values of the wires.
    fn simulator(&self) -> Simulator<'_> {
        Simulator {
            wires: self.wires.clone(),
            gates: &self.gates,
        }
    }

    /// Find the wires that have been swapped in the device.
    ///
    /// The device is a [Ripple Carry Adder].
    ///
    /// [Ripple Carry Adder]: https://en.wikipedia.org/wiki/Ripple-carry_adder
    fn find_swapped_wires(&self) -> SwappedWires {
        let mut swapped = Vec::new();

        // Track all the gates that the wires are connected to.
        let mut output = HashSet::new();
        for gate in &self.gates {
            output.insert((gate.in1, gate.kind));
            output.insert((gate.in2, gate.kind));
        }

        for gate in &self.gates {
            match gate.kind {
                GateKind::And => {
                    // Check that all AND gates point to an OR, except for first AND.
                    if gate.in1 != "x00"
                        && gate.in2 != "x00"
                        && !output.contains(&(gate.out, GateKind::Or))
                    {
                        swapped.push(gate.out);
                    }
                }
                GateKind::Or => {
                    // Check that only XOR gates point to output, except for last carry which is OR.
                    if gate.out.starts_with('z') && gate.out != "z45" {
                        swapped.push(gate.out);
                    }
                    // OR can never point to OR.
                    if output.contains(&(gate.out, GateKind::Or)) {
                        swapped.push(gate.out);
                    }
                }
                GateKind::Xor => {
                    if gate.in1.starts_with('x') || gate.in2.starts_with('x') {
                        // Check that first level XOR points to second level XOR, except for first XOR.
                        if gate.in1 != "x00"
                            && gate.in2 != "x00"
                            && !output.contains(&(gate.out, GateKind::Xor))
                        {
                            swapped.push(gate.out);
                        }
                    } else {
                        // Second level XOR must point to output.
                        if !gate.out.starts_with('z') {
                            swapped.push(gate.out);
                        }
                    }
                }
            }
        }

        swapped.sort_unstable();
        SwappedWires(swapped)
    }

    #[allow(dead_code)]
    fn write_dot_file(&self) -> Result<()> {
        let mut writer = BufWriter::new(File::create("monitoring_device.dot")?);
        writer.write_all(b"digraph monitoring_device {\n")?;

        // Write all nodes and colorize them according to their gate logic
        for gate in &self.gates {
            let color = match gate.kind {
                GateKind::And => "blue",
                GateKind::Or => "yellow",
                GateKind::Xor => "red",
            };
            writer.write_all(
                format!("{} [style=filled,fillcolor={}];\n", gate.out, color).as_bytes(),
            )?;
        }

        // Write all edges
        for gate in &self.gates {
            if gate.kind != GateKind::Or || gate.out.starts_with('z') {
                writer.write_all(format!("{} -> {};\n", gate.in1, gate.out).as_bytes())?;
                writer.write_all(format!("{} -> {};\n", gate.in2, gate.out).as_bytes())?;
            }
        }

        writer.write_all(b"}")?;
        Ok(())
    }
}

impl FromStr for MonitoringDevice {
    type Err = Error;

    fn from_str(s: &str) -> Result<MonitoringDevice, Error> {
        let Some((first, second)) = s.split_once("\n\n") else {
            return Err(anyhow!("Expected two sections separated by two newlines"));
        };

        let mut wires = HashMap::new();
        for line in first.lines() {
            let Some((name, value)) = line.split_once(": ") else {
                return Err(anyhow!(
                    "Expected a wire name and value separated by a colon"
                ));
            };
            wires.insert(name.parse()?, value == "1");
        }

        let gates = second
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Gate>, Error>>()?;

        Ok(MonitoringDevice { wires, gates })
    }
}

/// A simulator for the monitoring device used to simulate the system of gates and wires.
#[derive(Debug)]
struct Simulator<'a> {
    wires: HashMap<Wire, bool>,
    gates: &'a [Gate],
}

impl Simulator<'_> {
    /// Run the simulation until all gates have been executed.
    fn run(&mut self) {
        let mut queue = VecDeque::from_iter(self.gates);

        while let Some(gate) = queue.pop_front() {
            let (Some(in1), Some(in2)) = (
                self.wires.get(&gate.in1).copied(),
                self.wires.get(&gate.in2).copied(),
            ) else {
                queue.push_back(gate);
                continue;
            };
            let out = match gate.kind {
                GateKind::And => in1 & in2,
                GateKind::Or => in1 | in2,
                GateKind::Xor => in1 ^ in2,
            };
            self.wires.insert(gate.out, out);
        }
    }

    /// Return a number by combining the bits on all wires starting with `z`. `z00` is the least
    /// significant bit, then `z01`, then `z02`, and so on. This only supports up to 64 bits.
    fn number(&self) -> u64 {
        let mut number = 0;
        for bit in 0u8..64 {
            let wire = Wire::z_at(bit);
            let Some(&value) = self.wires.get(&wire) else {
                break;
            };
            number |= u64::from(value) << bit;
        }
        number
    }
}

struct SwappedWires(Vec<Wire>);

impl fmt::Display for SwappedWires {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for wire in &self.0 {
            if first {
                first = false;
            } else {
                write!(f, ",")?;
            }
            write!(f, "{wire}")?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> Result<()> {
    let device = MonitoringDevice::from_str(input)?;

    let mut simulator = device.simulator();
    simulator.run();

    println!("Part 1: {}", simulator.number());
    println!("Part 2: {}", device.find_swapped_wires());

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const SAMPLE_INPUT1: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    const SAMPLE_INPUT2: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    #[test_case(SAMPLE_INPUT1, 4)]
    #[test_case(SAMPLE_INPUT2, 2024)]
    fn sample(input: &str, expected: u64) {
        let device = MonitoringDevice::from_str(input).unwrap();
        let mut simulator = device.simulator();
        simulator.run();
        assert_eq!(simulator.number(), expected);
    }
}
