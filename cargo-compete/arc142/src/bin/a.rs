#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

fn f(x: i64)->i64{
    let mut x = x;
    while x % 10 == 0{
        x /= 10;
    }

    let y = rev(x);
    x.min(y)
}

fn rev(x: i64)->i64{
    let mut x = x.to_string().chars().collect_vec();
    x.reverse();
    x.iter().collect::<String>().parse::<i64>().unwrap()
}

#[fastout]
fn main() {
    input!{
        N: i64,
        K: i64,
    }

    if K != f(K){
        println!("0");
        return;
    }

    let mut ans = 0;
    for &n in btreeset![K, rev(K)].iter(){
        let mut n = n;
        while n <= N{
            ans += 1;
            n *= 10;
        }
    }

    println!("{}", ans);
    
}
