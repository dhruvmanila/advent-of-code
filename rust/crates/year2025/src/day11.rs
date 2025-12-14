use std::{collections::HashMap, fmt, str::FromStr};

use anyhow::{Error, Result, bail};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Device(u8, u8, u8);

impl Device {
    const fn you() -> Self {
        Device(b'y', b'o', b'u')
    }

    const fn svr() -> Self {
        Device(b's', b'v', b'r')
    }

    const fn dac() -> Self {
        Device(b'd', b'a', b'c')
    }

    const fn fft() -> Self {
        Device(b'f', b'f', b't')
    }

    const fn out() -> Self {
        Device(b'o', b'u', b't')
    }
}

impl FromStr for Device {
    type Err = Error;

    fn from_str(s: &str) -> Result<Device, Error> {
        let &[b1, b2, b3] = s.as_bytes() else {
            bail!("Device name must be exactly 3 characters: {}", s);
        };
        Ok(Device(b1, b2, b3))
    }
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.0 as char, self.1 as char, self.2 as char)
    }
}

struct Devices {
    connections: HashMap<Device, Vec<Device>>,
}

impl Devices {
    /// Counts the number of distinct paths from the "you" device to the "out" device.
    fn you_to_out_path_count(&self) -> usize {
        fn inner(
            devices: &Devices,
            current: Device,
            visited: &mut HashMap<Device, usize>,
        ) -> usize {
            if current == Device::out() {
                return 1;
            }
            if let Some(&count) = visited.get(&current) {
                return count;
            }
            let mut total_count = 0;
            if let Some(destinations) = devices.connections.get(&current) {
                for &next_device in destinations {
                    total_count += inner(devices, next_device, visited);
                }
            }
            visited.insert(current, total_count);
            total_count
        }

        inner(self, Device::you(), &mut HashMap::new())
    }

    /// Counts the number of distinct paths from the "svr" device to the "out" device such that the
    /// path contains both "dac" and "fft" devices.
    fn svr_to_out_path_count(&self) -> usize {
        fn inner(
            devices: &Devices,
            current: Device,
            visited: &mut HashMap<(Device, bool, bool), usize>,
            has_dac: bool,
            has_fft: bool,
        ) -> usize {
            if current == Device::out() {
                return usize::from(has_dac && has_fft);
            }
            if let Some(&count) = visited.get(&(current, has_dac, has_fft)) {
                return count;
            }
            let mut total_count = 0;
            if let Some(destinations) = devices.connections.get(&current) {
                for &next_device in destinations {
                    let next_has_dac = has_dac || next_device == Device::dac();
                    let next_has_fft = has_fft || next_device == Device::fft();
                    total_count += inner(devices, next_device, visited, next_has_dac, next_has_fft);
                }
            }
            visited.insert((current, has_dac, has_fft), total_count);
            total_count
        }

        inner(self, Device::svr(), &mut HashMap::new(), false, false)
    }
}

impl FromStr for Devices {
    type Err = Error;

    fn from_str(s: &str) -> Result<Devices, Error> {
        let mut connections = HashMap::new();
        for line in s.lines() {
            let Some((source, destinations)) = line.split_once(':') else {
                bail!("invalid line: {line} (expected 'source: dest1 dest2 ...')");
            };
            connections.insert(
                Device::from_str(source)?,
                destinations
                    .split_whitespace()
                    .map(Device::from_str)
                    .collect::<Result<Vec<_>>>()?,
            );
        }
        Ok(Devices { connections })
    }
}

pub fn solve(input: &str) -> Result<()> {
    let devices = Devices::from_str(input)?;

    println!("Part 1: {}", devices.you_to_out_path_count());
    println!("Part 2: {}", devices.svr_to_out_path_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const SAMPLE_INPUT2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn you_to_out_path_count() {
        let devices = Devices::from_str(SAMPLE_INPUT1).unwrap();
        assert_eq!(devices.you_to_out_path_count(), 5);
    }

    #[test]
    fn svr_to_out_path_count() {
        let devices = Devices::from_str(SAMPLE_INPUT2).unwrap();
        assert_eq!(devices.svr_to_out_path_count(), 2);
    }
}
