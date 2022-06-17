use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [i32;N],
    }

    let s = a.into_iter().collect::<BTreeSet<_>>();
    println!("{}", s.len());
}
