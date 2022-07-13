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
        S1: String,
        S2: String,
    }

    if (&S1[..], &S2[..]) == ("#.", ".#") || (&S1[..], &S2[..]) == (".#", "#."){
        println!("No");
    } else {
        println!("Yes");
    }

    
}
