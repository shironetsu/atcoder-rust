use proconio::input;
use std::fmt::Write;
use superslice::Ext;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [i64;N],
        X: [i64;Q],
    }
    A.sort();
    let mut B = vec![0; N + 1];
    for i in 0..N {
        B[i + 1] = B[i] + A[i];
    }
    let mut ans = String::new();

    for x in X {
        let m = A.lower_bound(&x);
        let n = ((m as i64) * x - B[m]) + (B[N] - B[m] - (N - m) as i64 * x);
        writeln!(ans, "{}", n);
    }
    print!("{}", ans);
}
