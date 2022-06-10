use itertools::Itertools;
use num_integer;
use proconio::input;
use std::collections::BTreeMap;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: u64,
        P: [(i64, i64);N],
    }

    if K == 1 {
        println!("Infinity");
        return;
    }

    let mut conj = BTreeMap::<(i64, i64, i64), u64>::new();
    for (i, j) in (0..N).tuple_combinations() {
        let (x0, y0) = P[i];
        let (x1, y1) = P[j];
        let mut a = y0 - y1;
        let mut b = -x0 + x1;
        let mut c = x0 * y1 - x1 * y0;
        let p = if a == 0 {
            if b < 0 {
                b *= -1;
                c *= -1;
            }
            let g = num_integer::gcd(b, c.abs());
            (0, b / g, c / g)
        } else {
            if a < 0 {
                a *= -1;
                b *= -1;
                c *= -1;
            }
            let g = num_integer::gcd(a, b.abs());
            (a / g, b / g, c / g)
        };
        *conj.entry(p).or_insert(1) += 1;
    }

    let ans = conj.iter().filter(|(_, &c)| c >= K * (K - 1) / 2).count();

    println!("{}", ans);
}
