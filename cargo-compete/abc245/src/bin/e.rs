use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::collections::BTreeMap;
#[allow(unused_imports)]
use std::fmt::Write;
//use std::ops::Bound::{Excluded, Included, Unbounded};

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

    let mut arr = vec![];
    for i in 0..N {
        arr.push((A[i], B[i], 0));
    }
    for i in 0..M {
        arr.push((C[i], D[i], 1));
    }
    //box first
    arr.sort_by_key(|(x, y, t)| (-x, -y, -t));

    let mut widths = BTreeMap::<i64, u64>::new();
    for &(x, y, t) in arr.iter() {
        //println!("{} {} {}", x, y, t);
        if t == 1 {
            *widths.entry(y).or_default() += 1;
        } else {
            if let Some((&ymin, &c)) = widths.range(y..).next(){
                //println!("- {} {}", ymin, c);
                if c == 1 {
                    widths.remove(&ymin);
                } else {
                    *widths.get_mut(&ymin).unwrap() -= 1;
                }
            } else {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
