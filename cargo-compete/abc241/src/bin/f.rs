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
        H: i64,
        W: i64,
        N: usize,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        XY: [(i64,i64);N],
    }

    let mut h = BTreeMap::<i64,Vec<i64>>::new();
    let mut w = BTreeMap::<i64,Vec<i64>>::new();

    for &(x, y) in XY.iter(){
        if let Some(v) = h.get_mut(&x) {
            v.push(y);
        }
        if let Some(v) = w.get_mut(&y) {
            v.push(x);
        }
    }

    let mut ad = BTreeMap::<(i64, i64), BTreeSet<(i64, i64)>::new();

    for (x, yy) in h.iter_mut(){
        yy.sort();
        for i in 0..yy.len()-1{
            let y0 = yy[i];
            let y1 = yy[i+1];
            ad.entry((x, y0)).or_insert(vec!()).insert((x,y1));
            ad.entry((x, y1)).or_insert(vec!()).insert((x,y0));
        }
    } 
    
    for (y, xx) in w.iter_mut(){
        xx.sort();
        for i in 0..xx.len()-1{
            let x0 = xx[i];
            let x1 = xx[i+1];
            ad.entry((x0, y)).or_insert(vec!()).insert((x0, y));
            ad.entry((x1, y)).or_insert(vec!()).insert((x1,y));
        }
    } 

    if let Some(v) = h.get(&a) {
        let i = v.upper_bound(&b);
        if i == 0 {
            println!("-1");
            return;
        }
        let y0 = v[i-1];
        let y1 = v[i];
        ad.entry((a, b)).or_insert(vec!()).insert((a, y0));
        ad.entry((a, b)).or_insert(vec!()).insert((a, y1));
    }

    if let Some(v) = w.get(&b) {
        let i = v.upper_bound(&a);
        if i == 0 {
            println!("-1");
            return;
        }
        let x0 = v[i-1];
        let x1 = v[i];
        ad.entry((a, b)).or_insert(vec!()).insert((x0, b));
        ad.entry((a, b)).or_insert(vec!()).insert((x1, b));
    }

    let mut q = VecDeque::<(i64, i64, i64)>::new();
    let mut dist = BTreeMap::<(i64, i64), i64>::new();
    q.push_back((0, a, b));
    loop {
        if let Some((d, x, y)) = q.pop_front(){
            for &(u, v) in ad.get((x, y)).unwrap().iter() {
                
            }
        } else {
            break;
        }
    }
}
