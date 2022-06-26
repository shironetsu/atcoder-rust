#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

const INF: usize = 1_000_000;

#[fastout]
fn main() {
    input!{
        N: usize,
        a: [i32;N],
        b: [i32;N],
        Q: usize,
        queries: [(Usize1, Usize1);Q],
    }

    //entry point
    let mut epa = BTreeMap::<i32, usize>::new();
    let mut epb = BTreeMap::<i32, usize>::new();

    for (i, &x) in a.iter().enumerate(){
        epa.entry(x).or_insert(i);
    }

    for (i, &x) in b.iter().enumerate(){
        epb.entry(x).or_insert(i);
    }

    // println!("{:?}", epa);
    // println!("{:?}", epb);

    let mut amax = vec![0;N];
    let mut bmax = vec![0;N];
    
    for i in 0..N{
            if let Some(&n) = epb.get(&a[i]) {
                amax[i] = if i > 0 { amax[i-1].max(n) } else { n };
            } else {
                amax[i] = INF;
            }
            if let Some(&n) = epa.get(&b[i]) {
                bmax[i] = if i > 0 { bmax[i-1].max(n) } else { n };
            } else {
                bmax[i] = INF;
            }
    }

    for &(x, y) in queries.iter(){
        //println!("{} {} {} {}", x, y, amax[x], bmax[y]);
        if  amax[x].max(bmax[y]) < INF && amax[x] <= y && bmax[y] <= x {
            println!("Yes");
        } else {
            println!("No");
        }
    }
    
}

pub trait Change<T: PartialOrd> {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}
impl<T: PartialOrd> Change<T> for T {
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}
