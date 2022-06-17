use proconio::fastout;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::fmt::Write;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        a: [u32;N],
    }

    let mut v = VecDeque::<(u32, u32)>::new();
    let mut balls = 0u32;
    for &a in a.iter() {
        if let Some((k, m)) = v.back_mut() {
            if *k == a {
                *m += 1;
            } else {
                v.push_back((a, 1))
            }
        } else {
            v.push_back((a, 1));
        }
        balls += 1;
        loop {
            if let Some(&(k, m)) = v.back() {
                if k == m {
                    v.pop_back();
                    balls -= k;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        println!("{}", balls);
    }
}
