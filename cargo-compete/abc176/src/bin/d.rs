#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        H: usize,
        W: usize,
        Ch: Usize1,
        Cw: Usize1,
        Dh: Usize1,
        Dw: Usize1,
        S: [Chars;H],
    }

    let dist = vec![vec![None;W];H];
    dist[Ch][Cw] = Some(0);
    let mut todo = VecDeque::new();
    todo.push_back((Ch, Cw));
    loop {
        if let Some((x, y)) = todo.pop_front(){
            for &(dx, dy) in [(1, 0), (!0, 0), (0, 1), (0, !0)].iter(){
                if x + dx < H && y + dy < W {
                    let x1 = x + dx;
                    let y1 = y + dy;
                    if dist[x1][y1].is_some() {
                        continue;
                    }
                    if S[x1][y1] == '.' {
                        dist[x1][y1] = Some(dist[x][y]);
                        todo.push_back((x1, y1));
                    }
                }
            }
        } else {
            break;
        }
    }

    let ans = dist[Dh][Dw].unwrap_or(-1);
    println!("{}", ans);
    
}
//______________________________________________________________________________
//
pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [char] {
    fn to_string(&self) -> String {
        self.iter().collect::<String>()
    }
}
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self)->String;
    fn fmtl(&self)->String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self)->String {
        self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self)->String {
        self
            .iter()
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

