use std::fmt::Write;
use std::str::FromStr;

use anyhow::{Context, Error, Result, anyhow};
use aoc_lib::matrix::Position;

/// A vertical edge of an [`OrthogonalPolygon`].
#[derive(Debug)]
struct VerticalEdge {
    col: usize,
    // `row_start` < `row_end`
    row_start: usize,
    row_end: usize,
}

/// A horizontal edge of an [`OrthogonalPolygon`].
#[derive(Debug)]
struct HorizontalEdge {
    row: usize,
    // `col_start` < `col_end`
    col_start: usize,
    col_end: usize,
}

/// An orthogonal polygon is a polygon whose edges are either horizontal or vertical lines.
///
/// Internally, this is represented as a list of vertices. The edges of the polygon are
/// formed by connecting each vertex to the next one in the list, and the last vertex connects
/// back to the first one. This means that the polygon is closed and does not intersect itself.
///
/// This invariant is enforced by the [`OrthogonalPolygonBuilder`], which will panic if the new
/// position is not connected to the last position in an orthogonal way and if the first
/// position is not connected to the last position in an orthogonal way.
#[derive(Debug)]
struct OrthogonalPolygon {
    vertices: Vec<Position>,
    /// Vertical edges sorted by column for fast rectangle intersection queries.
    vertical_edges: Vec<VerticalEdge>,
    /// Horizontal edges sorted by row for fast rectangle intersection queries.
    horizontal_edges: Vec<HorizontalEdge>,
}

impl OrthogonalPolygon {
    /// Returns an iterator over all unique pairs of vertices in the polygon.
    fn pairs(&self) -> impl Iterator<Item = (&Position, &Position)> {
        self.vertices
            .iter()
            .enumerate()
            .flat_map(move |(index, pos1)| {
                self.vertices
                    .get(index + 1..)
                    .unwrap_or(&[])
                    .iter()
                    .map(move |pos2| (pos1, pos2))
            })
    }

    /// Checks if an axis-aligned rectangle is entirely inside (or on the boundary of) the polygon.
    ///
    /// The rectangle is defined by two opposite corners, which are assumed to be vertices of
    /// the polygon (and thus already on the boundary).
    ///
    /// This method checks that no edge of the polygon passes through the interior of the rectangle.
    /// If any edge does, the rectangle is not entirely inside the polygon.
    fn contains_rectangle(&self, corner1: &Position, corner2: &Position) -> bool {
        // These variables define the rectangle formed by the given corners
        let start_row = corner1.row().min(corner2.row());
        let end_row = corner1.row().max(corner2.row());
        let start_col = corner1.col().min(corner2.col());
        let end_col = corner1.col().max(corner2.col());

        // Find the vertical edges that lies between the rectange's left and right sides and check
        // if any of them overlap with the rectangle's top and bottom sides.
        let start_idx = self
            .vertical_edges
            .partition_point(|edge| edge.col <= start_col);
        for edge in &self.vertical_edges[start_idx..] {
            if edge.col >= end_col {
                break;
            }
            if edge.row_start < end_row && edge.row_end > start_row {
                return false;
            }
        }

        // Find the horizontal edges that lies between the rectangle's top and bottom sides and
        // check if any of them overlap with the rectangle's left and right sides.
        let start_idx = self
            .horizontal_edges
            .partition_point(|edge| edge.row <= start_row);
        for edge in &self.horizontal_edges[start_idx..] {
            if edge.row >= end_row {
                break;
            }
            if edge.col_start < end_col && edge.col_end > start_col {
                return false;
            }
        }

        true
    }

    /// Generates an SVG representation of the polygon, fitting it within the given viewport size.
    #[allow(dead_code)]
    fn to_svg(&self, viewport_size: usize) -> String {
        let min_row = self.vertices.iter().map(|v| v.row()).min().unwrap_or(0);
        let max_row = self.vertices.iter().map(|v| v.row()).max().unwrap_or(0);
        let min_col = self.vertices.iter().map(|v| v.col()).min().unwrap_or(0);
        let max_col = self.vertices.iter().map(|v| v.col()).max().unwrap_or(0);

        let data_width = (max_col - min_col).max(1) as f64;
        let data_height = (max_row - min_row).max(1) as f64;

        // Scale to fit viewport with padding
        let padding = 40.0;
        let available = viewport_size as f64 - padding * 2.0;
        let scale = (available / data_width).min(available / data_height);

        let width = viewport_size;
        let height = viewport_size;

        // Helper to convert polygon coords to SVG coords (centered)
        let offset_x = (viewport_size as f64 - data_width * scale) / 2.0;
        let offset_y = (viewport_size as f64 - data_height * scale) / 2.0;
        let to_svg_x = |col: usize| (col - min_col) as f64 * scale + offset_x;
        let to_svg_y = |row: usize| (row - min_row) as f64 * scale + offset_y;

        let mut svg = String::new();

        // SVG header
        writeln!(
            &mut svg,
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">"#
        ).unwrap();

        // Background
        svg.push_str(r##"  <rect width="100%" height="100%" fill="#f0f0f0"/>"##);
        svg.push('\n');

        // Draw filled polygon
        let points: String = self
            .vertices
            .iter()
            .map(|v| format!("{:.1},{:.1}", to_svg_x(v.col()), to_svg_y(v.row())))
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(
            &mut svg,
            r#"  <polygon points="{points}" fill="lightgreen" stroke="green" stroke-width="2"/>"#
        )
        .unwrap();

        svg.push_str("</svg>\n");
        svg
    }
}

/// A builder for creating an [`OrthogonalPolygon`].
///
/// This builder enforces the invariant that each new position added must be on the same row
/// or column as the last position, and that the first and last positions must also be on the same
/// row or column.
struct OrthogonalPolygonBuilder {
    vertices: Vec<Position>,
}

impl OrthogonalPolygonBuilder {
    /// Creates a new, empty orthogonal polygon.
    fn new() -> Self {
        OrthogonalPolygonBuilder {
            vertices: Vec::new(),
        }
    }

    /// Push a new position onto the polygon.
    ///
    /// # Panics
    ///
    /// Panics if the new position is not on the same row or column as the last position.
    fn push(&mut self, position: Position) {
        if let Some(last) = self.vertices.last()
            && (last.row() != position.row() && last.col() != position.col())
        {
            panic!(
                "new position {position:?} is not on the same row or column as the \
                    last position {last:?}",
            );
        }
        self.vertices.push(position);
    }

    /// Finish building the polygon.
    ///
    /// # Panics
    ///
    /// Panics if the first position is not on the same row or column as the last position.
    fn finish(self) -> OrthogonalPolygon {
        if let (Some(first), Some(last)) = (self.vertices.first(), self.vertices.last())
            && first.row() != last.row()
            && first.col() != last.col()
        {
            panic!(
                "first position {first:?} is not on the same row or column as the \
                        last position {last:?}",
            );
        }

        let mut vertical_edges = Vec::new();
        let mut horizontal_edges = Vec::new();

        for (start, end) in self.vertices.iter().enumerate().map(|(index, start)| {
            // The `%` handles the final vertex connecting back to the first vertex as `index + 1`
            // for the last vertex would be equal to `self.vertices.len()` where the modulus wraps
            // it back to `0`.
            (start, &self.vertices[(index + 1) % self.vertices.len()])
        }) {
            if start.col() == end.col() {
                vertical_edges.push(VerticalEdge {
                    col: start.col(),
                    row_start: start.row().min(end.row()),
                    row_end: start.row().max(end.row()),
                });
            } else {
                horizontal_edges.push(HorizontalEdge {
                    row: start.row(),
                    col_start: start.col().min(end.col()),
                    col_end: start.col().max(end.col()),
                });
            }
        }

        vertical_edges.sort_by_key(|e| e.col);
        horizontal_edges.sort_by_key(|e| e.row);

        OrthogonalPolygon {
            vertices: self.vertices,
            vertical_edges,
            horizontal_edges,
        }
    }
}

/// A grid containing red tiles, represented by their positions.
#[derive(Debug)]
struct Grid {
    red_tiles: OrthogonalPolygon,
}

impl Grid {
    /// Returns a tuple containing the following:
    /// 1. The area of the largest rectangle that can be formed using two red tiles as opposite
    ///    corners.
    /// 2. The area of the largest rectangle that can be formed using two red tiles as opposite
    ///    corners, such that the rectangle is entirely contained within the polygon formed by
    ///    the red tiles.
    fn largest_rectangle_areas(&self) -> (usize, usize) {
        let mut largest_area1 = 0;
        let mut largest_area2 = 0;

        for (corner1, corner2) in self.red_tiles.pairs() {
            let rows = corner1.row().abs_diff(corner2.row()) + 1;
            let cols = corner1.col().abs_diff(corner2.col()) + 1;
            let area = rows * cols;

            // For part 1, track largest area unconditionally
            if area > largest_area1 {
                largest_area1 = area;
            }

            // We should only check for containment if the area is larger than the current largest
            // area for part 2.
            if area > largest_area2 && self.red_tiles.contains_rectangle(corner1, corner2) {
                largest_area2 = area;
            }
        }

        (largest_area1, largest_area2)
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Grid, Error> {
        let mut builder = OrthogonalPolygonBuilder::new();
        for line in s.lines() {
            let (row, col) = line
                .split_once(',')
                .ok_or_else(|| anyhow!("invalid line: {} (expected row,col)", line))?;
            let position = Position::new(
                row.parse()
                    .with_context(|| format!("invalid row coordinate: {row}"))?,
                col.parse()
                    .with_context(|| format!("invalid column coordinate: {col}"))?,
            );
            builder.push(position);
        }
        Ok(Grid {
            red_tiles: builder.finish(),
        })
    }
}

pub fn solve(input: &str) -> Result<()> {
    let grid = Grid::from_str(input)?;

    // Uncomment to generate SVG visualization:
    // std::fs::write("polygon.svg", grid.red_tiles.to_svg(2400))?;

    let (area1, area2) = grid.largest_rectangle_areas();

    println!("Part 1: {area1}");
    println!("Part 2: {area2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn sample() {
        let grid = Grid::from_str(SAMPLE_INPUT).unwrap();
        let (area1, area2) = grid.largest_rectangle_areas();
        assert_eq!(area1, 50);
        assert_eq!(area2, 24);
    }
}
