#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

fn judge(a: char, b: char) -> (i32, i32){
    match (a, b){
        ('G', 'C')|('C','P')|('P', 'G') => (1,0),
        ('C', 'G')|('P','C')|('G', 'P') => (0, 1),
        _ => (0, 0),
    }
}

#[fastout]
fn main() {
    input!{
        N: usize,
        M: usize,
        A: [Chars;2*N],
    }

    let mut r = (0..2*N).collect_vec();
    let mut wins = vec![0;2*N];

    for i in 0..M{
        for k in 0..N{
            let a = r[2*k];
            let b = r[2*k+1];
            let (c, d) = judge(A[a][i], A[b][i]);
            wins[a] += c;
            wins[b] += d;
        }
        r.sort_by_key(|&i|(-wins[i], i));
    }

    for i in r {
        println!("{}", i + 1);
    }
    
}
