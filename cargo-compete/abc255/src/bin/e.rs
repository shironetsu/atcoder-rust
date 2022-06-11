use proconio::input;
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
    let mut E = T.iter().step_by(2).copied().collect::<Vec<_>>();
    let mut O = T
        .iter()
        .skip(1)
        .step_by(2)
        .copied()
        .collect::<Vec<_>>()
        .clone();
    E.sort();
    O.sort();
    let mut ans = 0;
    for i in 0..N {
        for j in 0..M {
            let A0 = X[j] + if i % 2 == 0 { -T[i] } else { T[i] };
            let mut count = 0;
            for k in 0..M {
                let diff = X[k] - A0;
                let e = E.upper_bound(&diff) - E.lower_bound(&diff);
                let diff = X[k] + A0;
                let o = O.upper_bound(&diff) - O.lower_bound(&diff);
                count += e + o;
            }
            if ans < count {
                ans = count;
            }
        }
    }
    println!("{}", ans);
}
