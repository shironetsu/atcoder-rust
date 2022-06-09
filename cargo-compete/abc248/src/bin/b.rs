use proconio::input;
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: u64,
        B: u64,
        K: u64,
    }

    let mut i = 0;
    while A < B {
        A *= K;
        i += 1;
    }
    println!("{}", i);
}
