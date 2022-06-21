#![allow(unused_imports)]
#![allow(non_snake_case)]
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;

#[fastout]
fn main() {
    input! {
        X: Chars,
    }

    let N = X
        .into_iter()
        .rev()
        .map(|x| (x as u8 - '0' as u8) as i64)
        .collect::<Vec<_>>();
    let mut sum = N.iter().sum::<i64>();
    let mut A = Vec::<i64>::new();
    let mut car = 0;
    for i in 0.. {
        let (a, b) = ((sum + car) / 10, (sum + car) % 10);
        car = a;
        sum -= if i < N.len() { N[i] } else { 0 };
        if car == 0 && i >= N.len() {
            if b != 0 {
                A.push(b);
            }
            break;
        } else {
            A.push(b)
        }
    }

    let S = A
        .into_iter()
        .rev()
        .map(|a| ('0' as u8 + a as u8) as char)
        .collect::<String>();
    println!("{}", S);
}
