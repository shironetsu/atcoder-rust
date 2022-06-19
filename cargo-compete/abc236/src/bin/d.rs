#![allow(unused_imports)]
#![allow(non_snake_case)]
use maplit::*;
use permutohedron::LexicalPermutation;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;

pub trait Change<T: PartialOrd> {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}
impl<T: PartialOrd> Change<T> for T {
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut A = vec![vec![0; 2 * N]; 2 * N];
    for i in 0..2 * N {
        for j in i + 1..2 * N {
            input! {
                a: u64,
            }
            A[i][j] = a;
            A[j][i] = a;
        }
    }

    let mut bmax = 0;

    for s in 0..(1 << (2 * N)) {
        let mut zeros = vec![];
        let mut ones = vec![];
        for i in 0..2 * N {
            if (s >> i) & 1 == 1 {
                ones.push(i);
            } else {
                zeros.push(i);
            }
            if zeros.len() < ones.len() {
                break;
            }
        }
        if !(zeros.len() == N && ones.len() == N) {
            continue;
        }

        loop {
            let mut b = 0;
            let valid = zeros
                .iter()
                .zip(ones.iter())
                .fold(true, |acc, (x, y)| acc && (x < y));
            if valid {
                for i in 0..N {
                    b ^= A[zeros[i]][ones[i]];
                }

                bmax.chmax(b);
            }

            if !ones.next_permutation() {
                break;
            }
        }
    }

    println!("{}", bmax);
}
