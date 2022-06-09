use proconio::input;
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: u64,
        M: u64,
        K: u64,
    }

    //(N*1) * (K+1)
    let modulo = 998244353;
    let mut dp = vec![vec![0; (K + 1) as usize]; (N + 1) as usize];
    dp[0][0] = 1;
    for i in 0..N {
        for j in 0..=K {
            for a in 1..=M {
                if j + a <= K {
                    dp[i as usize + 1][(j + a) as usize] += dp[i as usize][j as usize];
                    dp[i as usize + 1][(j + a) as usize] %= modulo;
                }
            }
        }
    }
    let ans = dp[N as usize].iter().sum::<u64>() % modulo;
    println!("{}", ans);
}
