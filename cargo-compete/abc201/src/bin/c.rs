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

    let mut ans = 0;
    for i in 0000..=9999 {
        let p = format!("{:04}", i)
            .chars()
            .map(|c| (c as u8 - '0' as u8) as usize)
            .collect_vec();
        let mut c = [false; 10];
        for &pp in p.iter() {
            c[pp] = true;
        }
        let ok = S
            .iter()
            .zip(c.iter())
            .map(|x| match x {
                ('o', true) => true,
                ('o', false) => false,
                ('x', true) => false,
                ('x', false) => true,
                ('?', _) => true,
                _ => unreachable!(),
            })
            .fold(true, |acc, x| acc && x);
        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
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
