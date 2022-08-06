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
        M: i32,
    }

    let mut vv = vec![];
    for m in 1..=M{
        vv.push(vec![m]);
    }
    for i in 0..N-1{
        let mut w = vec![];
        for v in vv {
            for m in v[i]+1..=M{
                let mut a = v.clone();
                a.push(m);
                w.push(a);
            }
        }
        vv = w;
    }

    let mut ans = String::new();
    for (i, v) in vv.iter().enumerate() {
        let v = v.iter().map(|x|x.to_string()).join(" ");
        if i == vv.len() - 1 {
            println!("{}",  v);
        } else {
            println!("{}",  v);
        }
    }


    
}
