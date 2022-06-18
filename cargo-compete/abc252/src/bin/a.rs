#![allow(unused_imports)]
#![allow(non_snake_case)]
use ascii::AsciiChar;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;

#[fastout]
fn main() {
    input! {
        N: u8,
    }

    // let a_z = "abcdefghijklmnopqrstuvwxyz";
    // let ans = a_z.chars().collect::<Vec<_>>()[(N - 97) as usize];
    // println!("{}", ans);
    //println!("{}", char::from_u32(N).unwrap());

    //let ans = N as u8;
    println!("{}", N as char);
}
