#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

fn is_ok(vv: &Vec<Vec<i32>>) -> bool {
    let H = vv.len() as i32;
    let W = vv[0].len() as i32;
    for i in 0..H {
        for j in 0..W {
            let mut a = 0;
            let mut b = 0;
            for &k in [i - 1, i, i + 1].iter() {
                for &l in [j - 1, j, j + 1].iter() {
                    if (i, j) == (k, l) || !(0 <= k && k < H) || !(0 <= l && l < W) {
                        continue;
                    }
                    if vv[k as usize][l as usize] < vv[i as usize][j as usize] {
                        a += 1;
                    } else {
                        b += 1;
                    }
                }
            }
            if a == b {
                return false;
            }
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut ans = vvec![0;N,N];
    let mut dx = 1i32;
    let mut dy = 0i32;
    let mut x = 0;
    let mut y = 0;
    let mut n = 0;
    let mut u = vec![0; N * N];
    let mut a = 1;
    let mut b = (N * N) as i32;

    for i in 0..N * N {
        if i & 1 == 0 {
            u[i] = a;
            a += 1;
        } else {
            u[i] = b;
            b -= 1;
        }
    }

    loop {
        ans[x as usize][y as usize] = u[n];
        //println!("{} {} {} {} {}", x, y, dx, dy, n);
        n += 1;
        if n >= N * N {
            break;
        }
        let s = x + dx;
        let t = y + dy;
        if !(0 <= s && s < N as i32)
            || !(0 <= t && t < N as i32)
            || ans[s as usize][t as usize] != 0
        {
            let tmp = dx;
            dx = -dy;
            dy = tmp;
        }
        x = x + dx;
        y = y + dy;
    }

    println!("{}", is_ok(&ans));

    for i in 0..N {
        let line = ans[i].iter().map(|&x| x.to_string()).join(" ");
        println!("{}", line);
    }
}

#[macro_export]
macro_rules! vvec {
    ($ val : expr ; $ a : expr , $ b : expr ) => {
        vec![vec![$val; $b]; $a]
    };
}
