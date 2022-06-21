use proconio::input;
use proconio::marker::{Chars, Usize1};

#[allow(non_snake_case)]
fn main() {
    input! {
        L: Usize1,
        R: Usize1,
        S: Chars,
    }

    let mut T = String::new();
    for i in 0..L {
        T.push(S[i]);
    }
    for i in L..=R {
        let j = L + R - i;
        T.push(S[j]);
    }
    for i in R + 1..S.len() {
        T.push(S[i]);
    }
    let ans = T.to_string();
    println!("{}", ans);
}
