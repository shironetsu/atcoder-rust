use proconio::input;
use std::collections::BTreeSet;

#[allow(non_snake_case)]
fn main() {
    input! {
     N: usize,
     A: [u32;N],
    }

    let A = A.into_iter().collect::<BTreeSet<u32>>();
    for i in 0.. {
        if !A.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}
