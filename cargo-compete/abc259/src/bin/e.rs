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
    let mut m = vec![0usize; N];
    let mut p = vec![vec![]; N];
    let mut e = vec![vec![]; N];
    let mut primes = btreeset!();
    for i in 0..N {
        input! {
            mm: usize,
        }
        m[i] = mm;
        p[i] = vec![0; mm];
        e[i] = vec![0; mm];
        for j in 0..mm {
            input! {
                pp: i32,
                ee: i32,
            }
            p[i][j] = pp;
            e[i][j] = ee;
            primes.insert(pp);
        }
    }

    let mut c = BTreeMap::<i32, BTreeMap<i32, i32>>::new();
    for i in 0..N {
        for j in 0..m[i] {
            if c.contains_key(&p[i][j]) {
                *c.get_mut(&p[i][j]).unwrap().entry(e[i][j]).or_default() += 1;
            } else {
                c.insert(
                    p[i][j],
                    btreemap!(
                        e[i][j] => 1
                    ),
                );
            }
        }
    }

    let mut uni = 0;
    let mut com = 0;
    for i in 0..N {
        let mut found = false;
        for j in 0..m[i] {
            let (&max_key, &count) = c.get(&p[i][j]).unwrap().iter().rev().next().unwrap();
            if count == 1 && max_key == e[i][j] {
                uni += 1;
                found = true;
                break;
            }
        }
        if !found {
            com += 1;
        }
    }
    //println!("{} {}", com, uni);
    let ans = uni + if com >= 1 { 1 } else { 0 };
    println!("{}", ans);
}
