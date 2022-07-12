use std::cmp::min;
use std::fmt::{Display, Formatter};
use crate::arith::{gcd_vec, lcm};
use crate::return_on_error;

#[derive(Debug, Eq, PartialEq, Clone)]
// self.0 is the bi-dimensional row-major vector containing the coefficients
pub struct Matrix(Vec<Vec<i32>>);

impl Matrix {
    pub fn of_arr(arr: &[&[i32]]) -> Self {
        let mut coefs: Vec<Vec<i32>> = Vec::new();
        for row in arr {
            let row_vec: Vec<i32> = Vec::from(row.clone());
            coefs.push(row_vec);
        }
        Matrix::of_row_major(&coefs)
    }

    pub fn of_row_major(coefs: &Vec<Vec<i32>>) -> Self {
        assert!(!coefs.is_empty());
        let head_row: &Vec<i32> = &coefs[0];
        for row in coefs {
            assert_eq!(row.len(), head_row.len());
        }
        Matrix(coefs.clone())
    }

    pub fn coef_at(&self, row: usize, col: usize) -> i32 {
        self.0[row][col]
    }

    pub fn coefs(&self) -> Vec<Vec<i32>> {
        self.0.clone()
    }

    pub fn n_rows(&self) -> usize {
        self.0.len()
    }

    pub fn n_cols(&self) -> usize {
        self.0[0].len()
    }

    /// Returns an equivalent diagonal matrix, or reports an error
    pub fn diagonalized(&self) -> Result<Matrix, ()> {
        let mut coefs = self.coefs();
        let res = diagonalize(&mut coefs, self.n_rows(), self.n_cols());
        if res { Ok(Matrix(coefs)) } else { Err(()) }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let coefs = &self.0;
        for row in coefs {
            for coef in row {
                return_on_error!(write!(f, "{:4} ", coef));
            }
            return_on_error!(write!(f, "{}", "\n"));
        }
        Ok(())
    }
}

#[allow(dead_code)]
fn print_matrix_like(coefs: &Vec<Vec<i32>>) {
    for row in coefs {
        for coef in row {
            print!("{:4} ", coef);
        }
        println!();
    }
    println!()
}

macro_rules! return_if_false {
    ($e: expr) => {
        let res: bool = $e;
        if !res {
            return false
        }
    }
}

/// Searches the first row s.t. its coef. at index `pivot_idx` is non zero
/// 
/// If found, swaps this row with the one at index `pivot_idx`
/// 
/// Returns `true` if such a row was found or if all the rows starting at `pivot_idx` are 0s, o.w.
/// returns `false` (to report that diagonalizing is impossible)
fn place_first_non_zero_at(coefs: &mut Vec<Vec<i32>>, pivot_idx: usize, n_rows: usize, n_cols: usize) -> bool {
    for r in pivot_idx..n_rows {
        if coefs[r][pivot_idx] != 0 {
            if r > pivot_idx {
                coefs.swap(r, pivot_idx);
            }
            return true;
        }
    }
    // if rows from the pivot row are all 0s then it is ok
    for r in pivot_idx..n_rows {
        for c in 0..n_cols {
            if coefs[r][c] != 0 {
                return false
            }
        }
    }
    true
}

/// Divides all coefficients in the row by their GCD
fn simplify_row_if_possible(row: &mut Vec<i32>) {
    let gcd = gcd_vec(row);
    if gcd != 0 {
        for i in 0..row.len() {
            row[i] /= gcd;
        }
    }
}

/// Uses operations on the rows to zero out `coef[row_to_zero_out_idx][pivot_idx]`
fn zero_out_row_at_col(coefs: &mut Vec<Vec<i32>>, row_to_zero_out_idx: usize, pivot_idx: usize) {
    let pivot = coefs[pivot_idx][pivot_idx];
    if pivot != 0 && coefs[row_to_zero_out_idx][pivot_idx] != 0 {
        let lcm = lcm(coefs[row_to_zero_out_idx][pivot_idx], pivot);
        let row_mul = lcm / coefs[row_to_zero_out_idx][pivot_idx];
        let pivot_row_mul = lcm / pivot;
        for c in 0..coefs[row_to_zero_out_idx].len() {
            coefs[row_to_zero_out_idx][c] =
                row_mul * coefs[row_to_zero_out_idx][c] - pivot_row_mul * coefs[pivot_idx][c];
        }
        simplify_row_if_possible(&mut coefs[row_to_zero_out_idx])
    }
}

/// Uses Gaussian elimination to transform `coefs` into an equivalent diagonal matrix
/// 
/// Returns `true` iff it succeeds
fn diagonalize(coefs: &mut Vec<Vec<i32>>, n_rows: usize, n_cols: usize) -> bool {
    let diag_len: usize = min(n_rows, n_cols);

    for i in 0..coefs.len() {
        simplify_row_if_possible(&mut coefs[i]);
    }

    let zero_out_bottom_left = |coefs: &mut Vec<Vec<i32>>| {
        for pivot_idx in 0..diag_len {
            return_if_false!(place_first_non_zero_at(coefs, pivot_idx, n_rows, n_cols));
            for r in (pivot_idx + 1)..n_rows {
                zero_out_row_at_col(coefs, r, pivot_idx)
            }
        }
        true
    };

    let zero_out_top_right = |coefs: &mut Vec<Vec<i32>>| {
        for pivot_idx in (0..diag_len).rev() {
            for r in 0..pivot_idx {
                zero_out_row_at_col(coefs, r, pivot_idx);
            }
        }
    };

    let make_pivots_non_negative = |coefs: &mut Vec<Vec<i32>>|{
        for pivot_idx in 0..diag_len {
            if coefs[pivot_idx][pivot_idx] < 0 {
                for i in 0..n_cols {
                    coefs[pivot_idx][i] *= -1;
                }
            }
        }
    };

    return_if_false!(zero_out_bottom_left(coefs));
    zero_out_top_right(coefs);
    make_pivots_non_negative(coefs);
    true
}

