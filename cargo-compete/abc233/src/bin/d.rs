use proconio::input;
use superslice::Ext;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: i64,
        A: [i64;N],
    }

    let mut B = vec![(0, -1); N + 1];
    for i in 0..N {
        B[i + 1] = (B[i].0 + A[i], i as i32 + 1);
    }
    B.sort();
    let mut ans = 0;
    for &(b, i) in B.iter() {
        let d = b - K;
        let count = B.upper_bound(&(d, i)) - B.lower_bound(&(d, -1));
        ans += count;
    }

    if K == 0 {
        ans -= N + 1;
    }

    println!("{}", ans);
}
