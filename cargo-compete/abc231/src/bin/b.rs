#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
pub trait Change<T: PartialOrd> {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}
impl<T: PartialOrd> Change<T> for T {
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}
#[fastout]
fn main() {
    input!{
        N: usize,
        S: [String;N],
    }

    let mut m = BTreeMap::<String, usize>::new();
    for s in S.iter(){
        *m.entry(s.to_string()).or_default() += 1;
    }

    //let (ans, _) = m.iter().max().unwrap();
    let mut ans = String::new();
    let mut max = 0;
    for (name, count) in m.iter(){
        if max.chmax(*count){
            ans = name.to_string();
        }
    }
    
    println!("{}", ans);

    
}
