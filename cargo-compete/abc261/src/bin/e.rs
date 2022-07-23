#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

const M : usize = 30;

fn op(t: i32, a :i32 , x: i32)->i32{
    if t == 1 {
        x & a
    } else if t == 2 {
        x | a
    } else {
        x ^ a
    }
}

#[fastout]
fn main() {
    input!{
        N: usize,
        C: i32,
    }

    let mut b = vec![vec![0;M];N+1];
    let mut c = vec![0;N+1];
    b[0] = (0..M).map(|x|1<<x).collect_vec();

    let mut tt = vec![0;N+1];
    let mut aa = vec![0;N+1];

    for i in 1..=N{
        input!{
            t: i32,
            a: i32,
        }
        
        tt[i] = t;
        aa[i] = a;

        for j in 0..M{
            let x = b[i-1][j];
            b[i][j] = op(t, a, x);
        }

        let x = c[i-1];
        c[i] = op(t, a, x);
    }

    // for i in 0..=N{
    //     println!("{:?}", b[i]);
    // }

    // println!("{:?}", c);

    let mut x = C;
    for i in 1..=N{
        let mut y = 0;
        for j in 0..M{
        //for (k, &e) in b[i].iter().enumerate(){
            if (x >> j) & 1 == 1{
                if (b[i][j] >> j) & 1 == 1 {
                    y += 1 << j;
                }
            } else {
                if (c[i] >> j) & 1 == 1 {
                    y += 1 << j;
                }
            }
        }

        x = y;

        println!("{}", x);
    }
}
