#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        M: usize,
        uv: [(Usize1, Usize1);M],
        p: [Usize1;8],
    }

    let mut ad = vec![vec![];9];
    for (u, v) in uv{
        ad[u].push(v);
        ad[v].push(u);
    }

    let mut dist = BTreeMap::new();

    let mut init = [None;9];
    for (i, &x) in p.iter().enumerate(){
        init[x] = Some(i);
    }
    //dist.insert(init, 0);
    let mut todo = VecDeque::new();
    todo.push_back((init, 0));
    loop {
        if let Some((s, d)) = todo.pop_front(){
            if !dist.contains_key(&s){
                dist.insert(s, d);
                let (empty, _) = s.iter().enumerate().find(|&x|*x.1 == None).unwrap();
                for &v in ad[empty].iter(){
                    let mut t = s;
                    t[empty] = s[v];
                    t[v] = None;
                    todo.push_back((t, d+1));
                }
            }
        } else {
            break;
        }
    }

    let dest = (0..9).map(|x| if x < 8 { Some(x) } else { None }).collect_vec();
    if let Some(ans) = dist.get(&dest[..]){
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
