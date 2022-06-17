use bitset_fixed::BitSet;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    // input! {
    //     N: usize,
    //     X: usize,
    //     ab: [(usize, usize);N],
    // }

    // let mut p = vec![vec![false; N + 1]; X as usize + 1];
    // p[0][0] = true;
    // for (i, &(a, b)) in ab.iter().enumerate() {
    //     for x in 0..X {
    //         if x + a <= X {
    //             p[x + a][i + 1] |= p[x][i];
    //         }
    //         if x + b <= X {
    //             p[x + b][i + 1] |= p[x][i];
    //         }
    //     }
    // }
    // if p[X][N] {
    //     println!("Yes");
    // }else{
    //     println!("No");
    // }

    input! {
        N: usize,
        X: usize,
    }

    let mut bs = BitSet::new(X + 1);
    bs.set(0, true);
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }

        //bs = (&bs << a) | &(bs << b);
        bs.shl_or(b - a);
        bs <<= a;
    }
    if bs[X] {
        println!("Yes");
    } else {
        println!("No");
    }
}
