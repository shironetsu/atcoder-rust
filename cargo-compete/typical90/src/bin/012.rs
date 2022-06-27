#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};
use petgraph::unionfind::UnionFind;

#[derive(Clone, Copy)]
enum Query{
    One(usize, usize),
    Two(usize, usize, usize, usize),
}

#[fastout]
fn main() {
    input!{
        H: usize,
        W: usize,
        Q: usize,
    }

    let mut qs = vec![];
    let mut K = 0;
    for _ in 0..Q{
        input!{t: i32}
        if t == 1 {
            input!{ x: Usize1, y:Usize1 }
            qs.push(Query::One(x,y));
            K += 1;
        } else {
            input!{a:Usize1, b: Usize1, c: Usize1, d: Usize1}
            qs.push(Query::Two(a,b,c,d));
        }
    }

    let mut vs = BTreeMap::<(usize, usize), usize>::new();
    let mut uf = UnionFind::new(K);
    let mut i = 0;
    let mut ans = vec![];
    for &q in qs.iter(){
        match q {
            Query::One(x, y) => {
                vs.insert((x, y), i);
                if 0 < x {
                    if let Some(&j) = vs.get(&(x-1,y)){
                        uf.union(i, j);
                    }
                }
                if 0 < y {
                    if let Some(&j) = vs.get(&(x,y-1)){
                        uf.union(i, j);
                    }
                }
                if  x < H-1 {
                    if let Some(&j) = vs.get(&(x+1,y)){
                        uf.union(i, j);
                    }
                }
                if y < W-1 {
                    if let Some(&j) = vs.get(&(x,y+1)){
                        uf.union(i, j);
                    }
                }
                i += 1;
            },
            Query::Two(a,b,c,d) => {
                let mut res = false;
                if let Some(&i) = vs.get(&(a, b)){
                    if let Some(&j) = vs.get(&(c, d)){
                        if uf.equiv(i, j) {
                            res = true;
                        }
                    }
                }
                ans.push(res);
            },
        }
    }

    for &res in ans.iter(){
        if res {
            println!("Yes");
        } else {
            println!("No");
        }
    }
    
}
