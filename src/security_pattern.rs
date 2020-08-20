use rayon::prelude::*;

use crate::matrix::SquareMatrix;

#[derive(Clone, Debug)]
pub struct SecurityPattern {
    e: SquareMatrix,
    n: usize,
}

impl SecurityPattern {
    pub fn new(n: usize) -> Self {
        // n x n vertices
        let mut e = SquareMatrix::new(n);
        for i in 0..n {
            for j in 0..n {
                e.set(i, j, 1);
            }
        }
        for i in 0..n {
            for j in 0..n {
                let pos = (i * n) + j;
                for k in 0..n {
                    let pos2 = (i * n) + k;
                    if pos2 != pos && (pos > 0 && pos - 1 != pos2) && pos + 1 != pos2 {
                        e.set(pos, pos2, 0);
                        e.set(pos2, pos, 0);
                    }
                }
                for k in 0..n {
                    let pos2 = (k * n) + j;
                    if pos2 != pos && (pos >= n && pos - n != pos2) && pos + n != pos2 {
                        e.set(pos, pos2, 0);
                        e.set(pos2, pos, 0);
                    }
                }
                let (mut row_t, mut col_t) = (i, j);
                while row_t > 0 && col_t > 0 {
                    row_t -= 1;
                    col_t -= 1;
                }
                while row_t < n && col_t < n {
                    let pos2 = (row_t * n) + col_t;
                    if (i > 0 && row_t != i - 1) && row_t != i + 1 && row_t != i {
                        e.set(pos, pos2, 0);
                        e.set(pos2, pos, 0);
                    }
                    row_t += 1;
                    col_t += 1;
                }
                row_t = 0;
                col_t = 0;
                while row_t > 0 && col_t < n {
                    row_t -= 1;
                    col_t += 1;
                }
                while row_t < n && col_t > 0 {
                    let pos2 = (row_t * n) + col_t;
                    if (i > 0 && row_t != i - 1) && row_t != i + 1 && row_t != i {
                        e.set(pos, pos2, 0);
                        e.set(pos2, pos, 0);
                    }
                    row_t += 1;
                    col_t -= 1;
                }
            }
        }
        Self { e, n }
    }

    pub fn adj_matrix(&self) -> SquareMatrix {
        self.e.clone()
    }
}
