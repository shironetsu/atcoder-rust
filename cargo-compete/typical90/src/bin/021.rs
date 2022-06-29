#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use petgraph::unionfind::UnionFind;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut dir = vec![vec![]; N];
    let mut rev = vec![vec![]; N];
    for _ in 0..M {
        input! {
            a: Usize1,
            b: Usize1,
        }

        dir[a].push(b);
        rev[b].push(a);
    }

    let mut que = VecDeque::new();
    let mut seen = vec![false; N];
    let mut todo = VecDeque::new();
    for r in 0..N {
        if seen[r] {
            continue;
        }
        que.push_back((r, true)); //vertex, end
        loop {
            if let Some((v, end)) = que.pop_back() {
                if seen[v] && end {
                    continue;
                }
                seen[v] = true;
                if end {
                    que.push_back((v, false));
                    let mut children = 0;
                    for &w in dir[v].iter() {
                        if !seen[w] {
                            children += 1;
                            que.push_back((w, true));
                        }
                    }
                    if children == 0 {
                        que.pop_back();
                        todo.push_front(v);
                        //println!("{}", v);
                    }
                } else {
                    todo.push_front(v);
                    //println!("{}", v);
                }
            } else {
                break;
            }
        }
    }

    let mut ans = 0;
    let mut seen = vec![false; N];
    loop {
        if let Some(r) = todo.pop_front() {
            if seen[r] {
                continue;
            }
            let mut stack = VecDeque::new();
            stack.push_front(r);
            let mut size = 0;
            loop {
                if let Some(v) = stack.pop_front() {
                    if seen[v] {
                        continue;
                    }
                    //println!("{} {}",r,v);
                    size += 1;
                    seen[v] = true;
                    for &w in rev[v].iter() {
                        if !seen[w] {
                            stack.push_front(w);
                        }
                    }
                } else {
                    break;
                }
            }
            //println!("{}", size);
            let size = size as i64;
            ans += size * (size - 1) / 2;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
