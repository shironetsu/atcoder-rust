#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

fn f(x: i128) -> i128 {
    x.to_string()
        .chars()
        .map(|x| (x as u8 - '0' as u8) as i128)
        .product()
    // let mut x = x;
    // let mut y = 1;
    // while x > 0 {
    //     let r = x % 10;
    //     y *= r;
    //     x = (x - r) / 10;
    // }
    // y
}

#[fastout]
fn main() {
    input! {
        N: i128,
        B: i128,
    }

    let mut ans = if B.to_string().contains('0') && B <= N {
        1
    } else {
        0
    };

    for i in 0..=33 {
        let a = 2i128.pow(i);
        if a > N {
            continue;
        }
        for j in 0..=22 {
            let b = 3i128.pow(j);
            if a * b > N {
                continue;
            }
            for k in 0..=11 {
                let c = 5i128.pow(k);
                if a * b * c > N {
                    continue;
                }
                for l in 0..=11 {
                    let d = 7i128.pow(l);
                    let n = a * b * c * d;
                    if n > N {
                        continue;
                    }
                    if f(B + n) == n && B + n <= N {
                        ans += 1
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
