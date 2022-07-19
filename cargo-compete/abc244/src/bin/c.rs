#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
//use std::fmt::Write;
use std::io::{self, Write};
use superslice::{Ext, Ext2};


// macro_rules! print_flush {
//     ($format: expr, $x: expr) => {
//         print!($format, $x);
//     };
// }

fn main() {
    input! {
        N: i32,
    }
    let mut set = BTreeSet::<i32>::new();
    for i in 1..=2 * N + 1 {
        set.insert(i);
    }
    loop {
        let &out = set.iter().next().unwrap();
        println!("{}", out);
        io::stdout().flush().unwrap();
        set.remove(&out);
        input! {
            aoki: i32,
        }
        if aoki == 0 {
            return;
        }
        set.remove(&aoki);
    }
}
