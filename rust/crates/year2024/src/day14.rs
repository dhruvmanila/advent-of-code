use std::fmt;

use anyhow::{anyhow, Result};
use aoc_lib::geom::{point2, vec2, Point2D, Vector2D};
use aoc_lib::matrix::Matrix;
use image::GrayImage;

#[derive(Debug, Clone)]
struct Robot {
    /// The current position of the robot.
    position: Point2D<u32>,
    /// The velocity of the robot.
    velocity: Vector2D<u32>,
}

impl Robot {
    fn step(&mut self, width: u32, height: u32, time: u32) {
        self.position.x = (self.position.x + self.velocity.dx * time) % width;
        self.position.y = (self.position.y + self.velocity.dy * time) % height;
    }
}

/// A coordinate space with multiple robots.
#[derive(Debug)]
struct CoordinateSpace {
    /// The width of the space.
    width: u32,
    /// The height of the space.
    height: u32,
    /// The robots in the space.
    robots: Vec<Robot>,
}

impl CoordinateSpace {
    fn simulate(&mut self, time: u32) {
        for robot in &mut self.robots {
            robot.step(self.width, self.height, time);
        }
    }

    fn safety_factor(&self) -> usize {
        let mid_width = self.width / 2;
        let mid_height = self.height / 2;

        // The four quadrants going clockwise from the top-left.
        //
        // |-------------|
        // |  Q1  |  Q2  |
        // |------+------|
        // |  Q4  |  Q3  |
        // |-------------|
        let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

        for robot in self
            .robots
            .iter()
            .filter(|robot| robot.position.x != mid_width && robot.position.y != mid_height)
        {
            if robot.position.x < mid_width {
                if robot.position.y < mid_height {
                    q1 += 1;
                } else {
                    q4 += 1;
                }
            } else if robot.position.y < mid_height {
                q2 += 1;
            } else {
                q3 += 1;
            }
        }

        q1 * q2 * q3 * q4
    }

    #[allow(dead_code)]
    fn display(&self) -> DisplayCoordinateSpace {
        DisplayCoordinateSpace { space: self }
    }

    fn save_image(&self, n: u32) -> Result<()> {
        let mut image = GrayImage::new(self.width, self.height);
        for robot in &self.robots {
            image.put_pixel(robot.position.x, robot.position.y, image::Luma([255u8]));
        }
        image.save(format!("./crates/year2024/src/day14/{n:04}.png"))?;
        Ok(())
    }
}

struct DisplayCoordinateSpace<'a> {
    space: &'a CoordinateSpace,
}

impl fmt::Display for DisplayCoordinateSpace<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = Matrix::new_with(self.space.height as usize, self.space.width as usize, '.');
        for robot in &self.space.robots {
            grid[(robot.position.y as usize, robot.position.x as usize)] = '#';
        }
        for row in grid.rows() {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

fn parse_input(input: &str, width: u32, height: u32) -> Result<CoordinateSpace> {
    let mut robots = Vec::new();

    for line in input.lines() {
        let mut numbers = line
            .split(|ch: char| !ch.is_ascii_digit() && ch != '-')
            .filter(|word| !word.is_empty());

        let px = numbers
            .next()
            .ok_or_else(|| anyhow!("missing X for position"))?
            .parse::<u32>()?;
        let py = numbers
            .next()
            .ok_or_else(|| anyhow!("missing Y for position"))?
            .parse::<u32>()?;
        let vx = numbers
            .next()
            .ok_or_else(|| anyhow!("missing X for velocity"))?
            .parse::<i64>()?;
        let vy = numbers
            .next()
            .ok_or_else(|| anyhow!("missing Y for velocity"))?
            .parse::<i64>()?;

        robots.push(Robot {
            position: point2(px, py),
            velocity: vec2(
                u32::try_from(vx.rem_euclid(i64::from(width)))?,
                u32::try_from(vy.rem_euclid(i64::from(height)))?,
            ),
        });
    }

    // Similarly, we invert the y-range.
    Ok(CoordinateSpace {
        width,
        height,
        robots,
    })
}

pub fn solve(input: &str) -> Result<()> {
    const WIDTH: u32 = 101;
    const HEIGHT: u32 = 103;

    let mut space = parse_input(input, WIDTH, HEIGHT)?;

    for n in 1..(WIDTH * HEIGHT) {
        space.simulate(1);
        if n == 100 {
            println!("Part 1: {}", space.safety_factor());
        }
        space.save_image(n)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn sample() {
        let mut space = parse_input(SAMPLE_INPUT, 11, 7).unwrap();
        space.simulate(100);
        assert_eq!(space.safety_factor(), 12);
    }
}
