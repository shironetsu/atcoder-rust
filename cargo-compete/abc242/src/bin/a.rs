use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: i32,
        B: i32,
        C: i32,
        X: i32,
    }

    let ans = if X <= A {
        1.0
    } else if A + 1 <= X && X <= B {
        let D = B - A;
        (C as f64) / (D as f64)
    } else {
        0.0
    };
    println!("{}", ans);
}
