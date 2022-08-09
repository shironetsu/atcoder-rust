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
        M: usize,
        AB: [(Usize1,Usize1);M],
    }

    let mut aa = vec![vec![];N];
    let mut bb = vec![vec![];N];
    for &(a, b) in AB.iter(){
        aa[b].push(a);
        bb[a].push(b);
    }
    for v in aa.iter_mut(){
        v.sort();
    }
    for v in bb.iter_mut(){
        v.sort();
    }

    let mut ans = vec![];
    
    if let Some((h, _)) = aa.iter().enumerate().find(|(i, v)|v.len()==0){
        for i in 0..N{

        }
    } else {
        println!("-1");
    }





    
}
