#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Copy, Debug)]
pub enum Query {
    One(i64),
    Two,
    Three,
}

#[fastout]
fn main() {
    input!{
        Q: usize,
    }

    let mut qq = Vec::<Query>::with_capacity(Q);
    for _ in 0..Q{
        input!{
            i: i32,
        }
        let q = match i {
            1 => {
                input!{
                    x: i64,
                }
                Query::One(x)
            },
            2 => Query::Two,
            3 => Query::Three,
            _ => unreachable!(),
        };
        qq.push(q);
    }

    let mut ans = vec![];
    let mut l = BTreeMap::<i64, usize>::new();
    let mut r = VecDeque::<i64>::new();
    for &q in qq.iter(){
        match q {
            Query::One(x) => r.push_back(x),
            Query::Two => {
                let x = if l.len() > 0 {
                    let (&x, &c) = l.iter().next().unwrap();
                    if c > 1 {
                        l.insert(x, c-1);
                    } else {
                        l.remove(&x);
                    }
                    x
                } else {
                    r.pop_front().unwrap()
                };
                ans.push(x);
            },
            Query::Three => {
                loop {
                    if let Some(x) = r.pop_front(){
                        *l.entry(x).or_default() += 1;
                    } else {
                        break;
                    }
                }
            },
        }
    }

    for x in ans {
        println!("{}", x);
    }
    
}
