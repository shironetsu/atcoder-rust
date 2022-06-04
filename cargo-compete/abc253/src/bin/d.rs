use num_integer::lcm;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: u64,
        A: u64,
        B: u64,
    }

    let a = N * (N + 1) / 2;
    let b = A * (N / A) * (N / A + 1) / 2;
    let c = B * (N / B) * (N / B + 1) / 2;
    let g = lcm(A, B);
    let d = g * (N / g) * (N / g + 1) / 2;
    let ans = a - b - c + d;
    println!("{}", ans);
}
