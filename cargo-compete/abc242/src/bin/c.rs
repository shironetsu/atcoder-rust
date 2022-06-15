use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use std::fmt::Write;

const MODULO: i64 = 998244353;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: i64,
    }

    let mut dp = vec![vec![0; N as usize]; 10];
    for i in 1..=9 {
        dp[i][0] = 1;
    }
    for i in 1..N as usize {
        for m in 1..=9 {
            for n in (m - 1).max(1)..=(m + 1).min(9) {
                dp[m][i] += dp[n][i - 1];
            }
            dp[m][i] %= MODULO;
        }
    }

    let ans: i64 = (1..=9).map(|i| dp[i][N as usize - 1]).sum::<i64>() % MODULO;
    println!("{}", ans);
}
