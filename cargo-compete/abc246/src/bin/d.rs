use num_integer;
use proconio::input;

fn f(a: u64, b: u64) -> u64 {
    a * a * a + a * a * b + a * b * b + b * b * b
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: u64,
    }

    if N == 0 {
        println!("{}", 0);
        return;
    }

    let mut ans = std::u64::MAX;
    let c = num_integer::cbrt(N);
    for a in 0..=c {
        let mut l = 0;
        let mut r = c + 2;
        while r - l > 1 {
            let m = (l + r) / 2;
            let n = f(a, m);
            if n < N {
                l = m;
            } else {
                r = m;
            }
        }
        let n = f(a, r);
        if ans > n {
            ans = n;
        }
    }
    println!("{}", ans);
}
