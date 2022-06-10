use itertools::Itertools;
use num_integer;
use proconio::input;
use std::collections::BTreeMap;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
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
        if a < 0 {
            a *= -1;
            b *= -1;
            c *= -1;
        }
        if a == 0 {
            if b < 0 {
                b *= -1;
                c *= -1;
            }
            let g = num_integer::gcd(b, c.abs());
            let p = (0, b / g, c / g);
            if let Some(e) = conj.get_mut(&p) {
                *e += 1;
            } else {
                conj.insert(p, 1);
            }
        } else {
            let g = num_integer::gcd(a, b.abs());
            let p = (a / g, b / g, c / g);
            if let Some(e) = conj.get_mut(&p) {
                *e += 1;
            } else {
                conj.insert(p, 1);
            }
        }
    }

    let mut ans = 0;
    for (_, &c) in conj.iter() {
        let n = (1 + num_integer::sqrt(1 + 8 * c)) / 2;
        if n >= K as u64 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
