#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

pub enum Marker {
    Leaf,
    Inner,
    Pending,
}

#[fastout]
fn main() {
    input! {
        N: usize,
    }
    let mut ad = vec![vec![]; N];
    let mut edges = vec![];
    for _ in 0..N - 1 {
        input! {
            mut a: Usize1,
            mut b: Usize1,
        }
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        ad[a].push(b);
        ad[b].push(a);
        edges.push((a, b));
    }

    let mut marked = vec![false; N];
    let mut stack = VecDeque::<(usize, Marker)>::new(); //back only
    stack.push_back((0, Marker::Pending));
    marked[0] = true;
    let mut count = 0;
    let mut inn = vec![0; N];
    let mut out = vec![0; N];
    loop {
        let mut todo = vec![];
        let mut resolved = false;
        if let Some((u, m)) = stack.back_mut() {
            match m {
                Marker::Leaf => {
                    out[*u] = count;
                    count += 1;
                    resolved = true;
                }
                Marker::Inner => {
                    out[*u] = count;
                    count += 1;
                    resolved = true;
                }
                Marker::Pending => {
                    inn[*u] = count;
                    count += 1;
                    for &v in ad[*u].iter() {
                        if !marked[v] {
                            todo.push(v);
                            marked[v] = true;
                        }
                    }
                    if todo.len() == 0 {
                        *m = Marker::Leaf;
                    } else {
                        *m = Marker::Inner;
                    }
                }
            }
        } else {
            break;
        }
        for v in todo {
            stack.push_back((v, Marker::Pending));
        }
        if resolved {
            stack.pop_back();
        }
    }

    let mut ans = 0;
    //println!("{:?}", inn);
    //println!("{:?}", out);
    for (a, b) in edges {
        let p = (out[a] - inn[a]) / 2 + 1;
        let q = (out[b] - inn[b]) / 2 + 1;
        let c = p.min(q);
        ans += c * (N - c);
    }
    println!("{}", ans);
}
