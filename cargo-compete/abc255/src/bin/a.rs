use proconio::input;
use std::fmt::Write;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input!{
        R: Usize1,
        C: Usize1,
        A: [[u32;2];2],
    }

    println!("{}", A[R][C]);
}
