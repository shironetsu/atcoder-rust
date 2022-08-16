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
        a: i64,
        b: i64,
        c: i64,
    }

    let u = -a+b+c;
    let v = a-b+c;
    let w = a+b-c;
    let t = u.min(v).min(w);
    if t < 0 {
        println!("-1");
        return;
    }

    let pp = u-t;
    let qq = v-t;
    let rr = w-t;
    if pp&1 == 1 || qq&1==1 || rr&1==1 {
        println!("-1");
        return;
    }

    let ans = (pp+qq+rr)/2+t;
    println!("{}", ans);

    
}
