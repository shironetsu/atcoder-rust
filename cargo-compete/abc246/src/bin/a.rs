use itertools::Itertools;
use proconio::input;
use std::collections::BTreeSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        xy: [(i32, i32);3],
    }

    let x = xy.iter().map(|&(x, _)| x).collect::<BTreeSet<_>>();
    let y = xy.iter().map(|&(_, y)| y).collect::<BTreeSet<_>>();
    let xy = xy.into_iter().collect();
    let v = x
        .into_iter()
        .cartesian_product(y.iter().cloned())
        .collect::<BTreeSet<_>>();
    let (x, y) = v.difference(&xy).next().unwrap();
    println!("{} {}", x, y);
}
