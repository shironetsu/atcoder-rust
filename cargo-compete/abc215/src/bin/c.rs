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
        mut S: Chars,
        K: Usize1,
    }

    let mut k = 0;
    loop {
        if k == K {
            let ans = S.iter().collect::<String>();
            println!("{}", ans);
            break;
        }

        k += 1;
        S.next_permutation();
    }

    
}
