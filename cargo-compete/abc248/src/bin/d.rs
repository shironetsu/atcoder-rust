use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeMap;
use std::fmt::Write;
use superslice::Ext;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [u32;N],
        Q: usize,
        q: [(Usize1, Usize1, u32);Q],
    }

    let mut m = BTreeMap::<u32, Vec<usize>>::new();
    for (i, &a) in A.iter().enumerate() {
        if let Some(e) = m.get_mut(&a) {
            e.push(i);
        } else {
            m.insert(a, vec![i]);
        }
    }
    let mut ans = String::new();
    for (L, R, X) in q {
        if let Some(e) = m.get(&X) {
            let c = e.lower_bound(&(R + 1)) - e.lower_bound(&L);
            writeln!(ans, "{}", c);
        } else {
            writeln!(ans, "{}", 0);
        }
    }
    print!("{}", ans);
}
