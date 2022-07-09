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
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    let mut x = vec![0i64;N];
    let mut y = vec![0i64;N];
    let mut r = vec![0i64;N];

    for i in 0..N{
        input!{
            xx: i64,
            yy: i64,
            rr: i64,
        }

        x[i] = xx;
        y[i] = yy;
        r[i] = rr;
    }

    let mut ad = vec![vec![];N];
    for i in 0..N{
        //i < j
        for j in i+1..N{
            let dist2 = (x[i]-x[j]).pow(2) + (y[i]-y[j]).pow(2);
            let aa = (r[i] - r[j]).pow(2);
            let bb = (r[i] + r[j]).pow(2);
            if aa <= dist2 && dist2 <= bb {
                ad[i].push(j);
                ad[j].push(i);
            }            
        }
    }

    let mut starts = vec![];
    let mut goals = vec![];
    for i in 0..N{
        if (a-x[i]).pow(2) + (b-y[i]).pow(2) == r[i].pow(2) {
            starts.push(i);
        }
        if (c-x[i]).pow(2) + (d-y[i]).pow(2) == r[i].pow(2) {
            goals.push(i);
        }
    }

    let s = starts[0];

    let mut visited = vec![false;N];
    let mut todo = VecDeque::<usize>::new();
    todo.push_back(s);
    loop {
        if let Some(u) = todo.pop_front(){
            if visited[u] {
                continue;
            }
            visited[u] = true;
            for &v in ad[u].iter(){
                if !visited[v] {
                    todo.push_back(v);
                }
            }
        } else {
            break;
        }
    }

    // println!("{:?}", starts);
    // println!("{:?}", goals);
    // println!("{:?}", visited);

    if visited[goals[0]] {
        println!("Yes");
    } else {
        println!("No");
    }
    
}
