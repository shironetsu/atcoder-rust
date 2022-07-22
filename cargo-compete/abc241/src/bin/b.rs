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
        A: [i32;N],
        B: [i32;M]
    }

    let mut p = BTreeMap::<i32,usize>::new();
    for a in A {
        *p.entry(a).or_default() += 1;
    }

    let mut q = BTreeMap::<i32,usize>::new();
    for b in B {
        *q.entry(b).or_default() += 1;
    }

    for (b, &d) in q.iter(){
        if let Some(&e) = p.get(&b) {
            if !(d <= e) {
                println!("No");
                return;
            }
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
