#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Copy, Debug)]
pub enum Query {
    One(i64),
    Two(i64),
    Three,
}

#[fastout]
fn main() {
    input! {
        Q: usize,
    }

    let mut q = vec![];
    for i in 0..Q {
        input! {
            t: i32,
        }

        match t {
            1 => {
                input! {
                    x: i64,
                }
                q.push(Query::One(x));
            }
            2 => {
                input! {
                    x: i64,
                }
                q.push(Query::Two(x));
            }
            3 => {
                q.push(Query::Three);
            }
            _ => unreachable!(),
        }
    }

    let mut ans = vec![];
    let mut m = BTreeMap::<i64, i64>::new();

    let mut c = 0;
    for i in 0..Q {
        match q[i] {
            Query::One(x) => {
                *m.entry(x - c).or_default() += 1;
            }
            Query::Two(x) => {
                c += x;
            }
            Query::Three => {
                let (x, n) = m.iter_mut().next().unwrap();
                ans.push(*x + c);
                let (x, remove) = if *n == 1 {
                    (*x, true)
                } else {
                    *n -= 1;
                    (*x, false)
                };
                if remove {
                    m.remove(&x);
                }
            }
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
