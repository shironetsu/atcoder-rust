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

    let mut a = BTreeMap::<i64,usize>::new();
    let mut ans = vec![];
    for _ in 0..Q{
        input!{t:i32}
        if t == 1 {
            input!{x:i64}
            *a.entry(x).or_default() += 1;
        } else if t == 2 {
            input!{x:i64,mut k:usize}
            let mut it = a.range(..=x).rev();
            loop {
                if let Some((&y, &m)) = it.next(){
                    if k <= m {
                        ans.push(y);
                        break;
                    } else {
                        k -= m;
                    }
                } else {
                    ans.push(-1);
                    break;
                }
            }
        } else if t == 3 {
            input!{x:i64,mut k:usize}
            let mut it = a.range(x..);
            loop {
                if let Some((&y, &m)) = it.next(){
                    if k <= m {
                        ans.push(y);
                        break;
                    } else {
                        k -= m;
                    }
                } else {
                    ans.push(-1);
                    break;
                }
            }
        }
    }

    for x in ans {
        println!("{}", x);
    }
}
