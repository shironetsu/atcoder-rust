use proconio::input;
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        X: i64,
        mut A: i64,
        mut D: i64,
        N: i64,
    }

    if D == 0 {
        println!("{}", (X-A).abs());
        return;
    }

    if D < 0 {
        let A0 = A;
        let D0 = D;
        D = -D0;
        A = A0 + (N - 1) * D0;
    }
    let B = A + (N - 1) * D;
    let ans = if X < A {
        A - X
    } else if B < X {
        X - B
    } else {
        let k = (X - A).div_euclid(D);
        let Al = A + k * D;
        let Ar = A + (k + 1) * D;
        (X - Al).min(Ar - X)
    };

    println!("{}", ans);
}
