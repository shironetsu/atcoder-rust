use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: i64,
        A: [i64;N],
        B: [i64;N],
    }

    let mut dp = vec![false; N];
    let mut ep = vec![false; N];
    dp[0] = true;
    ep[0] = true;
    for i in 1..N {
        dp[i] = (dp[i - 1] && (A[i] - A[i - 1]).abs() <= K)
            || (ep[i - 1] && (A[i] - B[i - 1]).abs() <= K);
        ep[i] = (dp[i - 1] && (B[i] - A[i - 1]).abs() <= K)
            || (ep[i - 1] && (B[i] - B[i - 1]).abs() <= K);
    }
    if dp[N - 1] || ep[N - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
