use itertools::concat;
use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        W: usize,
    }

    let N = 300;
    let A: Vec<_> = concat(vec![
        (1..=100).collect(),
        (100..=10000).step_by(100).collect(),
        (10000..=1000000).step_by(10000).collect(),
    ]);
    println!("{}", N);
    println!("{:?}", A.iter().format(" "));
}
