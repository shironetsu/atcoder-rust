#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use std::ops::Bound::{Included, Unbounded};
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        P:[Usize1;N],
    }

    if K ==1 {
        let mut ans = vec![0;N];
        for (i, &p) in P.iter().enumerate(){
            ans[p] = i;
        }
        for a in ans {
            println!("{}", a + 1);
        }
        return;
    }

    let mut ans = vec![-1; N];
    let mut yama = BTreeMap::<usize, usize>::new();
    let mut vv: Vec<Vec<usize>> = vec![];
    for (i, &p) in P.iter().enumerate() {
        let mut kcopy = 0usize;
        let mut some = true;
        let mut rem = false;
        if let Some((&k, &j)) = yama.range((Included(&p), Unbounded)).next() {
            //let mut v = v;
            kcopy = k;
            yama.insert(p, j);
            vv[j].push(p);
            if vv[j].len() == K {
                rem = true;
                for &u in vv[j].iter() {
                    ans[u] = (i + 1) as i32;
                }
            }
        } else {
            some = false;
            vv.push(vec![p]);
            yama.insert(p, vv.len() - 1);
        }
        if some {
            yama.remove(&kcopy);
        }
        if rem {
            yama.remove(&p);
        }
    }

    for a in ans {
        println!("{}", a);
    }

    // let mut top = vec![];
    // let mut yama = vec![];
    // let mut ans = vec![-1;N];
    // let mut h = 0;
    // for i in 0..N{
    //     let j = top[h..].upper_bound(&P[i]);
    //     let j = j + h;
    //     //println!("{:?}", top.iter().copied().map(|i|{i+1}).collect_vec());
    //     if j >= top.len() {
    //         top.push(P[i]);
    //         yama.push(vec![P[i]]);
    //     } else {
    //         top[j] = P[i];
    //         yama[j].push(P[i]);
    //     }
    //     if yama[j].len() % K == 0 {
    //         h += 1;
    //         for &k in yama[j].iter().rev().take(K){
    //             ans[k] = (i + 1) as i32;
    //         }
    //     }
    // }
    // for a in ans{
    //     println!("{}", a);
    // }
}
