#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::{Display, Write};
use superslice::{Ext, Ext2};

fn atoi(a: char) -> usize {
    (a as u8 - b'a') as usize
}

fn isa(a: char) -> bool {
    'a' <= a && a <= 'z'
}

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        a: [Chars;H],
    }

    let (sx, sy) = (0..H)
        .cartesian_product(0..W)
        .find(|&(i, j)| a[i][j] == 'S')
        .unwrap();
    let (gx, gy) = (0..H)
        .cartesian_product(0..W)
        .find(|&(i, j)| a[i][j] == 'G')
        .unwrap();
    let mut t = vec![vec![]; 26];
    for i in 0..H {
        for j in 0..W {
            if isa(a[i][j]) {
                t[atoi(a[i][j])].push((i, j));
            }
        }
    }

    let mut todo = VecDeque::new();
    todo.push_back((sx, sy));
    let mut dd = vec![vec![None; W]; H];
    dd[sx][sy] = Some(0);
    let mut used = vec![false; 26];
    loop {
        if let Some((x, y)) = todo.pop_front() {
            if isa(a[x][y]) {
                let i = atoi(a[x][y]);
                if !used[i] {
                    used[i] = true;
                    for &(a, b) in t[i].iter() {
                        if dd[a][b] == None {
                            todo.push_back((a, b));
                            dd[a][b] = Some(dd[x][y].unwrap() + 1);
                        }
                    }
                }
            }
            for (dx, dy) in &[(1, 0), (!0, 0), (0, 1), (0, !0)] {
                if x + dx < H && y + dy < W && a[x + dx][y + dy] != '#' {
                    if dd[x + dx][y + dy] == None {
                        dd[x + dx][y + dy] = Some(dd[x][y].unwrap() + 1);
                        todo.push_back((x + dx, y + dy));
                    }
                }
            }
        } else {
            break;
        }
    }

    println!("{}", dd[gx][gy].unwrap_or(-1));
}
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self) -> String;
    fn fmtl(&self) -> String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self) -> String {
        self.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self) -> String {
        self.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn ansl(&self) {
        println!("{}", self.fmtl());
    }
}
//______________________________________________________________________________
//
#[macro_export]
macro_rules! input_edges {
    ($n: expr, $m: expr, $edges: tt, $ad: tt) => {
        input! {
            $edges: [(Usize1, Usize1); $m],
        }

        let mut $ad = vec![vec![]; $n];
        for &(a, b) in $edges.iter() {
            $ad[a].push(b);
            $ad[b].push(a);
        }
        let $ad = $ad;
    };
}
