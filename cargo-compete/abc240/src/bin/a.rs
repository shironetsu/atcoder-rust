use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        a: i32,
        b: i32,
    }

    if a + 1 == b || (a, b) == (1, 10) {
        println!("Yes");
    } else {
        println!("No");
    }
}
