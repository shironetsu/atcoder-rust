use proconio::input;
use proconio::marker::{Chars, Usize1};

#[allow(non_snake_case)]
fn main() {
    input! {
        L: Usize1,
        R: Usize1,
        mut S: Chars,
    }

    S[L..R + 1].reverse();
    let ans = S.into_iter().collect::<String>();
    println!("{}", ans);
}
