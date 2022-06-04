use proconio::input;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,

    }

    let mut S = BTreeSet::new();
    let mut M = BTreeMap::<u32, u32>::new();

    let mut ans = String::new();

    for q in 0..Q {
        input! {
            t: u32,
        }
        match t {
            1 => {
                input! {
                    x: u32,
                }
                match M.get_mut(&x) {
                    Some(c) => {
                        *c += 1;
                    }
                    None => {
                        M.insert(x, 1);
                        S.insert(x);
                    }
                }
            }
            2 => {
                input! {
                    x: u32,
                    c: u32,
                }
                match M.get_mut(&x) {
                    Some(d) => {
                        *d -= c.min(*d);
                        if *d == 0 {
                            M.remove(&x);
                            S.remove(&x);
                        }
                    }
                    None => (),
                }
            }
            3 => {
                let max = S.iter().last().unwrap();
                let min = S.iter().next().unwrap();
                writeln!(&mut ans, "{}", max - min);
            }
            _ => (),
        }
    }

    print!("{}", ans);
}
