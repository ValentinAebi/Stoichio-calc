use std::cmp::min;
use std::f32::consts::E;
use std::fmt::{Display, Formatter};
use crate::arith::{gcd_vec, lcm};
use crate::return_on_error;

#[derive(Debug, Eq, PartialEq, Clone)]
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

    pub fn at(&self, row: usize, col: usize) -> i32 {
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
fn print_matrix_like(coefs: &Vec<Vec<i32>>){
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

fn place_first_non_zero_at(coefs: &mut Vec<Vec<i32>>, pivot_idx: usize, diag_len: usize) -> bool {
    for r in pivot_idx..diag_len {
        if coefs[r][pivot_idx] != 0 {
            if r > pivot_idx {
                coefs.swap(r, pivot_idx);
            }
            return true;
        }
    }
    false
}

fn simplify_row_if_possible(row: &mut Vec<i32>){
    let gcd = gcd_vec(row);
    for i in 0..row.len() {
        row[i] /= gcd;
    }
}

fn zero_out_row_at_col(coefs: &mut Vec<Vec<i32>>, row_to_zero_out_idx: usize, pivot_idx: usize) {
    if coefs[row_to_zero_out_idx][pivot_idx] != 0 {
        let pivot = coefs[pivot_idx][pivot_idx];
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

fn diagonalize(coefs: &mut Vec<Vec<i32>>, n_rows: usize, n_col: usize) -> bool {
    let diag_len: usize = min(n_rows, n_col);

    for i in 0..coefs.len() {
        simplify_row_if_possible(&mut coefs[i]);
    }

    let zero_out_bottom_left = |coefs: &mut Vec<Vec<i32>>| {
        for pivot_idx in 0..diag_len {
            return_if_false!(place_first_non_zero_at(coefs, pivot_idx, diag_len));
            for r in (pivot_idx + 1)..diag_len {
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

    return_if_false!(zero_out_bottom_left(coefs));
    zero_out_top_right(coefs);
    true
}

