#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

fn bto1(c: char) -> i32{
    if c == '#'{
        1
    } else {
        0
    }
}

#[fastout]
fn main() {
    input!{
        N: usize,
        S: [Chars;N],
    }

    for i in 0..N{
        for j in 0..N-5{
            let mut i = i;
            let mut j = j;
            let mut count = 0;
            for k in 0..6{
                count += bto1(S[i][j+k]);
                if count >= 4 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    for i in 0..N-5 {
        for j in 0..N{
            let mut i = i;
            let mut j = j;
            let mut count = 0;
            for k in 0..6{
                count += bto1(S[i+k][j]);
                if count >= 4 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    for i in 0..N-5 {
        for j in 0..N-5{
            let mut i = i;
            let mut j = j;
            let mut count = 0;
            for k in 0..6{
                count += bto1(S[i+k][j+k]);
                if count >= 4 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    for i in 0..N-5 {
        for j in 5..N{
            let mut i = i;
            let mut j = j;
            let mut count = 0;
            for k in 0..6{
                count += bto1(S[i+k][j-k]);
                if count >= 4 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
    return;
}
