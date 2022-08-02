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
    }

    let mut T = vec![0;N];
    let mut K = vec![0;N];
    let mut A = vec![vec![];N];
    for i in 0..N{
        input!{
            t: i64,
            k: usize,
            a: [Usize1;k],
        }
        T[i] = t;
        K[i] = k;
        A[i] = a;
    }

    let mut req = vec![false;N];
    let mut todo = VecDeque::new();
    let start = N - 1;
    todo.push_back(start);
    req[start] = true;

    loop {
        if let Some(i) = todo.pop_front(){
            for &j in A[i].iter(){
                if !req[j]{
                    todo.push_back(j);
                    req[j] = true;
                }
            }
        } else {
            break;
        }
    }

    let ans = (0..N).filter(|&i| req[i]).map(|i| T[i]).sum::<i64>();
    println!("{}", ans);
}
