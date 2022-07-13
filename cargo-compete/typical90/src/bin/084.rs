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
        S: Chars,
    }
    let mut sub = 0i64;
    let mut c = S[0];
    let mut l = 0;
    let mut r = 0;
    while r < N {
        while r < N && S[r] == c {
            r += 1;
        }

        let h = (r - l) as i64;
        sub += h * (h + 1) / 2;

        if r < N {
            l = r;
            c = S[r];
        } else {
            break;
        }
    }

    let n = N as i64;
    let ans = n * (n + 1) / 2 - sub;

    println!("{}", ans);

    // let mut a = vec![0; N + 1];
    // let mut b = vec![0; N + 1];
    // for i in 0..N {
    //     if S[i] == 'o' {
    //         a[i + 1] = a[i] + 1;
    //         b[i + 1] = b[i];
    //     } else {
    //         a[i + 1] = a[i];
    //         b[i + 1] = b[i] + 1;
    //     }
    // }
    // // println!("{:?}", a);
    // // println!("{:?}", b);

    // let mut sub = 0i64;
    // let mut l = 1;
    // let mut r = 1;
    // loop {
    //     while a[r] == a[l] {
    //         r += 1;
    //         if r == N {
    //             break;
    //         }
    //     }
    //     let h = (r - l) as i64;
    //     sub += h * (h - 1) / 2;
    //     l = r;
    //     if l == N {
    //         break;
    //     }
    // }

    // let mut l = 1;
    // let mut r = 1;
    // loop {
    //     while b[r] == b[l] {
    //         r += 1;
    //         if r == N {
    //             break;
    //         }
    //     }
    //     let h = (r - l) as i64;
    //     sub += h * (h - 1) / 2;
    //     l = r;
    //     if l == N {
    //         break;
    //     }
    // }

    // let n = N as i64;
    // let ans = n * (n + 1) / 2 - sub;

    // println!("{}", ans);
}
