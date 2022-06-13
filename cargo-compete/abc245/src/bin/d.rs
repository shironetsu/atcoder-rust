use proconio::input;
#[allow(unused_imports)]
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [i64;N+1],
        mut C: [i64;N+M+1],
    }

    let mut B = vec![0; M + 1];
    for i in (0..=M).rev() {
        B[i] = C[N + i] / A[N];
        for j in 0..=N {
            C[i + j] -= A[j] * B[i];
        }
    }
    let ans = B
        .iter()
        .map(|&b| b.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
}
