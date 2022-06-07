use num_integer;
use proconio::input;
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: u64,
    }

    let mut r = (1..=N).collect::<Vec<_>>();

    for i in 2..=num_integer::sqrt(N) {
        for n in r.iter_mut() {
            while *n % (i * i) == 0 {
                *n /= i * i;
            }
        }
    }

    let ans = r.iter().map(|n| num_integer::sqrt(N / n)).sum::<u64>();

    println!("{}", ans);
}
