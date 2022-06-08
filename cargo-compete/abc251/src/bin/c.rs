use proconio::input;
use std::collections::BTreeMap;
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        submits: [(String, i32);N],
    }

    let mut m = BTreeMap::new();
    for (i, (s, t)) in submits.iter().enumerate() {
        if !m.contains_key(&s) {
            m.insert(s, (-t, i));
        }
    }
    let mut v = m.iter().map(|(_, x)| x).collect::<Vec<_>>();
    v.sort();
    let (_, ans) = v[0];
    println!("{}", ans + 1);
}
