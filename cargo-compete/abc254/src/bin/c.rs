use itertools::Itertools;
use proconio::input;
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: u32,
        mut a: [u32;N],
    }

    for i in 0..K as usize {
        let mut b = vec![];
        for j in (i..N).step_by(K as usize) {
            b.push(a[j]);
        }
        b.sort();
        for (k, j) in (i..N).step_by(K as usize).enumerate() {
            a[j] = b[k];
        }
    }

    let mut c = a.clone();
    c.sort();

    if a == c {
        println!("Yes");
    } else {
        println!("No");
    }
}
