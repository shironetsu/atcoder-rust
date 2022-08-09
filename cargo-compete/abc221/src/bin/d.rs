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
        AB: [(Usize1, usize);N],
    }

    let mut s = BTreeSet::new();
    for &(A, B) in AB.iter(){
        s.insert(A);
        s.insert(A + B);
    }
    let xx = s.iter().copied().collect_vec();
    let m = s.into_iter().enumerate().map(|(i, x)|(x, i)).collect::<BTreeMap<usize, usize>>();
;
    let mut imos = vec![0;m.len()];
    for (A, B) in AB{
        let &x = m.get(&A).unwrap();
        let &y = m.get(&(A+B)).unwrap();
        imos[x] += 1;
        imos[y] += -1;
    }

    for i in 0..m.len() -1{
        imos[i+1] += imos[i]; 
    }

    let mut d = vec![0;N+1];
    for i in 0..m.len()-1{
        let r = xx[i+1] - xx[i];
        d[imos[i] as usize] += r;
    }
    let ans = (1..=N).map(|i|d[i].to_string()).join(" ");
    println!("{}", ans);

}
