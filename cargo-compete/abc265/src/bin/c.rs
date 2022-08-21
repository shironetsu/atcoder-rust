#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        H: usize,
        W: usize,
        G: [Chars;H],
    }

    let mut visited = vec![vec![false;W];H];

    let (mut i, mut j) = (0,0);
    loop {
        if visited[i][j] {
            println!("{}", -1);
            return;
        } else {
            visited[i][j] = true;
        }

        match G[i][j] {
            'U' => {
                if i != 0 {
                    i -= 1;
                } else {
                    break;
                }
            },
            'D' => {
                if i != H - 1 {
                    i += 1;
                } else {
                    break;
                }
            },
            'L' => {
                if j != 0 {
                    j -= 1;
                } else {
                    break;
                }
            },
            'R' => {
                if j != W - 1 {
                    j += 1;
                } else {
                    break;
                }
            },
            _ => unreachable!(),
        }
    }

    println!("{} {}", i+1, j+1);
    
}
