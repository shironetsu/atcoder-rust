use proconio::input;
use std::collections::BTreeMap;

const CHOCO: i32 = 0;
const BOX: i32 = 1;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [i64;N],
        B: [i64;N],
        C: [i64;M],
        D: [i64;M],
    }

    //box first
    let mut arr = vec![];
    for i in 0..N {
        arr.push((A[i], B[i], CHOCO));
    }
    for i in 0..M {
        arr.push((C[i], D[i], BOX));
    }
    arr.sort_by_key(|&(x, y, t)| (-x, -y, -t));
    let mut ys = BTreeMap::<i64, u64>::new();
    for &(_, y, t) in arr.iter() {
        if t == BOX {
            *ys.entry(y).or_default() += 1;
        } else {
            if let Some((&ymin, &c)) = ys.range(y..).next() {
                if c == 1 {
                    ys.remove(&ymin);
                } else {
                    *ys.get_mut(&ymin).unwrap() -= 1;
                }
            } else {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
