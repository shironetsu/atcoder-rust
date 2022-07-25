#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};


fn rec(u: usize, a: &mut Vec<Vec<i64>>, ad: &Vec<Vec<usize>>, X: &Vec<i64>){
    if ad[u].len() == 0 {
        a[u] = vec![X[u]];
    } else {
        let mut bh = BinaryHeap::<i64>::new();
        bh.push(X[u]);
        for &v in ad[u].iter(){
            rec(v, a, ad, X);
            for &k in a[v].iter(){
                bh.push(k);
            }
        }
        let mut b = vec![];
        let mut count = 0;
        loop {
            if let Some(k) = bh.pop() {
                b.push(k);
                count += 1;
            } else {
                break;
            }
            if count >= 20 {
                break;
            }
        }
        a[u] = b;
    }
}

#[fastout]
fn main() {
    input!{
        N: usize,
        Q: usize,
        X: [i64;N],
        AB: [(Usize1,Usize1);N-1],
        VK: [(Usize1, Usize1);Q],
    }

    let mut ad = vec![vec![];N];
    for (A, B) in AB {
        ad[A].push(B);
        ad[B].push(A);
    }

    let root = 0;
    let mut todo = vec![root];
    for i in 0..N{
        let u = todo[i];
        for v in ad[u].clone(){
            ad[v].retain(|&w| w != u);
            todo.push(v);
        }
    }

    let mut a = vec![vec![];N];

    rec(root, &mut a, &ad, &X);

    for &(v, k) in VK.iter(){
        println!("{}", a[v][k]);
    }
}
