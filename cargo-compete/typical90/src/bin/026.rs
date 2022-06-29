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
        N: usize,
    }

    let mut ad = vec![vec![]; N];
    for _ in 0..N - 1 {
        input! {
            a: Usize1,
            b: Usize1,
        }
        ad[a].push(b);
        ad[b].push(a);
    }

    let mut colors = vec![0; N];
    let mut todo = VecDeque::new();
    todo.push_back((0, 1));
    let mut visited = vec![false; N];
    loop {
        if let Some((v, c)) = todo.pop_front() {
            if visited[v] {
                continue;
            }
            colors[v] = c;
            visited[v] = true;
            for &w in ad[v].iter() {
                if !visited[w] {
                    todo.push_back((w, -c));
                }
            }
        } else {
            break;
        }
    }

    for &c in [1, -1].iter() {
        let ans = colors
            .iter()
            .enumerate()
            .filter(|&(i, &ci)| ci == c)
            .map(|(i, &c)| i + 1)
            .take(N / 2)
            .collect_vec();
        if ans.len() == N / 2 {
            let ans = ans.into_iter().map(|i| i.to_string()).join(" ");
            println!("{}", ans);
            return;
        }
    }
}
