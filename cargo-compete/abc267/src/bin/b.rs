#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::{Display, Write};
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        S: Chars,
    }

    let S = S
        .into_iter()
        .map(|c| if c == '0' { 0 } else { 1 })
        .collect_vec();

    if S[0] != 0 {
        println!("No");
        return;
    }

    let lines = vec![
        vec![S[6]],
        vec![S[3]],
        vec![S[1], S[7]],
        vec![S[0], S[4]],
        vec![S[2], S[8]],
        vec![S[5]],
        vec![S[9]],
    ];


    for i in 0..7 {
        let a = lines[i].iter().sum::<i32>();
        if a == 0 {
            continue;
        }
        for j in i + 1..7 {
            let b = lines[j].iter().sum::<i32>();
            if b == 0 {
                continue;
            }
            for k in i+1..j{
                let c = lines[k].iter().sum::<i32>();
                if c == 0 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
//______________________________________________________________________________
//
pub trait Answer {
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn ans(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", ans);
    }

    fn ansl(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", ans);
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
