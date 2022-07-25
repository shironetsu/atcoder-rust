#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

pub fn get_primes(N: i64) -> Vec<i64> {
    let mut sieve = vec![true; N as usize + 1];
    let mut primes = vec![];
    for d in 2..=N {
        if !sieve[d as usize] {
            continue;
        }
        primes.push(d);
        for n in (2 * d..=N).step_by(d as usize) {
            sieve[n as usize] = false;
        }
    }
    primes
}

#[fastout]
fn main() {
    input! {
        A: i64,
        B: i64,
        C: i64,
        D: i64,
    }

    let p = get_primes(B + D);
    let p = p.iter().collect::<BTreeSet<_>>();
    let mut found = vec![false;(B-A+1) as usize];
    for x in A..=B{
        let i = (x - A) as usize;
        for y in C..=D{
            if p.contains(&(x+y)) {
                found[i] = true;
            }
        }
    }
    let a = found.into_iter().fold(true, |acc, x|{ acc && x});
    if a {
        println!("Aoki");
    } else {
        println!("Takahashi");
    }
}
