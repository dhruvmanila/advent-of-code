use std::fmt;

use crate::matrix::Matrix;

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display().fmt(f)
    }
}

impl<'a, T: fmt::Display> Matrix<T> {
    pub fn display(&'a self) -> DisplayMatrix<'a, T> {
        DisplayMatrix {
            matrix: self,
            settings: DisplaySettings::default(),
        }
    }

    pub fn display_with(&'a self, settings: DisplaySettings) -> DisplayMatrix<'a, T> {
        DisplayMatrix {
            matrix: self,
            settings,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct DisplaySettings {
    with_space: bool,
    augmented: bool,
}

impl DisplaySettings {
    #[must_use]
    pub fn with_space(mut self) -> Self {
        self.with_space = true;
        self
    }

    #[must_use]
    pub fn augmented(mut self) -> Self {
        self.augmented = true;
        self.with_space = true;
        self
    }
}

pub struct DisplayMatrix<'a, T> {
    matrix: &'a Matrix<T>,
    settings: DisplaySettings,
}

impl<T: fmt::Display> fmt::Display for DisplayMatrix<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let col_widths: Vec<usize> = (0..self.matrix.ncols())
            .map(|col| {
                (0..self.matrix.nrows())
                    .map(|row| self.matrix[(row, col)].to_string().len())
                    .max()
                    .unwrap_or(0)
            })
            .collect();

        for row in 0..self.matrix.nrows() {
            let first_row = row == 0;
            let last_row = row + 1 == self.matrix.nrows();

            if self.settings.augmented {
                let bracket = if first_row {
                    '┌'
                } else if last_row {
                    '└'
                } else {
                    '│'
                };
                write!(f, "{bracket} ")?;
            }

            for (col, width) in col_widths.iter().enumerate() {
                if col > 0 {
                    if self.settings.augmented && col + 1 == self.matrix.ncols {
                        write!(f, " │ ")?;
                    } else if self.settings.with_space {
                        write!(f, " ")?;
                    }
                }
                write!(f, "{:>width$}", self.matrix[(row, col)])?;
            }

            if self.settings.augmented {
                let bracket = if first_row {
                    '┐'
                } else if last_row {
                    '┘'
                } else {
                    '│'
                };
                write!(f, " {bracket}")?;
            }

            if !last_row {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_with_space() {
        let matrix: Matrix<i64> = Matrix::from_rows([[1, 2, 3], [4, 5, 6], [7, 8, 9]]).unwrap();
        let displayed = format!(
            "{}",
            matrix.display_with(DisplaySettings::default().with_space())
        );
        let expected = "\
1 2 3
4 5 6
7 8 9";
        assert_eq!(displayed, expected);
    }

    #[test]
    fn display_augmented() {
        let matrix: Matrix<i64> =
            Matrix::from_rows([[1, 22, 3, 10], [14, 5, 634, 11], [7, 8, 9, 122]]).unwrap();
        let displayed = format!(
            "{}",
            matrix.display_with(DisplaySettings::default().augmented())
        );
        let expected = "\
┌  1 22   3 │  10 ┐
│ 14  5 634 │  11 │
└  7  8   9 │ 122 ┘";
        assert_eq!(displayed, expected);
    }
}
