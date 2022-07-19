#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input, source::line::LineSource};
use std::collections::*;
//use std::fmt::Write;
use std::io::{stdin, stdout, BufReader, Write};
use superslice::{Ext, Ext2};

fn main() {
    let stdin =stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        N: i32,
    }
    let mut set = BTreeSet::<i32>::new();
    for i in 1..=2 * N + 1 {
        set.insert(i);
    }
    loop {
        let &out = set.iter().next().unwrap();
        println!("{}", out);
        stdout().flush().unwrap();
        set.remove(&out);
        input! {
            from &mut source,
            aoki: i32,
        }
        if aoki == 0 {
            return;
        }
        set.remove(&aoki);
    }
}
