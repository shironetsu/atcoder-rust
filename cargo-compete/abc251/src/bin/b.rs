use itertools::Itertools;
use proconio::input;
use std::collections::BTreeSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: u32,
        mut A: [u32; N],
    }

    let mut s = BTreeSet::new();

    A.push(0);
    A.push(0);
    //for &[i, j, k] in (0..N).combinations(3).map(|c|{c.as_slice()}) {
    for (i, j, k) in (0..N+2).tuple_combinations() {
        let w = A[i] + A[j] + A[k];
        if w <= W {
            s.insert(w);
        };
    }
    println!("{}", s.len());
}
