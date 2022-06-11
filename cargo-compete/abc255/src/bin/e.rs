use proconio::input;
use std::fmt::Write;
use superslice::Ext;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: [i64;N-1],
        X: [i64;M],
    }

    let mut T = vec![0; N];
    for i in 0..(N - 1) {
        T[i + 1] = S[i] - T[i];
    }
    let mut E = T.iter().step_by(2).collect::<Vec<_>>();
    let mut O = T.iter().skip(1).step_by(2).collect::<Vec<_>>();
    // println!("{:?}", E);
    // println!("{:?}", O);
    E.sort();
    O.sort();
    let mut ans = 0;
    for s in 0..1 << M {
        for t in 0..1 << M {
            for i in 0..M {
                if ((s >> i) & 1 == 0) {
                    //ans += E.upper_bound(&)
                }
            }
        }
    }
}
