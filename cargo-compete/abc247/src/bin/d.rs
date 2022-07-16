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
        Q: usize,
    }

    let mut q = VecDeque::<(i64, i64)>::new();
    let mut ans = vec![];
    for _ in 0..Q{
        input!{t:i32}
        if t == 1 {
            input!{x: i64, c: i64}
            q.push_back((x, c));
        } else {
            input!{c:i64}
            let mut cc = c;
            let mut xx = 0i64;
            loop {
                if let Some((x, c)) = q.pop_front(){
                    if cc >= c {
                        xx += x * c;
                        cc -= c;
                    } else { //cc < c
                        xx += x * cc;
                        q.push_front((x, c-cc));
                        break;
                    } 
                } else {
                    break;
                }
            }
            ans.push(xx);
        }
    }

    for a in ans{
        println!("{}", a);
    }

    
}
