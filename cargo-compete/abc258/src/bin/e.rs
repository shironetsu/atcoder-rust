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
        Q: usize,
        X: i64,
        W: [i64;N],
        K: [i64;Q],
    }

    let sum = W.iter().sum::<i64>();
    let WW = W.iter().chain(W.iter()).map(|&x|x).collect_vec();
    let mut cum = vec![0;2*N+1];
    for i in 0..2*N{
        cum[i+1] = cum[i] + WW[i];
    }
    
    let mut lens = vec![0;N];
    let mut next = vec![0;N];

    for i in 0..N{
        let x = X % sum;
        let s = X / sum;
        let j = cum.lower_bound(&(cum[i] + x));
        lens[i] = (j-i) as i64 + s * N as i64;
        next[i] = j % N;
    }

    //println!("{:?}", lens);
    //println!("{:?}", next);
    
    // find loop
    let mut a = 0;
    let mut b = 0;
    //let mut count = 0;
    loop{
        a = next[a];
        b = next[next[b]];
        //count += 1;
        if a == b {
            break;
        }
    }

    let start = a;
    let mut h = start;
    let mut circ = vec![];
    loop{
        circ.push(h);
        h = next[h];
        if h == start{
            break;
        }
    }

    let circset = circ.iter().map(|&x|{x}).collect::<BTreeSet::<_>>();
    let mut h = 0;
    let mut head = vec![];
    loop{
        if circset.contains(&h){
            break;
        } else {
            head.push(h);
        }
        h = next[h];
    }

    let mut circ = vec![];
    let mut visited = vec![false;N];
    loop{
        circ.push(h);
        visited[h] = true;
        h = next[h];
        if visited[h]{
            break;
        }
    }

    //debug
    //println!("{:?}", head);
    //println!("{:?}", circ);
    //debug

    let mut ans = vec![];
    for &k in K.iter(){
        let k = k - 1;
        if (k as usize) < head.len() {
            let i = head[k as usize];
            ans.push(lens[i]);
        } else {
            let res = k as usize - head.len();
            let rem = res % circ.len();
            let i = circ[rem as usize];
            ans.push(lens[i]);
        }
    }

    for a in ans{
        println!("{}", a);
    }

    
}
