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
        N: i64,
    }

    let m = 1_000_001;
    let mut sieve = vec![true;m as usize];
    for i in 2..m{
        if sieve[i]{
            let mut k = 2 * i;
            while k < m {
                sieve[k] = false;
                k += i;
            }
        }
    }

    let mut p = vec![];
    for i in 2..m{
        if sieve[i] {
            p.push(i as i64);
        }
    }

    let mut ans = 0;
    for &q in p.iter(){
        let qqq = q * q * q;
        let b = (N / qqq).min(q-1);
        ans += p.upper_bound(&b);
    }

    println!("{}", ans);

    
}
