#[allow(unused_imports)]
use proconio::input;
use std::collections::*;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: u64,
    }
    let mut L = vec![0; N];
    let mut a = vec![vec![]; N];
    for i in 0..N {
        input! {l:usize}
        L[i] = l;
        for _ in 0..L[i] {
            input! { x: u64 }
            a[i].push(x);
        }
    }

    let mut m = (0..=N)
        .map(|_| BTreeMap::<u64, usize>::new())
        .collect::<Vec<_>>();
    m[0].insert(X, 1);
    for i in 0..N {
        let mc = m[i].clone();
        for (&x, &c) in mc.iter() {
            for j in 0..L[i] {
                if x % a[i][j] == 0 {
                    *m[i + 1].entry(x / a[i][j]).or_default() += c;
                }
            }
        }
    }
    if let Some(ans) = m[N].get(&1) {
        println!("{}", ans);
    } else {
        println!("{}", 0);
    }
}
