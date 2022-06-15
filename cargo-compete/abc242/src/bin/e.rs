use num_integer;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use std::fmt::Write;
use superslice::Ext;

const MODULO: i64 = 998244353;

fn powmod(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    } else if b == 1 {
        return a.rem_euclid(MODULO);
    }

    let u = powmod(a, b >> 1);
    let uu = (u * u).rem_euclid(MODULO);
    if b & 1 == 1 {
        (a * uu).rem_euclid(MODULO)
    } else {
        uu
    }
}

fn is_palindrome(s: String) -> bool {
    s.chars()
        .zip(s.chars().rev())
        .fold(true, |acc, (c0, c1)| acc && c0 == c1)
}

#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }

    let abc = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<_>>();

    let mut ans = String::new();

    for _ in 0..T {
        input! {
            N: i64,
            S: Chars,
        }

        let mut count = 0;
        for (i, &c) in S.iter().enumerate() {
            let r = num_integer::div_ceil(N - 2 * (i as i64 + 1), 2);
            if r < 0 {
                break;
            }
            let u = abc.lower_bound(&c) as i64;
            let v = u * powmod(26, r);
            count += v % MODULO;
            count %= MODULO;
        }
        let p = S
            .iter()
            .enumerate()
            .zip(S.iter().rev())
            .map(
                |((i, &a), &b)| {
                    if i <= (N as usize - 1) / 2 {
                        a
                    } else {
                        b
                    }
                },
            )
            .collect::<String>();
        if p <= S.iter().collect() {
            count += 1; //hack_01.txt
        }
        writeln!(ans, "{}", count % MODULO);
    }

    print!("{}", ans);
}
