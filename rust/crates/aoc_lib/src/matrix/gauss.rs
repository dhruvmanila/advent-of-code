//! Gaussian elimination operations for matrices.
//!
//! This module provides methods for performing Gaussian elimination on matrices to obtain Row
//! Echelon Form (REF) and Reduced Row Echelon Form (RREF).
//!
//! The implementation uses `Rational` numbers for exact arithmetic, ensuring that RREF produces
//! proper leading 1s in each pivot position.

use crate::Rational;

use super::Matrix;

impl Matrix<Rational> {
    /// Performs [Gaussian elimination] to obtain [Row Echelon Form] (REF).
    ///
    /// In REF:
    /// - All zero rows are at the bottom
    /// - The leading entry (pivot) of each nonzero row is to the right of the pivot above it
    /// - All entries below each pivot are zero
    ///
    /// The matrix is assumed to be augmented as [A | b], with the last column as the constants.
    ///
    /// [Gaussian elimination]: https://en.wikipedia.org/wiki/Gaussian_elimination
    /// [Row Echelon Form]: https://en.wikipedia.org/wiki/Row_echelon_form
    pub fn row_echelon_form(&mut self) -> GaussianEliminationResult {
        let mut pivot_cols = Vec::new();

        // Current row to consider for pivoting
        let mut pivot_row = 0;

        for col in 0..(self.ncols - 1) {
            // Find a non-zero entry in this column at or below the current pivot row
            let mut non_zero_row = None;
            for row in pivot_row..self.nrows() {
                if !self[(row, col)].is_zero() {
                    non_zero_row = Some(row);
                    break;
                }
            }

            let Some(swap_row) = non_zero_row else {
                continue; // No pivot in this column
            };

            // Swap rows if needed
            if swap_row != pivot_row {
                self.swap_rows(pivot_row, swap_row);
            }

            // Eliminate entries below the pivot in this column i.e., make them zero
            let pivot_value = self[(pivot_row, col)];
            for row in (pivot_row + 1)..self.nrows() {
                let target_value = self[(row, col)];
                if !target_value.is_zero() {
                    let factor = -target_value / pivot_value;
                    self.add_scaled_row(row, pivot_row, factor);
                }
            }

            pivot_cols.push(col);
            pivot_row += 1;

            if pivot_row >= self.nrows() {
                // We've processed all rows which means there are free variables left i.e., columns
                // without pivots.
                break;
            }
        }

        GaussianEliminationResult {
            pivot_cols,
            variable_count: self.ncols - 1,
        }
    }

    /// Performs [Gaussian elimination] to obtain [Reduced Row Echelon Form] (RREF).
    ///
    /// In RREF:
    /// - The matrix is in [Row Echelon Form]
    /// - The leading entry (pivot) in each nonzero row is 1
    /// - Each pivot is the only nonzero entry in its column
    ///
    /// The matrix is assumed to be augmented as [A | b], with the last column as the constants.
    ///
    /// [Gaussian elimination]: https://en.wikipedia.org/wiki/Gaussian_elimination
    /// [Row Echelon Form]: https://en.wikipedia.org/wiki/Row_echelon_form
    /// [Reduced Row Echelon Form]: https://en.wikipedia.org/wiki/Row_echelon_form#Reduced_row_echelon_form
    pub fn reduced_row_echelon_form(&mut self) -> GaussianEliminationResult {
        let result = self.row_echelon_form();

        // Scale each pivot row to make the pivot equal to 1
        for (pivot_row, &pivot_col) in result.pivot_cols.iter().enumerate() {
            let pivot_value = self[(pivot_row, pivot_col)];
            if pivot_value != Rational::ONE {
                for col in 0..self.ncols() {
                    self[(pivot_row, col)] /= pivot_value;
                }
            }
        }

        // Eliminate entries above each pivot (from bottom to top) to make them zero
        for (pivot_row, pivot_col) in result.pivot_positions().rev() {
            for row in 0..pivot_row {
                let target_value = self[(row, pivot_col)];
                if !target_value.is_zero() {
                    // We can directly subtract since the pivot is now 1 from the previous step
                    self.add_scaled_row(row, pivot_row, -target_value);
                }
            }
        }

        result
    }

    /// Adds a multiple of one row to another: `target_row += source_row * factor`.
    fn add_scaled_row(&mut self, target_row: usize, source_row: usize, factor: Rational) {
        for col in 0..self.ncols() {
            self[(target_row, col)] = self[(target_row, col)] + factor * self[(source_row, col)];
        }
    }
}

/// Result of Gaussian elimination containing the pivot column indices.
#[derive(Debug, Clone)]
pub struct GaussianEliminationResult {
    /// Indices of columns that contain pivots (in order of pivot rows).
    ///
    /// The i-th element is the column index of the pivot in row i.
    pivot_cols: Vec<usize>,

    /// Total number of variables (columns excluding the augmented part).
    variable_count: usize,
}

impl GaussianEliminationResult {
    /// Returns the pivot column indices.
    pub fn pivot_cols(&self) -> &[usize] {
        &self.pivot_cols
    }

    /// Returns an iterator over the (row, column) positions of the pivots.
    pub fn pivot_positions(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + '_ {
        self.pivot_cols
            .iter()
            .enumerate()
            .map(|(row, &col)| (row, col))
    }

    /// Returns the free variables (non-pivot) in this result.
    pub fn free_variables(&self) -> Vec<usize> {
        (0..self.variable_count)
            .filter(|col| !self.pivot_cols.contains(col))
            .collect()
    }
}
