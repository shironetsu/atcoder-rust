use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [Usize1; K],
        XY: [(f64, f64); N],
    }

    let mut B = BTreeSet::new();
    for i in 0..N {
        B.insert(i);
    }
    for a in A.iter() {
        B.remove(&a);
    }

    let mut ans = 0f64;
    for &b in B.iter() {
        let (x0, y0) = XY[b];
        let mut dmin = 10000000.0;
        for &a in A.iter() {
            let (x1, y1) = XY[a];
            let dx = x1 - x0;
            let dy = y1 - y0;
            let rr = dx * dx + dy * dy;
            let r = rr.sqrt();
            if dmin > r {
                dmin = r;
            }
        }
        if ans < dmin {
            ans = dmin;
        }
    }

    println!("{}", ans);
}
