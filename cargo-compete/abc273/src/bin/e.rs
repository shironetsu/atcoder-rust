#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};
use std::rc::Rc;

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Copy, Debug)]
pub enum Query{
    Add(i64),
    Delete,
    Save(i64),
    Load(i64),
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Copy, Debug)]
pub struct Entry {
    prev: Option<usize>,
    value: i64,
}

#[fastout]
fn main() {
    input!{
        Q: usize,
    }

    let mut q = Vec::<Query>::with_capacity(Q);

    for _ in 0..Q{
        input!{
            s: String,
        }

        match s.as_str() {
            "ADD" => {
                input!{
                    x: i64,
                }
                q.push(Query::Add(x));
            },
            "DELETE" => {
                q.push(Query::Delete);
            },
            "SAVE" => {
                input!{
                    y: i64,
                }
                q.push(Query::Save(y));
            },
            "LOAD" => {
                input!{
                    z: i64,
                }
                q.push(Query::Load(z));
            },
            _ => unreachable!()
        }
    }

    let mut entries = vec![];
    let mut note = BTreeMap::<i64, Option<usize>>::new();
    let mut c = None;

    let mut ans = Vec::with_capacity(Q);

    for &qq in q.iter(){
        match qq {
            Query::Add(x) => {
                entries.push(Entry { prev: c, value: x });
                c = Some(entries.len()-1);
            },
            Query::Delete => {
                if let Some(i) = c {
                    c = entries[i].prev;
                }
            },
            Query::Save(y) => {
                note.insert(y, c);
            },
            Query::Load(z) => {
                c = *note.get(&z).unwrap_or(&None);
            }
        }
        ans.push(c.map_or(-1, |i| entries[i].value));
    }

    ans.ans();
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

