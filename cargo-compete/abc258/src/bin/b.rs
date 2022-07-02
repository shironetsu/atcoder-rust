#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [Chars;N],
    }
    // let A = A
    //     .into_iter()
    //     .map(|row| {
    //         row.into_iter()
    //             .map(|x| (x as u8 - '0' as u8) as i32)
    //             .collect_vec()
    //     })
    //     .collect_vec();

    let mut ans = 0i64;
    for &(dx, dy) in [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]
    .iter()
    {
        for i in 0..N {
            for j in 0..N {
                let mut v = vec![];
                let mut i = i;
                let mut j = j;
                for _ in 0..N {
                    v.push(A[i][j]);
                    //sum += A[i][j];
                    i = ((i as i32 + N as i32 + dx) as usize) % N;
                    j = ((j as i32 + N as i32 + dy) as usize) % N;
                }
                let sum = v.iter().collect::<String>().parse::<i64>().unwrap();
                ans.chmax(sum);
            }
        }
    }
    println!("{}", ans);
}

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
