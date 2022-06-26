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
    }

    if N % 2 == 1 {
        return;
    }

    let mut ans = vec![];

    for i in 0..1 << N {
        let mut a = 0;
        let mut b = 0;
        let mut res = true;
        for j in 0..N {
            if (i >> j) & 1 == 0 {
                a += 1;
            } else {
                b += 1;
            }
            if b > a || (j == N - 1 && a != b) {
                res = false;
                break;
            }
        }
        if res {
            let s = (0..N)
                .map(|j| if (i >> j) & 1 == 0 { '(' } else { ')' })
                .collect::<String>();
            ans.push(s);
        }
    }

    ans.sort();
    for a in ans.iter(){
        println!("{}", a);
    }
}
