#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Usize1;
use std::fmt::Write;

fn find(
    root: usize,
    (a, b): (usize, usize),
    (c, d): (usize, usize),
    P: &Vec<usize>,
    Iinv: &Vec<usize>,
    lr: &mut Vec<(Option<usize>, Option<usize>)>,
) -> Result<(), ()> {
    //[a    ...       b)
    //root pre(l) pre(r)
    //
    //[c    ...     d)
    //in(l) root in(r)

    let e = Iinv[root];
    if c <= e && e < d {
        let (lsize, rsize) = (e - c, d - e - 1);
        let l = if lsize > 0 { Some(P[a + 1]) } else { None };
        let r = if rsize > 0 {
            Some(P[a + 1 + lsize])
        } else {
            None
        };
        lr[root] = (l, r);
        let lresult = if let Some(l) = l {
            find(l, (a + 1, a + 1 + lsize), (c, c + lsize), P, Iinv, lr)
        } else {
            Ok(())
        };
        let rresult = if let Some(r) = r {
            find(r, (a + 1 + lsize, b), (c + lsize + 1, d), P, Iinv, lr)
        } else {
            Ok(())
        };
        lresult.and(rresult)
    } else {
        Err(())
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [Usize1; N],
        I: [Usize1; N],
    }

    let mut Iinv = vec![0; N];
    for i in 0..N {
        Iinv[I[i]] = i;
    }

    let mut lr = vec![(None, None); N];

    let result = find(0, (0, N), (0, N), &P, &Iinv, &mut lr);
    let mut ans = String::new();
    match result {
        Ok(()) => {
            for &(l, r) in lr.iter() {
                let l = match l {
                    Some(l) => l + 1,
                    None => 0,
                };
                let r = match r {
                    Some(r) => r + 1,
                    None => 0,
                };
                writeln!(ans, "{} {}", l, r);
            }
        }
        Err(()) => {
            ans = String::from("-1");
        }
    }
    print!("{}", ans);
}
