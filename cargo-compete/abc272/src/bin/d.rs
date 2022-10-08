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
        N: usize,
        M: usize,
    }

    let mut v = BTreeSet::new();
    for dx in 0..=num_integer::sqrt(M){
        let dy = num_integer::sqrt(M-dx*dx);
        if dx*dx+dy*dy==M{
            v.insert((dx, dy));
            if dx != 0 {
                v.insert((!(dx-1), dy));
            }
            if dy != 0 {
                v.insert((dx, !(dy-1)));
            }
            if dx != 0 && dy != 0 {
                v.insert((!(dx-1), !(dy-1)));
            }
        }
    }
    let mut v = v.into_iter().collect_vec();
    v.sort();

    //println!("{:?}", v);

    let mut dd = vec![vec![None;N];N];

    let mut todo = VecDeque::new();
    todo.push_back((0, 0));
    dd[0][0] = Some(0);
    loop {
        if let Some((x, y)) = todo.pop_front() {
            for &(dx, dy) in v.iter() {
                let p = x + dx;
                let q = y + dy;
                if p < N && q < N {
                    if dd[p][q] == None {
                        dd[p][q] = Some(dd[x][y].unwrap()+1);
                        todo.push_back((p, q));
                    }
                }
            }
        } else {
            break;
        }
    }

    for i in 0..N{
        let row = dd[i].clone().iter().map(|r|r.unwrap_or(-1)).collect_vec();
        row.ans();
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

