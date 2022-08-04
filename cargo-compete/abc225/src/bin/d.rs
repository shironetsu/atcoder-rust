#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut ans = String::new();
    let mut links = vec![(None, None); N];
    for _ in 0..Q {
        input! {
            t: i32,
        }
        match t {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                links[x].1 = Some(y);
                links[y].0 = Some(x);
            }
            2 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                links[x].1 = None;
                links[y].0 = None;
            }
            3 => {
                input! {
                    x: Usize1,
                }
                let mut trains = VecDeque::new();
                trains.push_back(x);
                let mut a = links[x].0;
                loop {
                    if let Some(x) = a {
                        trains.push_front(x);
                        a = links[x].0;
                    } else {
                        break;
                    }
                }
                let mut b = links[x].1;
                loop {
                    if let Some(x) = b {
                        trains.push_back(x);
                        b = links[x].1;
                    } else {
                        break;
                    }
                }
                let line = trains.iter().map(|&x| (x + 1).to_string()).join(" ");
                writeln!(&mut ans, "{} {}", trains.len(), line);
            }
            _ => (),
        }
    }

    print!("{}", ans);
}
