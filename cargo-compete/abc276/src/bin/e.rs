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
        C: [Chars;H],
    }

    let (a, b) = (0..H).cartesian_product(0..W).find(|&(i, j)|C[i][j] == 'S').unwrap();

    for &(dx, dy) in [(1, 0), (!0, 0), (0, 1), (0, !0)].iter(){
        let a1 = a + dx;
        let b1 = b + dy;
        if !(a1 < H && b1 < W) || C[a1][b1] == '#' {
            continue;
        }

        let C = C.clone();
        let mut dd = vec![vec![None;W];H];
        dd[a1][b1] = Some(0);
        let mut todo = VecDeque::new();
        todo.push_back((a1, b1));
        loop {
            if let Some((i, j)) = todo.pop_front(){
                for &(dx, dy) in [(1, 0), (!0, 0), (0, 1), (0, !0)].iter(){
                    let i1 = i + dx;
                    let j1 = j + dy;
                    if (i,j) == (a1,b1) && (i1,j1) == (a,b) {
                        continue;
                    }
                    if i1 < H && j1 < W && C[i1][j1] != '#' {
                        if dd[i1][j1].is_none() {
                            let d = dd[i][j].unwrap();
                            dd[i1][j1] = Some(d+1);
                            todo.push_back((i1, j1));
                        }
                    }
                }
            } else {
                break;
            }
        }

        if dd[a][b].is_some(){
            println!("Yes");
            return;
        }
    }

    println!("No");
    
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

