#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    let mut s = vec![];
    let mut t = vec![];

    let mut i = 0;
    while i < S.len() {
        let h = S[i];
        let mut r = 1;
        while i + r < S.len() && h == S[i + r] {
            r += 1;
        }
        s.push((h, r));
        i += r;
    }

    let mut i = 0;
    while i < T.len() {
        let h = T[i];
        let mut r = 1;
        while i + r < T.len() && h == T[i + r] {
            r += 1;
        }
        t.push((h, r));
        i += r;
    }

    if s.len() != t.len(){
        println!("No");
        return;
    } else {
        for i in 0..s.len(){
            let (c, m) = s[i];
            let (d, n) = t[i];
            if c == d && ((m == n && n == 1) || (m > 1 && n > 1 && m <= n ) ){
                continue;
            } else{
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
