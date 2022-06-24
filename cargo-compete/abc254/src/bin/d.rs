use itertools::*;
use num_integer;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: u64,
    }

    let mut sieve = vec![true; N as usize + 1];
    for i in 2..=num_integer::sqrt(N) {
        let ii = i * i;
        let mut j = ii;
        loop {
            sieve[j as usize] = false;
            j += ii;
            if j > N {
                break;
            }
        }
    }
    let square_frees = (1..=N).filter(|&i| sieve[i as usize]).collect_vec();
    let ans = square_frees
        .iter()
        .map(|&k| num_integer::sqrt(N / k).pow(2))
        .sum::<u64>();
    println!("{}", ans);
}
