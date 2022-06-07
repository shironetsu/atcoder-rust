use num_integer;
use proconio::input;
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: u64,
    }

    let mut ans = String::new();

    for i in 0..N {
        for j in 0..=i {
            write!(
                &mut ans,
                "{}{}",
                num_integer::binomial(i, j),
                if i == j { "\n" } else { " " }
            );
        }
    }
    print!("{}", ans);
}
