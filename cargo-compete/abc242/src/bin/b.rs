use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        mut S: Chars,
    }

    S.sort();
    let ans = S.iter().collect::<String>();
    println!("{}", ans);
}
