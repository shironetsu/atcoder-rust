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
        A:[i64;N],
    }

    let all = A.iter().sum::<i64>();
    let AA = A.iter().copied().chain(A.iter().copied()).collect_vec();
    let mut l = 0;
    let mut r = 0;
    let mut sum = 0;
    //let mut ans = 0;
    loop{
        while sum * 10 < all {
            sum += AA[l];
            l += 1;
        }
        if sum * 10 == all {
            println!("Yes");
            return;
        }
        while sum * 10 >= all {
            sum -= AA[r];
            r += 1;
            if r >= N {
                println!("No");
                return;
            }
        }
    }
}
