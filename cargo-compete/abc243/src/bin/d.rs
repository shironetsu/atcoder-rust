#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

// #[derive(Clone,Copy,PartialEq,PartialOrd,Eq,Ord,Debug)]
// enum Dir{
//     Up,
//     Left,
//     Right,
// }

#[fastout]
fn main() {
    input!{
        N: usize,
        X: i64,
        S: Chars,
    }

    let mut up = 0;
    let mut stack = VecDeque::<char>::new();
    for s in S {
        if s == 'U' {
            if !stack.is_empty() {
                stack.pop_back();
            } else {
                up += 1;
            }
        } else {
            stack.push_back(s);
        } 
    }

    let mut X = X;
    for i in 0..up{
        X /= 2;
    }

    loop {
        if let Some(d) = stack.pop_front(){
            if d == 'L' {
                X = 2 * X;
            } else {
                X = 2 * X + 1;
            }
        } else {
            break;
        }
    }

    println!("{}", X);    
}
