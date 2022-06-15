use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        Q: usize,
        tk:[(i64,Usize1);Q],
    }

    let T = S
        .iter()
        .map(|&x| match x {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let U = vec!['A', 'B', 'C'];

    let mut ans = String::new();
    for &(t, k) in tk.iter() {
        let mut h = k;
        let mut ones = 0;
        let mut e = 0;
        while h > 0 && e < t {
            if h & 1 == 1 {
                ones += 1;
            }
            h >>= 1;
            e += 1;
        }
        let zeros = t - ones;
        let delta = zeros - ones;
        let u = (T[h as usize] + delta) as i64;
        let v = u.rem_euclid(3);
        writeln!(ans, "{}", U[v as usize]);
    }
    print!("{}", ans);
}
