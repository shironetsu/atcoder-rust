#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: Chars,
    }

    // let mut leftmost = vec![vec![0; N]; 26];
    // let mut l = [N; 26];
    // for i in (0..N).rev() {
    //     let cc = S[i];
    //     let cu = (cc as u8 - b'a') as usize;
    //     l[cu] = i;
    //     for c in 0..26 {
    //         leftmost[c][i] = l[c];
    //     }
    // }

    // let mut ans = vec![];
    // let mut h = 0;
    // for i in 0..K {
    //     for c in 0..26 {
    //         let l = leftmost[c][i];
    //         if h <= l && l < N - K + i + 1 {
    //             h = l + 1;
    //             ans.push((c as u8 + b'a') as char);
    //             break;
    //         }
    //     }
    // }
    // let ans = ans.iter().collect::<String>();
    // println!("{}", ans);

    // for c in 0..26{
    //     let cc = (b'a' + c as u8) as char;
    //     let mut l = N;
    //     for i in (0..N).rev(){
    //         leftmost[c][i] = l;
    //         if S[i] == cc {
    //             l = i;
    //         }
    //     }
    // }

    let mut pos = vec![vec![]; 26];
    for (i, &c) in S.iter().enumerate() {
        let c = c as u8 - b'a';
        pos[c as usize].push(i);
    }

    let mut ans = vec![];
    let mut a = 0;
    for i in 0..K {
        for j in 0..26 {
            let b = pos[j].lower_bound(&a);
            if b == pos[j].len() {
                continue;
            }
            let k = pos[j][b];
            if k < N - K + i + 1 {
                ans.push((b'a' + j as u8) as char);
                a = k + 1;
                break;
            }
        }
    }
    let ans = ans.iter().collect::<String>();
    println!("{}", ans);
}
