// TODO: Add support rayon
// use rayon::prelude::*;

use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

#[derive(Clone, Debug)]
pub struct SquareMatrix {
    data: Vec<Vec<usize>>,
    ord: usize,
}

impl SquareMatrix {
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![vec![0;n];n],
            ord: n,
        }
    }

    pub fn order(&self) -> usize {
        self.ord
    }

    pub fn set(&mut self, i: usize, j: usize, val: usize) {
        self.data[i][j] = val;
    }
}

impl Add for SquareMatrix {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order(), rhs.order());
        let mut res = Self::new(self.order());
        for i in 0..self.order() {
            for j in 0..self.order() {
                res.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        res
    }
}

impl AddAssign for SquareMatrix {
    fn add_assign(&mut self, rhs: Self) {
        let lhs = self.clone();
        *self = lhs + rhs;
    }
}

impl Sub for SquareMatrix {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order(), rhs.order());
        let mut res = Self::new(self.order());
        for i in 0..self.order() {
            for j in 0..self.order() {
                res.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        res
    }
}

impl SubAssign for SquareMatrix {
    fn sub_assign(&mut self, rhs: Self) {
        let lhs = self.clone();
        *self = lhs - rhs;
    }
}

impl Mul for SquareMatrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order(), rhs.order());
        if self.order() == 0 {
            Self::new(0)
        } else if self.order() == 1 {
            Self {
                data: vec![vec![self.data[0][0] * self.data[0][0]]],
                ord: 1,
            }
        } else {
            // Divide-conquer-method (Strassen's algorithm)
            let mut new_ord = 1;
            while new_ord < self.order() {
                new_ord <<= 1;
            }
            let (mut new_a, mut new_b) = (self.clone(), rhs.clone());
            for i in 0..new_ord {
                if new_a.order() < i {
                    new_a.data.push(vec![0;new_ord]);
                    new_b.data.push(vec![0;new_ord]);
                } else {
                    new_a.data[i].resize(new_ord, 0);
                    new_b.data[i].resize(new_ord, 0);
                }
            }
            let (mut sub_a, mut sub_b): (Vec<Vec<SquareMatrix>>, Vec<Vec<SquareMatrix>>) = (vec![vec![SquareMatrix::new(new_ord>>1);2];2], vec![vec![SquareMatrix::new(new_ord>>1);2];2]);
            for i in 0..2 {
                for j in 0..2 {
                    for row in 0..(new_ord>>1) {
                        for col in 0..(new_ord>>1) {
                            sub_a[i][j].data[row][col] = new_a.data[i*(new_ord>>1) + row][j*(new_ord>>1) + col];
                            sub_b[i][j].data[row][col] = new_b.data[i*(new_ord>>1) + row][j*(new_ord>>1) + col];
                        }
                    }
                }
            }
            let m_1 = (sub_a[0][0].clone() + sub_a[1][1].clone())*(sub_b[0][0].clone() + sub_b[1][1].clone());
            let m_2 = (sub_a[1][0].clone() + sub_a[1][1].clone())*(sub_b[0][0].clone());
            let m_3 = sub_a[0][0].clone()*(sub_b[0][1].clone() - sub_b[1][1].clone());
            let m_4 = sub_a[1][1].clone()*(sub_b[1][0].clone() - sub_b[0][0].clone());
            let m_5 = (sub_a[0][0].clone() + sub_a[0][1].clone())*sub_b[1][1].clone();
            let m_6 = (sub_a[1][0].clone() - sub_a[0][0].clone())*(sub_b[0][0].clone() + sub_b[0][1].clone());
            let m_7 = (sub_a[0][1].clone() - sub_a[1][1].clone())*(sub_b[1][0].clone() + sub_b[1][1].clone());
            let mut sub_c = vec![vec![SquareMatrix::new(new_ord>>1);2];2];
            sub_c[0][0] = m_1.clone() + m_4.clone() - m_5.clone() + m_7.clone();
            sub_c[0][1] = m_3.clone() + m_5.clone();
            sub_c[1][0] = m_2.clone() + m_4.clone();
            sub_c[1][1] = m_1.clone() - m_2.clone() + m_3.clone() + m_6.clone();
            let mut c = SquareMatrix::new(new_ord);
            for i in 0..new_ord {
                for j in 0..new_ord {
                    let a = i / (new_ord>>1);
                    let b = j / (new_ord>>1);
                    c.data[i][j] = sub_c[a][b].data[i % (new_ord>>1)][j % (new_ord>>1)];
                }
            }
            c
        }
    }
}

impl MulAssign for SquareMatrix {
    fn mul_assign(&mut self, rhs: Self) {
        let lhs = self.clone();
        *self = lhs * rhs;
    }
}

impl fmt::Display for SquareMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.order() {
            for j in 0..self.order() {
                write!(f, "{} ", self.data[i][j]).ok();
            }
            if i != self.order()-1 {
                write!(f, "\n").ok();
            }
        }
        write!(f, "")
    }
}
