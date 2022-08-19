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
        N: usize,
        M: usize,
    }

    let mut a = vec![vec![];M];
    for i in 0..M{
        input!{
            k: usize,
            aa: [Usize1;k],
        }

        a[i] = aa;
    }

    let mut e = vec![BTreeSet::new();N];
    let mut indeg = vec![0;N];
    for i in 0..M{
        for j in 0..a[i].len() - 1{
            let from = a[i][j]; // below
            let to = a[i][j+1];
            if e[from].insert(to){
                indeg[to] += 1;
            }
        }
    }

    let mut ord = vec![];

    let mut q = VecDeque::new();
    for i in 0..N{
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }
    
    loop {
        if let Some(i) = q.pop_front(){
            for &j in e[i].iter(){
                indeg[j] -= 1;
                if indeg[j] == 0 {
                    q.push_back(j);
                }
            }
            ord.push(i);
        } else {
            break;
        }
    }

    let ord = ord.iter().map(|&i|i+1).collect_vec();
    //println!("{:?}", ord);

    if ord.len() == N {
        println!("Yes");
    } else {
        println!("No");
    }
}
