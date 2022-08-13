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
        S: Chars,
    }

    let mut d = "atcoder".to_string().chars().enumerate().map(|(i, c)|(c, i)).collect::<BTreeMap<_,_>>();
    let v = S.iter().map(|&c|d.get(&c).unwrap()).collect_vec();
    let ans = {
        let mut inv = 0;
        for i in 0..v.len(){
            for j in i+1..v.len(){
                if v[i] > v[j] {
                    inv += 1;
                }
            }
        }
        inv
    };
    println!("{}", ans);

    
}
