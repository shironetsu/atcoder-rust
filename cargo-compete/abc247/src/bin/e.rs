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
        N: usize,
        X: i64,
        Y: i64,
        A: [i64;N],
    }

    let mut max = vec![];
    let mut min = vec![];
    let mut vio = vec![];
    for (i, &a) in A.iter().enumerate(){
        if a == X {
            max.push(i);
        }
        if a == Y {
            min.push(i);
        }
        if a < Y || X < a {
            vio.push(i);
        }
    }

    let mut ans = 0i64;
    for l in 0..N{
        let a = max.lower_bound(&l);
        let b = min.lower_bound(&l);
        let c = vio.lower_bound(&l);
        if a == max.len() || b == min.len() {
            break;
        }
        let a = max[a];
        let b = min[b];
        let c = if c == vio.len() {
            N
        } else {
            vio[c]
        };
        let rmin = a.max(b);
        //println!("{} {} {} {}",l,a,b,c);
        if rmin <= c {
            ans += (c - rmin) as i64;
        }
    }

    println!("{}", ans);    
}
